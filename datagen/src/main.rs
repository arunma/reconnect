use bench::data_generator::postgres_generator::PostgresGenerator;
use bench::data_generator::DataGenerator;
use std::fs;
use std::path::Path;
use std::time::Instant;

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let start = Instant::now();

    let num_rows = 10_000_000;
    let customer1 = Path::new("bench_data/customer1.csv");
    let customer2 = Path::new("bench_data/customer2.csv");
    fs::create_dir_all(customer1.parent().unwrap())?;
    fs::create_dir_all(customer2.parent().unwrap())?;

    let datagen = PostgresGenerator::new();

    datagen.drop_table("customer1")?;
    datagen.drop_table("customer2")?;

    datagen.generate_data_as_csv(num_rows, customer1)?;
    //fs::copy(customer1, customer2).expect("Unable to copy file");
    datagen.introduce_differences_in_csv(customer1, customer2, num_rows, 0.20)?;

    datagen.persist_data_to_database(customer1, "customer1")?;
    datagen.persist_data_to_database(customer2, "customer2")?;

    let end = Instant::now();
    println!("Time taken: {:?}", end - start);

    Ok(())
}
