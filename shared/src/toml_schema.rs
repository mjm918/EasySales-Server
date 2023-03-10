use anyhow::{Result,Error};
use std::fs;
use serde_derive::{Deserialize};
use crate::SYS_CFG;

#[derive(Debug, Deserialize, Clone)]
pub struct BaseConfig {
    pub ports: Ports,
    pub certs: Certs,
    pub log: Log,
    pub moedb: MoeDb
}

#[derive(Debug, Deserialize,Clone)]
pub struct Ports {
    pub tcp: i32,
    pub http: i32,
    pub tls_tcp: i32,
    pub tls_http: i32
}

#[derive(Debug, Deserialize,Clone)]
pub struct Certs {
    pub crt: String,
    pub key: String,
    pub ca: String
}

#[derive(Debug, Deserialize,Clone)]
pub struct Log {
    pub app: String,
    pub tcp: String,
    pub http: String,
    pub moedb: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct CertConfig {
    pub cert: String,
    pub key: String,
    pub ocsp: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct MoeDb {
    pub path: String,
    pub ops_log: String,
    pub strict_mode: bool,
    pub date_format: String,
    pub default_auth: Vec<String>
}

pub fn read_sys_cfg() -> Result<BaseConfig,Error> {
    let content = fs::read_to_string(SYS_CFG);
    if content.is_err() {
        return Err(Error::new(content.err().unwrap()));
    }
    let sys_cfg = toml::from_str(content.unwrap().as_str());
    if sys_cfg.is_err() {
        return Err(Error::new(sys_cfg.err().unwrap()));
    }
    Ok(sys_cfg.unwrap())
}
