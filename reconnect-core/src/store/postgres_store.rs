use crate::config::TableConfig;
use crate::differ::DiffResult;
use crate::store::SQL_TEMPLATES;
use std::collections::HashMap;

pub struct PostgresStore {
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String,
    params: Option<HashMap<String, String>>,
}

impl PostgresStore {
    pub fn new(
        host: String,
        port: u16,
        username: String,
        password: String,
        database: String,
        params: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            host,
            port,
            username,
            password,
            database,
            params,
        }
    }

    pub fn diff_datasets(&self, left: &TableConfig, right: &TableConfig) -> anyhow::Result<DiffResult> {
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

        //println!("Query : {}", query);

        Ok(DiffResult {
            headers: vec![],
            rows: vec![],
        })
    }
}
