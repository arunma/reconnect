use anyhow::anyhow;
use reconnect_core::config::DiffConfig;
use reconnect_core::differ::Differ;

pub mod postgres_full_table;

pub const NUM_ROWS_TO_TEST: [usize; 4] = [10_000, 100_000, 1_000_000, 10_000_000];
//pub const NUM_ROWS_TO_TEST: [usize; 2] = [1000, 10_000];
pub const DIFF_PCT: f64 = 0.10;

pub fn diff_call(config_string: String) -> anyhow::Result<()> {
    let config: DiffConfig = serde_yaml::from_str(&config_string).map_err(|e| {
        eprintln!("Error: {:?}", e);
        anyhow!(e)
    })?;

    let differ = Differ::new(config);

    let diff_result = differ.diff(std::collections::HashMap::new())?;
    //println!("Diff result count {:?}", diff_result.rows.len());

    Ok(())
}
