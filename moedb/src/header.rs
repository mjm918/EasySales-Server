use std::sync::Arc;
use anyhow::{Result,Error};
use dashmap::DashMap;
use mio::net::{UnixListener, UnixStream};
use mio::Token;
use rocksdb::{DBWithThreadMode, MultiThreaded};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use crate::uerr::StatementError;

pub struct Command {
    pub disk: Disk,
    pub mem: Memory
}

pub struct UnixSock {
    pub connections: DashMap<Token,UnixConn>,
    pub next_id: usize,
    pub listener: UnixListener,
    pub command: Arc<Command>
}

pub struct UnixConn {
    pub socket: UnixStream,
    pub token: Token,
    pub is_writable: bool,
    pub closing: bool,
    pub closed: bool,
    pub message: String,
    pub command: Arc<Command>
}

pub trait MoDb<T> {
    fn new() -> Result<T,Error>;

    fn create_db(&self, cf_info:&CfWithInfo) -> bool;
    fn drop_db(&self, cf_info:&str) -> bool;
    fn exists_db(&self, name:&str) ->  bool;
}


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SocketResponse {
    pub err: bool,
    pub message: StatementError,
    pub data: Value
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CfData {
    pub cf: String,
    pub data: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CfDataArray {
    pub cf: String,
    pub data: Vec<String>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CfWithInfo {
    pub name: String,
    pub unique_key: Vec<String>,
    pub in_memory: bool
}

#[derive(Clone)]
pub struct Memory {
    pub(crate) db: Arc<DashMap<String,DashMap<String,Value>>>
}

#[derive(Clone)]
pub struct Disk {
    pub(crate) db: Arc<DBWithThreadMode<MultiThreaded>>
}
#[derive(Clone)]
pub struct ParsedStatement {
    pub cmd: Option<StatementType>,
    pub db: String,
    pub store: String,
    pub pbs_data: String
}
#[derive(Clone,Ord, PartialOrd, Eq, PartialEq,Debug)]
pub enum StatementType {
    Create,
    CreateDb,
    Get,
    Upsert,
    Delete,
    Drop,
    DropDb,
    DbList,
    Truncate
}
