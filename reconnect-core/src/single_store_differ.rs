use crate::config::{DiffConfig, TableConfig};
use crate::differ::DiffResult;
use crate::store;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

pub struct SingleStoreDiffer {
    config: DiffConfig,
}

impl SingleStoreDiffer {
    pub fn new(config: DiffConfig) -> Self {
        Self { config }
    }

    //TODO - replace most of anyhow errors with valid errors
    pub fn diff(&self, params: HashMap<String, String>) -> anyhow::Result<DiffResult> {
        let left = self.config.left.clone();
        let right = self.config.right.clone();
        let store = store::get_store(&left)?;
        store.diff_datasets(&left, &right)
    }

    //TODO - Need to find a way to do static dispatching here once we have more datasources
}
