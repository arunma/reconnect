use crate::config::DiffConfig;
use crate::multi_store_differ::MultiStoreDiffer;
use crate::single_store_differ::SingleStoreDiffer;
use anyhow::Result as AResult;
use log::info;
use std::any::Any;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Differ {
    config: DiffConfig,
}

#[derive(Debug)]
pub struct DiffResult {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

//Need to figure out how to do inheritance/enum scoping here
//may be enum Differ(SingleDiffer,MultiDiffer)?

impl Differ {
    pub fn new(config: DiffConfig) -> Self {
        Self { config }
    }

    pub fn diff(&self, params: HashMap<String, String>) -> AResult<DiffResult> {
        if self.config.left.connection_uri == self.config.right.connection_uri {
            //TODO: Implement single store diff
            let single_store_diff = SingleStoreDiffer::new(self.config.clone());
            return single_store_diff.diff(params);
        } else {
            info!("MULTI store diff ");
            let multi_store_diff = MultiStoreDiffer::new(self.config.clone());
            return multi_store_diff.diff(params);
        }

        Ok(DiffResult {
            headers: vec![],
            rows: vec![],
        })
    }
}
