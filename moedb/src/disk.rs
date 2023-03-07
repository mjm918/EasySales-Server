use std::sync::Arc;
use anyhow::{Result,Error};
use log::error;
use rocksdb::DB;
use serde_json::Value;
use shared::{MOEDB_INF, MOEDB_STORE_INF};
use shared::toml_schema::read_sys_cfg;
use crate::common::{check_if_cf_exists, get_cfs, get_rocksdb_cfg};
use crate::header::{CfData, CfDataArray, CfWithInfo, Disk, MoDb};

impl MoDb<Disk> for Disk {
    fn new() -> Result<Disk,Error> {
        let sys_cfg = read_sys_cfg().unwrap();
        let cfs = get_cfs();
        let instance = DB::open_cf(&get_rocksdb_cfg(),sys_cfg.moedb.path,cfs);
        if instance.is_err() {
            return Err(Error::new(instance.err().unwrap()));
        } else {
            let moedb = instance.unwrap();
            // check or create main store
            let inf_exists = check_if_cf_exists(MOEDB_INF);
            if !inf_exists {
                let moedb_inf_created = moedb.create_cf(MOEDB_INF,&get_rocksdb_cfg());
                if moedb_inf_created.is_err() {
                    return Err(Error::new(moedb_inf_created.err().unwrap()));
                }
                let moedb_store_inf_created = moedb.create_cf(MOEDB_STORE_INF,&get_rocksdb_cfg());
                if moedb_store_inf_created.is_err() {
                    return Err(Error::new(moedb_store_inf_created.err().unwrap()));
                }
            }
            Ok(Self {
                db: Arc::new(moedb),
            })
        }
    }

    fn create_db(&self,cf_info: &CfWithInfo) -> bool {
        if self.exists_db(cf_info.name.as_str()) {
            false
        } else {
            let is_db_created = self.db.create_cf(cf_info.name.as_str(), &get_rocksdb_cfg());
            is_db_created.is_ok()
        }
    }

    fn drop_db(&self, name: &str) -> bool {
        if self.exists_db(name) {
            false
        } else {
            let is_db_dropped = self.db.drop_cf(name);
            is_db_dropped.is_ok()
        }
    }

    fn exists_db(&self, name: &str) -> bool {
        check_if_cf_exists(name)
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