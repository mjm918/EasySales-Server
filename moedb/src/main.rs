use std::time::Instant;
use serde_json::{from_str, Value};

use shared::DB_PATH;
use shared::toml_schema::read_sys_cfg;

mod header;
mod disk;
mod memory;
mod common;
mod manipulator;
mod schema;
mod uerr;

fn main() {
    let path_db;
    let sys_cfg = read_sys_cfg();
    if sys_cfg.is_ok() {
        let cfg = sys_cfg.unwrap();
        path_db = cfg.moedb.path;
    } else {
        path_db = DB_PATH.to_string();
    }
    let schema_v = schema::is_schema_ok("Hello World");
    if schema_v.is_err() {
        println!("{}",schema_v.err().unwrap().to_string());
    }
}
