use crate::config::DiffConfig;
use crate::differ::DiffResult;
use crate::store;

use std::collections::HashMap;

pub struct SingleStoreDiffer {
    config: DiffConfig,
}

impl SingleStoreDiffer {
    pub fn new(config: DiffConfig) -> Self {
        Self { config }
    }

    //TODO - replace most of anyhow errors with valid errors
    pub async fn diff(&self, _params: HashMap<String, String>) -> anyhow::Result<DiffResult> {
        let left = self.config.left.clone();
        let right = self.config.right.clone();
        let mut store = store::get_store(&left).await?;
        store.diff_datasets(&left, &right).await
    }

    //TODO - Need to find a way to do static dispatching here once we have more datasources
}
