use crate::config::TableConfig;
use crate::differ::DiffResult;
use crate::store::{RowResult, Segment, SQL_TEMPLATES};
use anyhow::anyhow;
use anyhow::Result as AResult;
use chrono::Utc;
use log::info;
use rust_decimal::Decimal;

use bb8::{Pool, PooledConnection, RunError};
use bb8_postgres::PostgresConnectionManager;
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use tera::Context;
use tokio_postgres::{Config, Error, NoTls, Row};

//FIXME - Most methods must be pub(crate)
static VALUE_NOT_FOUND: Lazy<String> = Lazy::new(|| String::from("VALUE_NOT_FOUND"));

#[derive(Debug, Clone)]
pub struct PostgresStore {
    host: String,
    port: u16,
    username: String,
    password: String,
    dbname: String,
    params: Option<HashMap<String, String>>,
    pool: Pool<PostgresConnectionManager<NoTls>>,
}

pub struct PostgresStoreBuilder {
    host: String,
    port: u16,
    username: String,
    password: String,
    dbname: String,
    params: Option<HashMap<String, String>>,
}

impl PostgresStoreBuilder {
    pub fn new(
        host: String,
        port: u16,
        username: String,
        password: String,
        dbname: String,
        params: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            host,
            port,
            username,
            password,
            dbname,
            params,
        }
    }

    pub async fn build(&self) -> anyhow::Result<PostgresStore> {
        let conn_string = format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.dbname
        );

        let config = Config::from_str(&conn_string)?;
        let pg_mgr = PostgresConnectionManager::new(config, tokio_postgres::NoTls);
        let pool = Pool::builder().max_size(10).build(pg_mgr).await?;

        Ok(PostgresStore {
            host: self.host.clone(),
            port: self.port,
            username: self.username.clone(),
            password: self.password.clone(),
            dbname: self.dbname.clone(),
            params: self.params.clone(),
            pool,
        })
    }
}

impl PostgresStore {
    pub fn builder(
        host: String,
        port: u16,
        username: String,
        password: String,
        dbname: String,
        params: Option<HashMap<String, String>>,
    ) -> PostgresStoreBuilder {
        PostgresStoreBuilder::new(host, port, username, password, dbname, params)
    }

    pub async fn pool(&self) -> Pool<PostgresConnectionManager<NoTls>> {
        self.pool.clone()
    }

    pub async fn connection(&self) -> AResult<PooledConnection<'_, PostgresConnectionManager<NoTls>>> {
        let conn = self.pool.get().await?;
        Ok(conn)
    }

    pub async fn diff_datasets(&mut self, left: &TableConfig, right: &TableConfig) -> anyhow::Result<DiffResult> {
        let mut context = tera::Context::new();
        context.insert("left_table", &left.table);
        context.insert("left_alias", &left.alias.to_uppercase());
        context.insert(
            "left_key",
            &left.key.iter().map(|s| s.to_uppercase()).collect::<Vec<String>>(),
        );
        context.insert(
            "left_satellite_fields",
            &left
                .satellite_fields
                .iter()
                .map(|s| s.to_uppercase())
                .collect::<Vec<String>>(),
        );
        context.insert(
            "left_compare_fields",
            &left
                .compare_fields
                .iter()
                .map(|s| s.to_uppercase())
                .collect::<Vec<String>>(),
        );
        context.insert("left_filter_conditions", &left.filter_conditions);
        context.insert("right_table", &right.table);
        context.insert("right_alias", &right.alias.to_uppercase());
        context.insert(
            "right_key",
            &right.key.iter().map(|s| s.to_uppercase()).collect::<Vec<String>>(),
        );
        context.insert(
            "right_satellite_fields",
            &right
                .satellite_fields
                .iter()
                .map(|s| s.to_uppercase())
                .collect::<Vec<String>>(),
        );
        context.insert(
            "right_compare_fields",
            &right
                .compare_fields
                .iter()
                .map(|s| s.to_uppercase())
                .collect::<Vec<String>>(),
        );
        context.insert("right_filter_conditions", &right.filter_conditions);
        //println!("Context is ready. Attempting to render SQL");
        let query = SQL_TEMPLATES.render("single_store_diff_postgres.sql", &context)?;
        //println!("Query is ready. Attempting to execute \n {}", query);
        let rows = self.connection().await?.query(query.as_str(), &[]).await?;

        let headers = rows[0]
            .columns()
            .iter()
            .map(|c| c.name().to_string())
            .collect::<Vec<String>>();
        let mut values = vec![];
        for row in rows {
            let row_map = self.row_to_map(&row);
            values.push(row_map.values().cloned().collect::<Vec<String>>());
        }

        Ok(DiffResult { headers, rows: values })
    }

    fn row_to_map(&self, row: &Row) -> HashMap<String, String> {
        let mut map = HashMap::new();

        for (index, col) in row.columns().iter().enumerate() {
            let column_type = col.type_().name();
            let value = match column_type {
                "bool" => {
                    let v: Option<bool> = row.get(index);
                    v.map(|v| v.to_string())
                }
                "varchar" | "char(n)" | "text" | "name" => {
                    let v: Option<String> = row.get(index);
                    v
                }
                "int2" | "smallserial" | "smallint" => {
                    let v: Option<i16> = row.get(index);
                    v.map(|v| v.to_string())
                }
                "int" | "int4" | "serial" => {
                    let v: Option<i32> = row.get(index);
                    v.map(|v| v.to_string())
                }
                "int8" | "bigserial" | "bigint" => {
                    let v: Option<i64> = row.get(index);
                    v.map(|v| v.to_string())
                }
                "float4" | "real" => {
                    let v: Option<f32> = row.get(index);
                    v.map(|v| v.to_string())
                }
                "float8" | "double precision" => {
                    let v: Option<f64> = row.get(index);
                    v.map(|v| v.to_string())
                }
                "timestamp" | "timestamptz" => {
                    // with-chrono feature is needed for this
                    let v: Option<chrono::DateTime<Utc>> = row.get(index);
                    v.map(|v| v.to_string())
                }
                "numeric" => {
                    let v: Option<Decimal> = row.get(index);
                    v.map(|v| v.to_string())
                }
                p => {
                    println!("Unknown type: {}", p);
                    Some("CANNOT PARSE".to_string())
                }
            };

            map.insert(col.name().to_string(), value.unwrap_or("".into()));
        }
        map
    }

    //TODO - Need to add instrumentation/tracing for this
    pub(crate) async fn get_agg_count_and_checksums(
        &self,
        config: &TableConfig,
        _params: HashMap<String, String>,
    ) -> AResult<Segment> {
        let mut context = Context::new();
        context.insert("table", &config.table);
        context.insert("alias", &config.alias.to_uppercase());
        context.insert("key", &config.key);
        context.insert("compare_fields", &config.compare_fields);
        context.insert("filter_conditions", &config.filter_conditions);

        let query = SQL_TEMPLATES
            .render("get_agg_count_checksum_postgres.sql", &context)
            .unwrap();

        info!("Agg Count and checksum query: {}", query);

        let rows = self.connection().await?.query(query.as_str(), &[]).await?;

        let row = rows
            .get(0)
            .ok_or(anyhow!("No rows returned from agg_count_and_checksums query"))?;

        let row_map = self.row_to_map(row);
        Ok(Segment {
            min: (&row_map["seg_min"]).clone(),
            max: (&row_map["seg_max"]).clone(),
            count: (&row_map["seg_count"]).clone().parse::<usize>().unwrap_or(0),
            checksum: (&row_map["seg_checksum"]).clone(),
        })
    }

    pub(crate) async fn get_keys_and_checksums(
        &self,
        config: &TableConfig,
        seg_min: String,
        seg_max: String,
    ) -> AResult<HashSet<String>> {
        let mut context = Context::new();
        context.insert("table", &config.table);
        context.insert("alias", &config.alias.to_uppercase());
        context.insert("key", &config.key);
        context.insert("compare_fields", &config.compare_fields);
        context.insert("filter_conditions", &config.filter_conditions);
        context.insert("seg_min", &seg_min);
        context.insert("seg_max", &seg_max);

        let query = SQL_TEMPLATES
            .render("get_ids_and_checksums_for_segment_postgres.sql", &context)
            .unwrap();

        info!("Agg Count and checksum query: {}", query);

        let rows = self.connection().await?.query(query.as_str(), &[]).await?;

        let mut kcs: HashSet<String> = HashSet::new();

        for row in rows {
            kcs.insert(row.get(0));
        }

        Ok(kcs)
    }

    pub(crate) async fn get_rows_by_keys(
        &self,
        config: &TableConfig,
        diff_keys: &HashSet<String>,
    ) -> AResult<RowResult> {
        let mut context = Context::new();
        context.insert("table", &config.table);
        context.insert("alias", &config.alias.to_uppercase());
        context.insert("key", &config.key);
        context.insert("compare_fields", &config.compare_fields);
        context.insert("satellite_fields", &config.satellite_fields);
        context.insert("filter_conditions", &config.filter_conditions);
        context.insert("diff_keys", &diff_keys);

        let query = SQL_TEMPLATES.render("get_rows_by_keys_postgres.sql", &context).unwrap();

        info!("Get rows by keys query: {}", query);

        let rows = self.connection().await?.query(query.as_str(), &[]).await?;

        let mut row_results = HashMap::new();
        for row in rows {
            //FIXME - this fetches just the concatenated rows
            let row_map = self.row_to_map(&row);
            let key = row_map.get("KEY").unwrap().clone();
            row_results.insert(key, row_map);
        }

        Ok(row_results)
    }
}
