use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct TableConfig {
    pub connection_uri: String,
    pub schema: String,
    pub table: String,
    pub alias: String,
    pub key: Vec<String>,
    pub satellite_fields: Vec<String>,
    pub compare_fields: Vec<String>,
    pub filter_conditions: Vec<String>,
    pub exclude_fields: Vec<String>,
    pub date_fields: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ResultConfig {
    pub connection_uri: String,
    pub schema: String,
    pub diff_table: String,
    pub summary_table: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DiffConfig {
    pub name: String,
    pub left: TableConfig,
    pub right: TableConfig,
    pub result: ResultConfig,
}
