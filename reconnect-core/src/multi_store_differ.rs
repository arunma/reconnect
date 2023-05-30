use crate::config::{DiffConfig, TableConfig};
use crate::differ::DiffResult;
use crate::store;
use crate::store::postgres_store::PostgresStore;
use crate::store::Segment;
use anyhow::Result as AResult;
use log::info;
use std::collections::HashMap;

pub struct MultiStoreDiffer {
    config: DiffConfig,
}

impl MultiStoreDiffer {
    pub fn new(config: DiffConfig) -> Self {
        Self { config }
    }

    pub fn diff(&self, params: HashMap<String, String>) -> AResult<DiffResult> {
        let mut left_store = store::get_store(&self.config.left)?;
        let mut right_store = store::get_store(&self.config.right)?;
        self.diff_datasets(&mut left_store, &mut right_store, params)
    }
    fn diff_datasets(
        &self,
        left_store: &mut PostgresStore,
        right_store: &mut PostgresStore,
        params: HashMap<String, String>,
    ) -> AResult<DiffResult> {
        //TODO - This has to be two calls - One fore global segment and then branched recursive segment
        let lsegment = left_store.get_agg_count_and_checksums(&self.config.left, params.clone())?;
        let rsegment = right_store.get_agg_count_and_checksums(&self.config.right, params.clone())?;

        println!("lsegment: {:?}", lsegment);
        println!("rsegment: {:?}", rsegment);

        let lconfig = &self.config.left;
        let rconfig = &self.config.right;

        //TODO - Extend this to compose primary keys
        let mut headers = vec![lconfig.key[0].clone(), rconfig.key[0].clone(), "status".into()];
        headers.extend(lconfig.compare_fields.clone());
        headers.extend(rconfig.compare_fields.clone());
        headers.extend(lconfig.satellite_fields.clone());
        headers.extend(rconfig.satellite_fields.clone());

        if lsegment.count == rsegment.count {
            return Ok(DiffResult { headers, rows: vec![] });
        }

        //FIXME - This isn't optimal at all, at the moment but let's get something out first
        let lkcs = left_store.get_keys_and_checksums(&self.config.left, lsegment.min, lsegment.max)?;
        let rkcs = right_store.get_keys_and_checksums(&self.config.right, rsegment.min, rsegment.max)?;

        let diff_keys = self.get_diff_keys(&lkcs, &rkcs);
        info!("Diff keys: {:?}", diff_keys);

        /*
        let (lvalues, rvalues) = self.fetch_diff_rows(left_store, right_store, diff_keys);

        return self.build_results_from_values(lvalues, rvalues, headers);*/
        todo!()
    }
    fn build_results_from_values(
        &self,
        left_values: Vec<String>,
        right_values: Vec<String>,
        headers: Vec<String>,
    ) -> AResult<DiffResult> {
        todo!()
    }

    //TODO - Need to make this generic
    fn fetch_diff_rows(
        &self,
        lstore: &PostgresStore,
        rstore: &PostgresStore,
        keys: Vec<String>,
    ) -> AResult<(Vec<String>, Vec<String>)> {
        todo!()
    }

    //TODO - Make this generic
    fn get_diff_keys(&self, lkeysums: &HashMap<String, String>, rkeysums: &HashMap<String, String>) -> Vec<String> {}
}
