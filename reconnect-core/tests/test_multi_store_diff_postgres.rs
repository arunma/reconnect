use crate::helpers::CONF_TEMPLATES;
use anyhow::anyhow;
use dotenv::dotenv;
use reconnect_core::config::{DiffConfig, TableConfig};
use reconnect_core::differ::Differ;
use std::env;
use std::time::Instant;
use tera::{Context, Tera};

pub mod helpers;

#[test]
pub fn test_multi_store_diff() -> anyhow::Result<()> {
    //Populate template with credential variables
    dotenv().ok();
    let mut context = Context::new();
    for (key, value) in env::vars() {
        context.insert(key, &value);
    }

    //Render template
    let query = CONF_TEMPLATES
        .render("customer_diff_multi_store_postgres_full_table.yaml", &context)
        .map_err(|e| {
            eprintln!("Error: {:?}", e);
            anyhow!(e)
        })?;

    let start = Instant::now();
    //Construct config
    let config: DiffConfig = serde_yaml::from_str(&query).map_err(|e| {
        eprintln!("Error: {:?}", e);
        anyhow!(e)
    })?;

    //Assert config values
    let left = &config.left;
    let right = &config.right;
    assert_eq!(left.schema, right.schema);
    assert_eq!(left.table, "customer1");
    assert_eq!(right.table, "customer2");

    //Generate Single table template
    let differ = Differ::new(config);
    let diff_result = differ.diff(std::collections::HashMap::new());
    if diff_result.is_err() {
        eprintln!("Error: {:?}", diff_result);
    }
    let diff_result = diff_result?;
    println!("Diff Result: {:?}", diff_result);

    let end = start.elapsed();
    println!("Time elapsed: {:?}", end);
    //Run SQL and generate diff results

    //Assert diff results

    Ok(())
}
