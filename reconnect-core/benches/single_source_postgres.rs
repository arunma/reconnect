use anyhow::anyhow;
use criterion::{criterion_group, criterion_main, Criterion};
use dotenv::dotenv;
use lazy_static::lazy_static;
use reconnect_core::config::DiffConfig;
use reconnect_core::differ::Differ;
use std::env;
use tera::{Context, Tera};

lazy_static! {
    pub static ref CONF_TEMPLATES: Tera = {
        let mut tera = Tera::new("../examples/conf/*").unwrap();
        tera.autoescape_on(vec![]);
        tera
    };
}

fn single_source_postgres(c: &mut Criterion) {
    dotenv().ok();
    let mut context = Context::new();
    for (key, value) in env::vars() {
        context.insert(key, &value);
    }

    //Render template
    let config_string = CONF_TEMPLATES
        .render("customer_diff_single_store_postgres.yaml", &context)
        .map_err(|e| {
            eprintln!("Error: {:?}", e);
            anyhow!(e)
        })
        .unwrap();

    //Construct config

    c.bench_function("diff", |b| b.iter(|| diff_call(config_string.clone())));
}

fn diff_call(config_string: String) -> anyhow::Result<()> {
    let config: DiffConfig = serde_yaml::from_str(&config_string).map_err(|e| {
        eprintln!("Error: {:?}", e);
        anyhow!(e)
    })?;

    let differ = Differ::new(config);

    let diff_result = differ.diff(std::collections::HashMap::new())?;

    Ok(())
}

criterion_group!(benches, single_source_postgres);
criterion_main!(benches);
