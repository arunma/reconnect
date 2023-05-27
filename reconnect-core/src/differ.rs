use crate::config::DiffConfig;
use crate::single_store_differ::SingleStoreDiffer;
use std::any::Any;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Differ {
    config: DiffConfig,
}

#[derive(Debug)]
pub struct DiffResult {
    pub headers: Vec<String>,
    pub rows: Vec<Box<dyn Any>>,
}

impl Differ {
    pub fn new(config: DiffConfig) -> Self {
        Self { config }
    }

    pub fn diff(&self, params: HashMap<String, String>) -> anyhow::Result<DiffResult> {
        if self.config.left.connection_uri == self.config.right.connection_uri {
            //TODO: Implement single store diff
            let single_store_diff = SingleStoreDiffer::new(self.config.clone());
            return single_store_diff.diff(params);
        } else {
            /*let multi_store_diff = MultiStoreDiff::new(self.config.clone());
            multi_store_diff.diff(params)*/
            todo!("Implement multi store diff")
        }

        Ok(DiffResult {
            headers: vec![],
            rows: vec![],
        })
    }
}
