use std::{fs, path};
use std::fs::File;
use log::{error, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, Config, TerminalMode, TermLogger, WriteLogger};

use shared::{DB_PATH, DB_SOCKET_PATH, LOG_DIR_APP, LOG_FILE_APP};
use shared::toml_schema::read_sys_cfg;
use crate::header::{Command, UnixSock};

mod header;
mod disk;
mod memory;
mod common;
mod util;
mod schema_store;
mod uerr;
mod unix_conn;
mod unix_sock;
mod statement;
mod command;

#[tokio::main]
async fn main() {
    let path_db;
    let path_log;
    let dir_log_server;

    let sys_cfg = read_sys_cfg();
    if sys_cfg.is_ok() {
        let cfg = sys_cfg.unwrap();

        path_db = cfg.moedb.path;
        path_log = cfg.log.moedb;
        dir_log_server = path::Path::new(&path_log).parent().unwrap().to_str().unwrap();
    } else {
        path_db = DB_PATH.to_string();
        path_log = LOG_FILE_APP.to_string();
        dir_log_server = LOG_DIR_APP;

        error!("{}",sys_cfg.err().unwrap());
    }
    fs::create_dir_all(dir_log_server).expect("Error creating log directory");
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Trace, Config::default(), File::create(path_log.as_str()).unwrap()),
            WriteLogger::new(LevelFilter::Debug, Config::default(), File::create(path_log.as_str()).unwrap()),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create(path_log.as_str()).unwrap()),
            WriteLogger::new(LevelFilter::Warn, Config::default(), File::create(path_log.as_str()).unwrap()),
        ]
    ).unwrap();
    fs::remove_file(DB_SOCKET_PATH).unwrap();
    tokio::spawn(async move {
        let command = Command::new().unwrap();
        let unix_sock = UnixSock::new(DB_SOCKET_PATH, command);
        unix_sock.await.unwrap().start();
    }).await.expect("failed to start unix socket");
}
