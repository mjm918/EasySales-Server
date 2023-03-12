use std::sync::Arc;
use anyhow::Error;
use crate::header::{CfWithInfo, Memory, MoDb};

impl MoDb<Memory> for Memory {
    fn new() -> Result<Memory,Error> {
        Ok(Self { db: Arc::new(Default::default()) })
    }

    fn create_db(&self, _cf_info: &CfWithInfo) -> bool {
        todo!()
    }

    fn drop_db(&self, _name: &str) -> bool {
        todo!()
    }

    fn exists_db(&self, _name: &str) -> bool {
        todo!()
    }
}