use std::io;
use std::net::{AddrParseError, SocketAddr};
use std::sync::Arc;

use anyhow::{Error, Result};
use async_trait::async_trait;
use dashmap::DashMap as HashMap;
use log::{debug, info, warn};
use mio::{Events, Interest, Poll, Registry, Token};
use mio::event::Event;
use mio::net::TcpListener;
use rustls::ServerConnection;
use shared::tls::make_config;
use shared::toml_schema::CertConfig;

use crate::header::{InsecureTcpServer, InternalTcpServer, SecureTcpConnection, SecureTcpServer};
use crate::common::use_default_tcp_server;

const LISTENER: Token = Token(0);
#[async_trait]
impl InternalTcpServer<SecureTcpServer> for SecureTcpServer {
    async fn new(address: String, cfg: Option<Arc<CertConfig>>) -> Result<SecureTcpServer, Error> {
        let socket_address = address.parse()?;
        let tcp_listener = TcpListener::bind(socket_address);
        if tcp_listener.is_err() {
            return Err(Error::new(tcp_listener.err().unwrap()));
        }
        let config = cfg.unwrap();
        Ok(Self{
            listener: tcp_listener.unwrap(),
            connections: HashMap::new(),
            next_id: 2,
            socket_address,
            tls_config: make_config(config.as_ref()),
            user_config: config.as_ref().clone()
        })
    }

    fn start(&mut self) {
        debug!("tcp(tls) server is starting");

        self.connections.clear();

        let server_builder = use_default_tcp_server(&mut self.listener, LISTENER);
        let mut poll = server_builder.1;
        let mut events = server_builder.0;

        info!("tcp(tls) server started listening to {}",self.socket_address.port());

        loop {
            poll.poll(&mut events, None).unwrap();
            for event in events.iter() {
                match event.token() {
                    LISTENER =>{
                        self.accept_connection(poll.registry()).expect("failed to accept connection");
                    },
                    _ => self.connect_event(poll.registry(), event)
                }
            }
        }
    }

    fn connect_event(&mut self, registry: &Registry, event: &Event) {
        let token = event.token();
        if self.connections.contains_key(&token) {
            self.connections
                .get_mut(&token)
                .unwrap()
                .ready(registry, event);

            if self.connections.get(&token).unwrap().is_closed() {
                self.connections.remove(&token);
            }
        }
    }

    fn accept_connection(&mut self, registry: &Registry) -> Result<(), Error> {
        loop {
            match self.listener.accept() {
                Ok((socket, addr)) => {
                    debug!("accepting new connection from {:?}", addr);

                    let tls_conn =
                        ServerConnection::new(Arc::clone(&self.tls_config)).unwrap();

                    let token = Token(self.next_id);
                    self.next_id += 1;

                    let mut connection = SecureTcpConnection::new(socket, token, tls_conn);
                    connection.register(registry);
                    self.connections
                        .insert(token, connection);
                }
                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => return Ok(()),
                Err(err) => {
                    debug!(
                        "encountered error while accepting connection; err={:?}",
                        err
                    );
                    return Err(Error::new(err));
                }
            }
        }
    }
}