mod benchmarks;

use criterion::criterion_main;

criterion_main! {
    // benchmarks::single_source_postgres_full_table::single_source,
    // benchmarks::multi_source_postgres_full_table::multi_source,
    benchmarks::postgres_full_table::postgres_full_table
}
