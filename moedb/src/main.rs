use std::{fs, path};
use std::fs::File;
use std::time::Instant;
use log::{debug, error, LevelFilter};
use serde_json::{from_str, Value};
use simplelog::{ColorChoice, CombinedLogger, Config, TerminalMode, TermLogger, WriteLogger};

use shared::{DB_PATH, DB_SOCKET_PATH, LOG_DIR_APP, LOG_FILE_APP};
use shared::toml_schema::read_sys_cfg;
use crate::header::UnixSock;

mod header;
mod disk;
mod memory;
mod common;
mod manipulator;
mod schema;
mod uerr;
mod unix_conn;
mod unix_sock;
mod statement;

/*use tokio::net::UnixListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_path = Path::new(DB_SOCKET_PATH);
    let listener = UnixListener::bind(socket_path)?;

    println!("Listening for connections on {:?}", socket_path);

    loop {
        let (mut stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buffer = [0; 1024];

            loop {
                match stream.read(&mut buffer).await {
                    Ok(0) => break,
                    Ok(n) => {
                        println!("Read {} bytes", n);

                        // Process the received data here...
                        let response = b"OK";
                        stream.write_all(response).await.unwrap();
                    },
                    Err(e) => {
                        eprintln!("Error reading from socket; error = {:?}", e);
                        break;
                    }
                }
            }
        });
    }
}*/


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
    std::fs::remove_file(DB_SOCKET_PATH).unwrap();
    tokio::spawn(async move {
        let mut unix_sock = UnixSock::new(DB_SOCKET_PATH);
        unix_sock.await.unwrap().start();
    }).await.expect("failed to start unix socket");
}
