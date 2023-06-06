use crate::benchmarks::{diff_call, DIFF_PCT, NUM_ROWS_TO_TEST};
use anyhow::anyhow;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use dotenv::dotenv;
use once_cell::sync::Lazy;
use reconnect_datagen::prepare_database;
use std::env;
use std::path::Path;
use tera::{Context, Tera};
use tokio::runtime::{Builder, Runtime};

static CONF_TEMPLATES: Lazy<Tera> = Lazy::new(|| {
    let mut tera = Tera::new("../examples/conf/*").unwrap();
    tera.autoescape_on(vec![]);
    tera
});

fn runtime() -> Runtime {
    Runtime::new().unwrap()
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("postgres_full_table");
    dotenv().ok();
    let mut context = Context::new();
    for (key, value) in env::vars() {
        context.insert(key, &value);
    }

    let root_path = Path::new("../bench_data");
    for num_rows in NUM_ROWS_TO_TEST {
        if let Err(e) = prepare_database(root_path, num_rows) {
            eprintln!("Error preparing database: {:?}", e);
            break;
        }
        println!("Prepared database with {} rows", num_rows);

        let single_store_config_string = CONF_TEMPLATES
            .render("customer_diff_single_store_postgres_full_table.yaml", &context)
            .map_err(|e| {
                eprintln!("Error: {:?}", e);
                anyhow!(e)
            })
            .unwrap();

        group.bench_with_input(
            BenchmarkId::new(format!("single_store"), num_rows),
            &num_rows,
            |b, &i| {
                b.to_async(runtime())
                    .iter(|| diff_call(single_store_config_string.clone()))
            },
        );

        let multi_store_config_string = CONF_TEMPLATES
            .render("customer_diff_multi_store_postgres_full_table.yaml", &context)
            .map_err(|e| {
                eprintln!("Error: {:?}", e);
                anyhow!(e)
            })
            .unwrap();

        group.bench_with_input(
            BenchmarkId::new(format!("multi_store"), num_rows),
            &num_rows,
            |b, &i| {
                b.to_async(runtime())
                    .iter(|| diff_call(multi_store_config_string.clone()))
            },
        );
    }
    group.finish();
}

criterion_group! {
    name = postgres_full_table;
    config = Criterion::default().sample_size(20);//measurement_time(std::time::Duration::from_secs(300)).sample_size(10);
    targets = bench
}

criterion_main!(postgres_full_table);
