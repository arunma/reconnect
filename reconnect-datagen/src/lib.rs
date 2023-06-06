use crate::data_generator::postgres_generator::PostgresGenerator;
use crate::data_generator::DataGenerator;
use anyhow::Result as AResult;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

pub mod data_generator;

pub fn prepare_database(root_path: &Path, num_rows: usize) -> AResult<()> {
    dotenv::dotenv().ok();
    //env_logger::try_init()?;

    let start = Instant::now();

    fs::create_dir_all(root_path)?;
    let customer1 = PathBuf::from(root_path).join("customer1.csv");
    let customer2 = PathBuf::from(root_path).join("customer2.csv");

    let datagen = PostgresGenerator::new();

    datagen.drop_table("customer1")?;
    datagen.drop_table("customer2")?;

    datagen.generate_data_as_csv(num_rows, &customer1)?;
    //fs::copy(customer1, customer2).expect("Unable to copy file");
    datagen.introduce_differences_in_csv(&customer1, &customer2, num_rows, 0.1)?;

    datagen.persist_data_to_database(&customer1, "customer1")?;
    datagen.persist_data_to_database(&customer2, "customer2")?;

    let end = Instant::now();
    println!("Time taken: {:?}", end - start);
    Ok(())
}
