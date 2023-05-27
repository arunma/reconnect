use anyhow::anyhow;
use chrono::prelude::*;
use log::error;
use rand::prelude::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};

use std::fs::{File, OpenOptions};
use std::io::BufWriter;
use std::path::Path;

use crate::data_generator::constants::{RANDOM_CITY_COUNTRIES, RANDOM_NAMES};
use crate::data_generator::{constants, postgres_generator, DataGenerator};
use postgres::Client;
use std::{env, io};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Customer {
    id: usize,
    uuid: String,
    age: u8,
    first_name: String,
    last_name: String,
    city: String,
    country: String,
    /*#[serde(with = "chrono::serde::ts_seconds")]
    updated_at: DateTime<Utc>,*/
    updated_at: String,
}
pub struct PostgresGenerator {}

impl PostgresGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn drop_table(&self, table_name: &str) -> anyhow::Result<()> {
        let db_host = env::var("RECON_POSTGRES_HOST").expect("RECON_POSTGRES_HOST not set");
        let db_port = env::var("RECON_POSTGRES_PORT").expect("RECON_POSTGRES_PORT not set");
        let db_name = env::var("RECON_POSTGRES_DBNAME").expect("RECON_POSTGRES_DBNAME not set");
        let db_user = env::var("RECON_POSTGRES_USER").expect("RECON_POSTGRES_USER not set");
        let db_password = env::var("RECON_POSTGRES_PASSWORD").expect("RECON_POSTGRES_PASSWORD not set");

        let conn_string =
            format!("host={db_host} port={db_port} dbname={db_name} user={db_user} password={db_password}");

        //Connection
        let mut client = Client::connect(&conn_string, postgres::NoTls).map_err(|e| anyhow!(e))?;

        let drop_sql = format!("DROP TABLE IF EXISTS {table_name}");
        client.execute(&drop_sql, &[]).map_err(|e| anyhow!(e))?;
        Ok(())
    }

    fn create_table(&self, client: &mut Client, table_name: &str) -> anyhow::Result<()> {
        let create_table_sql = format!(
            "CREATE TABLE IF NOT EXISTS {table_name} (
            ID INT PRIMARY KEY,
            UUID VARCHAR(36) NOT NULL,
            AGE SMALLINT NOT NULL,
            FIRST_NAME VARCHAR(255) NOT NULL,
            LAST_NAME VARCHAR(255) NOT NULL,
            CITY VARCHAR(255) NOT NULL,
            COUNTRY VARCHAR(255) NOT NULL,
            UPDATED_AT TIMESTAMP
        )"
        );
        client.execute(&create_table_sql, &[]).map_err(|e| {
            println!("Error: {:?}", e);
            anyhow!(e)
        })?;
        Ok(())
    }

    fn load_csv_file(&self, client: &mut Client, table_name: &str, file_path: &Path) -> anyhow::Result<()> {
        println!("File path: {:?}", file_path.canonicalize().unwrap());
        let copy_sql = format!(
            r#"
                COPY {table_name} 
                FROM '/var/lib/postgresql/bench_data/{table_name}.csv' 
                WITH (FORMAT CSV, HEADER true, DELIMITER ',')
             "#,
        );
        println!("Copy SQL: {}", copy_sql);

        let set_sql = "SET datestyle = 'ISO, DMY';";

        client.simple_query(set_sql).map_err(|e| {
            println!("Error: {:?}", e);
            anyhow!(e)
        })?;
        client.copy_in(&copy_sql).map_err(|e| {
            println!("Error: {:?}", e);
            anyhow!(e)
        })?;
        println!("Copy Complete for : {table_name}");
        Ok(())
    }
}

impl DataGenerator for PostgresGenerator {
    fn generate_data_as_csv(&self, num_rows: usize, file_path: &Path) -> io::Result<()> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_path)?;
        let writer = BufWriter::new(file);
        let mut csv_writer = csv::Writer::from_writer(writer);
        let total_name_count = constants::RANDOM_NAMES.len();
        let total_city_count = constants::RANDOM_CITY_COUNTRIES.len();

        for i in 0..num_rows {
            let utc = Utc::now();
            let utc = utc.format("%Y-%m-%d %H:%M:%S").to_string();
            let name = RANDOM_NAMES[rand::random::<usize>() % total_name_count];
            let city_country = RANDOM_CITY_COUNTRIES[rand::random::<usize>() % total_city_count];
            csv_writer.serialize(Customer {
                id: i + 1,
                uuid: Uuid::new_v4().to_string(),
                age: rand::random::<u8>(),
                first_name: name.0.into(),
                last_name: name.1.into(),
                city: city_country.1.into(),
                country: city_country.0.into(),
                //updated_at: local.into(),
                updated_at: utc,
            })?;
        }

        csv_writer.flush()?;

        Ok(())
    }

    fn persist_data_to_database(&self, file_path: &Path, table_name: &str) -> anyhow::Result<()> {
        let db_host = env::var("RECON_POSTGRES_HOST").expect("RECON_POSTGRES_HOST not set");
        let db_port = env::var("RECON_POSTGRES_PORT").expect("RECON_POSTGRES_PORT not set");
        let db_name = env::var("RECON_POSTGRES_DBNAME").expect("RECON_POSTGRES_DBNAME not set");
        let db_user = env::var("RECON_POSTGRES_USER").expect("RECON_POSTGRES_USER not set");
        let db_password = env::var("RECON_POSTGRES_PASSWORD").expect("RECON_POSTGRES_PASSWORD not set");

        let conn_string =
            format!("host={db_host} port={db_port} dbname={db_name} user={db_user} password={db_password}");

        //Connection
        let mut client = Client::connect(&conn_string, postgres::NoTls).map_err(|e| anyhow!(e))?;

        //Create tables
        self.create_table(&mut client, table_name)?;

        //Load file into table
        if let Err(e) = self.load_csv_file(&mut client, table_name, file_path) {
            //For some reason, the server throws an UnexpectedMessage error but works just fine.
            //The copy command takes 650ms for 10k records as compared to 15+ seconds for the insert command. Hence sticking to this.
            error!("Error: {:?}", e);
        }

        //INSERT
        //self.insert_into_table(file_path, &table_name, &mut client)?;
        Ok(())
    }

    fn introduce_differences_in_csv(
        &self,
        source_path: &Path,
        target_path: &Path,
        num_rows: usize,
        max_diff_pct: f64,
    ) -> anyhow::Result<()> {
        let source_file = File::open(source_path).unwrap();
        let mut reader = csv::Reader::from_reader(source_file);
        let mut writer = csv::Writer::from_path(target_path).unwrap();
        let mut target_changes = (num_rows as f64 * max_diff_pct) as usize;
        let mut column_indices = vec!["age", "first_name", "last_name", "city", "country"];
        let mut rng = rand::thread_rng();
        let mut header_row = true;
        for record in reader.deserialize() {
            if header_row {
                header_row = false;
                continue;
            }
            let mut customer: Customer = record?;
            let coin_toss = rng.gen_range(0..=1);
            //Introduce differences up to a maximum of target_rows number of changes
            if coin_toss == 0 && target_changes > 0 {
                column_indices.shuffle(&mut rng);
                target_changes -= 1;
                match column_indices[0] {
                    "age" => {
                        let new_age = rng.gen_range(0..=100);
                        customer.age = new_age;
                    }
                    "first_name" => {
                        let new_name = RANDOM_NAMES[rand::random::<usize>() % RANDOM_NAMES.len()];
                        customer.first_name = new_name.0.into();
                    }
                    "last_name" => {
                        let new_name = RANDOM_NAMES[rand::random::<usize>() % RANDOM_NAMES.len()];
                        customer.last_name = new_name.1.into();
                    }
                    "city" => {
                        let new_city_country =
                            RANDOM_CITY_COUNTRIES[rand::random::<usize>() % RANDOM_CITY_COUNTRIES.len()];
                        customer.city = new_city_country.1.into();
                    }
                    "country" => {
                        let new_city_country =
                            RANDOM_CITY_COUNTRIES[rand::random::<usize>() % RANDOM_CITY_COUNTRIES.len()];
                        customer.country = new_city_country.0.into();
                    }
                    _ => {
                        panic!("Invalid column name")
                    }
                }
            }
            writer.serialize(customer)?;
        }

        writer.flush().unwrap();
        Ok(())
    }

    /*fn insert_into_table(
        &self,
        file_path: &Path,
        table_name: &&str,
        client: &mut Client,
    ) -> anyhow::Result<()> {
        let query = client.prepare(&format!(
            r#"
                INSERT INTO {}
                (id, uuid, age, first_name, last_name, city, country, updated_at)
                VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            table_name
        ))?;

        let file = File::open(file_path)?;
        let mut reader = csv::Reader::from_reader(file);

        for result in reader.deserialize() {
            let record: Customer = result?;

            client.execute(
                &query,
                &[
                    &(record.id as i32),
                    &record.uuid,
                    &(record.age as i16),
                    &record.first_name,
                    &record.last_name,
                    &record.city,
                    &record.country,
                    &record.updated_at.naive_utc(),
                ],
            )?;
        }
        Ok(())
    }*/
}
