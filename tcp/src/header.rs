use std::net::SocketAddr;
use std::sync::Arc;
use anyhow::{Error, Result};
use async_trait::async_trait;
use dashmap::{DashMap as HashMap};
use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Registry, Token};
use rustls::{ServerConfig, ServerConnection};
use serde_derive::Deserialize;
use shared::toml_schema::CertConfig;

#[async_trait]
pub trait InternalTcpServer<T> {
    async fn new(address: String, tls_cfg: Option<Arc<CertConfig>>) -> Result<T,Error>;
    fn start(&mut self);
    fn connect_event(&mut self, registry: &Registry, event: &Event);
    fn accept_connection(&mut self, registry: &Registry) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct InsecureTcpServer {
    pub listener: TcpListener,
    pub connections: HashMap<Token, InsecureTcpConnection>,
    pub next_id: usize,
    pub socket_address: SocketAddr
}

#[derive(Debug)]
pub struct SecureTcpServer {
    pub listener: TcpListener,
    pub connections: HashMap<Token, SecureTcpConnection>,
    pub next_id: usize,
    pub tls_config: Arc<ServerConfig>,
    pub user_config: CertConfig,
    pub socket_address: SocketAddr
}

#[derive(Debug)]
pub struct InsecureTcpConnection {
    pub socket: TcpStream,
    pub token: Token,
    pub is_writable: bool,
    pub closing: bool,
    pub closed: bool,
    pub message: String
}

#[derive(Debug)]
pub struct SecureTcpConnection {
    pub socket: TcpStream,
    pub token: Token,
    pub is_writable: bool,
    pub closing: bool,
    pub closed: bool,
    pub message: String,
    pub tls_conn: ServerConnection
}
