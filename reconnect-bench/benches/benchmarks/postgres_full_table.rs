use crate::benchmarks::{diff_call, DIFF_PCT, NUM_ROWS_TO_TEST};
use anyhow::anyhow;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use datagen::prepare_database;
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;
use tera::{Context, Tera};

lazy_static! {
    pub static ref CONF_TEMPLATES: Tera = {
        let mut tera = Tera::new("../examples/conf/*").unwrap();
        tera.autoescape_on(vec![]);
        tera
    };
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("postgres_full_table");
    dotenv().ok();
    let mut context = Context::new();
    for (key, value) in env::vars() {
        context.insert(key, &value);
    }

    for num_rows in NUM_ROWS_TO_TEST {
        if let Err(e) = prepare_database(num_rows) {
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
            |b, &i| b.iter(|| diff_call(single_store_config_string.clone())),
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
            |b, &i| b.iter(|| diff_call(multi_store_config_string.clone())),
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
