use std::{fs, path};
use std::fs::File;
use std::os::unix::raw::mode_t;
use std::sync::{Arc, mpsc};
use actix_web::rt;
use futures::task::SpawnExt;
use futures::TryFutureExt;
use log::{error, LevelFilter};
use simplelog::*;
use shared::{LOG_DIR_APP, LOG_FILE_APP};
use shared::toml_schema::{CertConfig, read_sys_cfg};
use crate::header::{InsecureHttpServer, InternalHttpServer, SecureHttpServer};

mod header;
mod secure;
mod insecure;
mod common;

#[tokio::main]
async fn main() {
    let insecure_http_addr;
    let secure_http_addr;

    let secure_cert;
    let secure_key;
    let secure_ocsp;

    let dir_log_server;
    let path_log_server;

    let sys_cfg = read_sys_cfg();
    if sys_cfg.is_ok() {
        let cfg = sys_cfg.unwrap();

        insecure_http_addr = format!("127.0.0.1:{}", cfg.ports.http);
        secure_http_addr = format!("0.0.0.0:{}", cfg.ports.tls_http);

        secure_cert = cfg.certs.crt;
        secure_key = cfg.certs.key;
        secure_ocsp = cfg.certs.ca;

        path_log_server = cfg.log.http;
        dir_log_server = path::Path::new(&path_log_server).parent().unwrap().to_str().unwrap();
    } else {

        insecure_http_addr = "127.0.0.1:9090".to_string();
        secure_http_addr = "0.0.0.0:9091".to_string();

        secure_cert = "".to_string();
        secure_key = "".to_string();
        secure_ocsp = "".to_string();

        path_log_server = LOG_FILE_APP.to_string();
        dir_log_server = LOG_DIR_APP;

        error!("{}",sys_cfg.err().unwrap());
    }
    let cert_cfg = Arc::new(CertConfig {
        cert: secure_cert,
        key: secure_key,
        ocsp: secure_ocsp
    });
    let http_cert = cert_cfg.clone();

    fs::create_dir_all(dir_log_server).expect("Error creating log directory");
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Trace, Config::default(), File::create(path_log_server.as_str()).unwrap()),
            WriteLogger::new(LevelFilter::Debug, Config::default(), File::create(path_log_server.as_str()).unwrap()),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create(path_log_server.as_str()).unwrap()),
            WriteLogger::new(LevelFilter::Warn, Config::default(), File::create(path_log_server.as_str()).unwrap()),
        ]
    ).unwrap();

    if !cfg!(debug_assertions) {
        tokio::spawn(async move {
            let server = SecureHttpServer::new(secure_http_addr.to_string(), None).await.unwrap();
            server.start().await.unwrap();
        });
    }

    tokio::spawn(async move {
        let server = InsecureHttpServer::new(insecure_http_addr.to_string(), None).await.unwrap();
        server.start().await.unwrap();
    }).await.unwrap();
}
