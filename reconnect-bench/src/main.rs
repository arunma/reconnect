use anyhow::Result as AResult;
use datagen::prepare_database;

fn main() -> AResult<()> {
    let num_rows = 10_000;
    prepare_database(num_rows)?;
    Ok(())
}
