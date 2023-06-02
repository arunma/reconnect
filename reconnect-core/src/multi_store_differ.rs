use crate::config::{DiffConfig, TableConfig};
use crate::differ::DiffResult;
use crate::store;
use crate::store::postgres_store::PostgresStore;
use crate::store::RowResult;
use anyhow::Result as AResult;
use log::{error, info};
use std::collections::{HashMap, HashSet};

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

        let lconfig = &self.config.left;
        let rconfig = &self.config.right;

        //TODO - Extend this to compose primary keys
        let mut headers = vec![lconfig.key[0].clone(), rconfig.key[0].clone(), "status".into()];
        headers.extend(lconfig.compare_fields.clone());
        headers.extend(rconfig.compare_fields.clone());
        headers.extend(lconfig.satellite_fields.clone());
        headers.extend(rconfig.satellite_fields.clone());

        if (lsegment.count == rsegment.count) && (lsegment.checksum == rsegment.checksum) {
            return Ok(DiffResult { headers, rows: vec![] });
        }

        //FIXME - This isn't optimal at all, at the moment but let's get something out first
        let lkcs = left_store.get_keys_and_checksums(&self.config.left, lsegment.min, lsegment.max)?;
        let rkcs = right_store.get_keys_and_checksums(&self.config.right, rsegment.min, rsegment.max)?;

        let diff_keys = self.get_diff_keys(&lkcs, &rkcs);
        info!("Diff keys: {:?}", diff_keys);
        //println!("Diff keys: {:?}", diff_keys);
        let (lrow_result, rrow_result) = self.fetch_diff_rows(left_store, right_store, &diff_keys)?;
        return self.build_results_from_values(lrow_result, rrow_result, headers, diff_keys);
    }
    fn build_results_from_values(
        &self,
        lrow_result: RowResult,
        rrow_result: RowResult,
        headers: Vec<String>,
        diff_keys: HashSet<String>,
    ) -> AResult<DiffResult> {
        let mut diff_contents = Vec::with_capacity(diff_keys.len());

        let lcompare_fields =
            self.prefix_alias(&self.config.left.compare_fields, &self.config.left.alias.to_uppercase());
        let rcompare_fields = self.prefix_alias(
            &self.config.right.compare_fields,
            &self.config.right.alias.to_uppercase(),
        );
        let lsatellite_fields = self.prefix_alias(
            &self.config.left.satellite_fields,
            &self.config.left.alias.to_uppercase(),
        );
        let rsatellite_fields = self.prefix_alias(
            &self.config.right.satellite_fields,
            &self.config.right.alias.to_uppercase(),
        );

        for key in diff_keys {
            let lrow = lrow_result.get(&key);
            let rrow = rrow_result.get(&key);

            let diff_content = match (lrow, rrow) {
                (Some(lmap), Some(rmap)) => {
                    let mut diff_content = vec![];
                    diff_content.push(key.clone());
                    diff_content.push("DF".into());
                    for field in &lcompare_fields {
                        diff_content.push(lmap.get(field).unwrap().clone());
                    }
                    for field in &rcompare_fields {
                        diff_content.push(rmap.get(field).unwrap().clone());
                    }
                    for field in &lsatellite_fields {
                        diff_content.push(lmap.get(field).unwrap().clone());
                    }
                    for field in &rsatellite_fields {
                        diff_content.push(rmap.get(field).unwrap().clone());
                    }
                    diff_content
                }
                (Some(lmap), None) => {
                    let mut diff_content = vec![];
                    diff_content.push(key.clone());
                    diff_content.push("LO".into());
                    for field in &lcompare_fields {
                        diff_content.push(lmap.get(field).unwrap().clone());
                    }
                    for _ in &rcompare_fields {
                        diff_content.push("".into());
                    }
                    for field in &lsatellite_fields {
                        diff_content.push(lmap.get(field).unwrap().clone());
                    }
                    for _ in &rsatellite_fields {
                        diff_content.push("".into());
                    }
                    diff_content
                }
                (None, Some(rmap)) => {
                    let mut diff_content = vec![];
                    diff_content.push(key.clone());
                    diff_content.push("RO".into());
                    for _ in &lcompare_fields {
                        diff_content.push("".into());
                    }
                    for field in &rcompare_fields {
                        diff_content.push(rmap.get(field).unwrap().clone());
                    }
                    for _ in &lsatellite_fields {
                        diff_content.push("".into());
                    }
                    for field in &rsatellite_fields {
                        diff_content.push(rmap.get(field).unwrap().clone());
                    }
                    diff_content
                }
                (None, None) => {
                    error!("No row found for key: {}", key);
                    continue;
                }
            };
            diff_contents.push(diff_content);
        }
        Ok(DiffResult {
            headers,
            rows: diff_contents,
        })
    }

    fn prefix_alias(&self, fields: &Vec<String>, alias: &str) -> Vec<String> {
        fields
            .iter()
            .map(|f| format!("{}__{}", alias, f))
            .collect::<Vec<String>>()
    }

    //TODO - Need to make this generic
    fn fetch_diff_rows(
        &self,
        lstore: &mut PostgresStore,
        rstore: &mut PostgresStore,
        diff_keys: &HashSet<String>,
    ) -> AResult<(RowResult, RowResult)> {
        let left_rows = lstore.get_rows_by_keys(&self.config.left, diff_keys)?;
        let right_rows = rstore.get_rows_by_keys(&self.config.right, diff_keys)?;
        Ok((left_rows, right_rows))
    }

    //TODO - Make this generic
    fn get_diff_keys(&self, lkeysums: &HashSet<String>, rkeysums: &HashSet<String>) -> HashSet<String> {
        let mut diff_keys = HashSet::new();
        diff_keys.extend(lkeysums.difference(rkeysums).cloned());
        diff_keys.extend(rkeysums.difference(lkeysums).cloned());
        diff_keys
            .into_iter()
            .map(|k| k.split("__").next().unwrap().into())
            .collect()
    }
}
