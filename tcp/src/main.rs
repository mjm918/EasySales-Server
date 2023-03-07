use std::{fs, path, thread};
use std::fs::File;
use std::sync::Arc;
use log::{error, LevelFilter};
use simplelog::*;

use shared::{LOG_DIR_APP, LOG_FILE_APP};
use shared::toml_schema::{CertConfig, read_sys_cfg};
use crate::header::{InsecureTcpServer, InternalTcpServer, SecureTcpServer};

mod header;
mod secure;
mod insecure;
mod secure_conn;
mod insecure_conn;
mod common;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let insecure_tcp_addr;
    let secure_tcp_addr;

    let secure_cert;
    let secure_key;
    let secure_ocsp;

    let dir_log_tcp;
    let path_log_tcp;

    let sys_cfg = read_sys_cfg();
    if sys_cfg.is_ok() {
        let cfg = sys_cfg.unwrap();
        insecure_tcp_addr = format!("127.0.0.1:{}", cfg.ports.tcp);
        secure_tcp_addr = format!("0.0.0.0:{}", cfg.ports.tls_tcp);

        secure_cert = cfg.certs.crt;
        secure_key = cfg.certs.key;
        secure_ocsp = cfg.certs.ca;

        path_log_tcp = cfg.log.tcp;
        dir_log_tcp = path::Path::new(&path_log_tcp).parent().unwrap().to_str().unwrap();
    } else {
        insecure_tcp_addr = "127.0.0.1:8080".to_string();
        secure_tcp_addr = "0.0.0.0:8081".to_string();

        secure_cert = "".to_string();
        secure_key = "".to_string();
        secure_ocsp = "".to_string();

        path_log_tcp = LOG_FILE_APP.to_string();
        dir_log_tcp = LOG_DIR_APP;

        error!("{}",sys_cfg.err().unwrap());
    }
    let cert_cfg = Arc::new(CertConfig {
        cert: secure_cert,
        key: secure_key,
        ocsp: secure_ocsp
    });
    let tcp_cert = cert_cfg.clone();

    fs::create_dir_all(dir_log_tcp).expect("Error creating log directory");
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Trace, Config::default(), File::create(path_log_tcp.as_str()).unwrap()),
            WriteLogger::new(LevelFilter::Debug, Config::default(), File::create(path_log_tcp.as_str()).unwrap()),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create(path_log_tcp.as_str()).unwrap()),
            WriteLogger::new(LevelFilter::Warn, Config::default(), File::create(path_log_tcp.as_str()).unwrap()),
        ]
    ).unwrap();

    if !cfg!(debug_assertions) {
        tokio::spawn(async move {
            let secure_tcp = SecureTcpServer::new(secure_tcp_addr.to_string(), Some(tcp_cert)).await;
            if secure_tcp.is_ok() {
                secure_tcp.unwrap().start();
            } else {
                error!("{}",secure_tcp.err().unwrap());
            }
        });
    }
    tokio::spawn(async move {
        let insecure_tcp = InsecureTcpServer::new(insecure_tcp_addr.to_string(), None).await;
        if insecure_tcp.is_ok() {
            insecure_tcp.unwrap().start();
        } else {
            error!("{}",insecure_tcp.err().unwrap());
        }
    }).await.unwrap();
}
