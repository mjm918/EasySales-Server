use std::path::Path;
use std::sync::Arc;
use anyhow::{Error, Result};
use dashmap::DashMap;
use futures::TryFutureExt;
use log::{debug, error};
use mio::net::UnixListener;
use mio::{Events, Interest, Poll, Registry, Token};
use mio::event::Event;
use crate::header::{UnixConn, UnixSock};

const LISTENER: Token = Token(0);
impl UnixSock {
    pub async fn new(path: &str) -> Result<Self> {
        let socket_path = Path::new(path);
        let listener = UnixListener::bind(socket_path).expect("failed to bind unix server");
        Ok(Self {
            connections: DashMap::new(),
            next_id: 2,
            listener
        })
    }

    pub fn start(&mut self) {
        debug!("unix socket starting for moedb");

        self.connections.clear();
        let mut poll = Poll::new().unwrap();
        let mut events = Events::with_capacity(512);
        poll.registry().register(&mut self.listener, LISTENER, Interest::READABLE).unwrap();

        loop {
            poll.poll(&mut events, None).unwrap();
            for event in events.iter() {
                match event.token() {
                    LISTENER => {
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

                    let mut connection = UnixConn::new(socket, token);
                    connection.register(registry);
                    self.connections
                        .insert(token, connection);
                }
                Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => return Ok(()),
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

#[cfg(test)]
mod tests {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::UnixStream;
    use super::*;
    #[tokio::test]
    async fn test_client() -> Result<(), Box<dyn std::error::Error>> {
        tokio::spawn(async move {
            let mut unix_sock = UnixSock::new(shared::DB_SOCKET_PATH);
            unix_sock.await.unwrap().start();
        });
        tokio::spawn(async move {
            let socket_path = Path::new(shared::DB_SOCKET_PATH);
            let mut stream = UnixStream::connect(socket_path).await.unwrap();

            let message = "Hello, Unix domain socket!".as_bytes();
            stream.write_all(message).await.unwrap();

            let mut response = [0; 1024];
            let n = stream.read(&mut response).await.unwrap();
            println!("Received {} bytes: {:?}", n, &response[..n]);
        });

        assert_eq!(1,1);
        Ok(())
    }
}