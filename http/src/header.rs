use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use actix_web::dev::{Server, ServerHandle};
use anyhow::{Result, Error};
use async_trait::async_trait;
use rustls::ServerConfig;
use shared::toml_schema::CertConfig;
use serde_derive::Deserialize;

#[async_trait]
pub trait InternalHttpServer<T> {
    async fn new(address: String, tls_cfg: Option<Arc<CertConfig>>) -> Result<T,Error>;
    fn start(&self) -> Server;
}

#[derive(Debug)]
pub struct SecureHttpServer {
    pub address: SocketAddr,
    pub tls_config: Arc<ServerConfig>,
    pub sys_config: CertConfig
}

#[derive(Debug)]
pub struct InsecureHttpServer {
    pub address: SocketAddr
}