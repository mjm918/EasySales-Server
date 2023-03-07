use anyhow::Error;
use serde_json::Value;
use crate::header::{CfData, CfDataArray, CfWithInfo, Memory, MoDb};

impl MoDb<Memory> for Memory {
    fn new() -> Result<Memory,Error> {
        todo!()
    }

    fn create_db(&self, cf_info: &CfWithInfo) -> bool {
        todo!()
    }

    fn drop_db(&self, name: &str) -> bool {
        todo!()
    }

    fn exists_db(&self, name: &str) -> bool {
        todo!()
    }

    fn upsert(data: CfData) -> bool {
        todo!()
    }

    fn upsert_bulk(data_arr: CfDataArray) -> bool {
        todo!()
    }

    fn delete(data: CfData) -> bool {
        todo!()
    }

    fn delete_bulk(data_arr: CfDataArray) -> bool {
        todo!()
    }

    fn get(query: String) -> Vec<Value> {
        todo!()
    }
}