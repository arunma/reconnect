use std::io;
use std::path::Path;

pub mod constants;
pub mod postgres_generator;

pub trait DataGenerator {
    fn generate_data_as_csv(&self, num_rows: usize, file_path: &Path) -> io::Result<()>;
    fn persist_data_to_database(&self, file_path: &Path, table_name: &str) -> anyhow::Result<()>;
    fn introduce_differences_in_csv(
        &self,
        source_path: &Path,
        target_path: &Path,
        num_rows: usize,
        diff_pct: f64,
    ) -> anyhow::Result<()>;
}
