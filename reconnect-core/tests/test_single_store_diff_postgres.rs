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
pub fn test_single_store_diff() -> anyhow::Result<()> {
    //Populate template with credential variables
    dotenv().ok();
    let mut context = Context::new();
    for (key, value) in env::vars() {
        context.insert(key, &value);
    }

    //Render template
    let query = CONF_TEMPLATES
        .render("customer_diff_single_store_postgres.yaml", &context)
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
    assert_eq!(left.connection_uri, right.connection_uri);
    assert_eq!(left.schema, right.schema);
    assert_eq!(left.table, "customer1");
    assert_eq!(right.table, "customer2");
    assert_eq!(left.key, right.key);
    assert_eq!(left.satellite_fields, vec!["country", "city"]);
    assert_eq!(left.compare_fields, vec!["age", "first_name"]);
    assert_eq!(right.compare_fields, vec!["age", "first_name"]);

    //Generate Single table template
    let differ = Differ::new(config);
    let diff_result = differ.diff(std::collections::HashMap::new())?;
    println!("Diff Result: {:?}", diff_result);

    let end = start.elapsed();
    println!("Time elapsed: {:?}", end);
    //Run SQL and generate diff results

    //Assert diff results

    Ok(())
}
