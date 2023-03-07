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
use rayon::prelude::*;
use shared::toml_schema::CertConfig;

use crate::header::{InsecureTcpConnection, InsecureTcpServer, InternalTcpServer};
use crate::common::use_default_tcp_server;

const LISTENER: Token = Token(0);
#[async_trait]
impl InternalTcpServer<InsecureTcpServer> for InsecureTcpServer {
    async fn new(address: String, _cfg: Option<Arc<CertConfig>>) -> Result<InsecureTcpServer, Error> {
        let socket_address = address.parse()?;
        let tcp_listener = TcpListener::bind(socket_address);
        if tcp_listener.is_err() {
            return Err(Error::new(tcp_listener.err().unwrap()));
        }
        Ok(Self{
            listener: tcp_listener.unwrap(),
            connections: HashMap::new(),
            next_id: 2,
            socket_address
        })
    }

    fn start(&mut self) {
        debug!("tcp server is starting");

        self.connections.clear();
        /*let mut poll = Poll::new().unwrap();
        poll.registry()
            .register(&mut self.listener, LISTENER, Interest::READABLE)
            .unwrap();

        let mut events = Events::with_capacity(256);*/

        let server_builder = use_default_tcp_server(&mut self.listener, LISTENER);
        let mut poll = server_builder.1;
        let mut events = server_builder.0;

        info!("tcp server started listening {}",self.socket_address.port());

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

                    let token = Token(self.next_id);
                    self.next_id += 1;

                    let mut connection = InsecureTcpConnection::new(socket, token);
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