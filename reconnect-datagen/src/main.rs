use anyhow::Result as AResult;
use reconnect_datagen::prepare_database;
use std::path::Path;

fn main() -> AResult<()> {
    let num_rows = 10_000;
    let root_path = Path::new("../bench_data");
    prepare_database(root_path, num_rows)?;
    Ok(())
}
