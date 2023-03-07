use std::io;
use std::io::{Read, Write};
use std::net;
use std::net::Shutdown;

use log::{error, info};
use mio::{Interest, Registry, Token};
use mio::event::Event;
use mio::net::TcpStream;

use shared::validator::try_read;

use crate::header::{InsecureTcpConnection};

impl InsecureTcpConnection {
    pub fn new(socket: TcpStream, token: Token) -> Self {
        Self {
            socket,
            token,
            is_writable: false,
            closing: false,
            closed: false,
            message: "".to_string(),
        }
    }

    pub fn ready(&mut self, registry: &Registry, ev: &Event) {
        if ev.is_readable() {
            let mut buffer = [0; 4096];
            let nbytes = self.socket.read(&mut buffer).expect("failed to read");
            if nbytes == 0 {
                self.unregister(registry);
            } else {
                self.message = String::from_utf8_lossy(&buffer[..nbytes]).to_string();
            }
        }
        if ev.is_writable() {
            self.is_writable = true;
        } else {
            self.closing = false;
            self.is_writable = false;
        }

        if self.closing {
            self.socket.shutdown(Shutdown::Both).expect("failed to shutdown socket");
            self.closed = true;
            self.unregister(registry);
        } else {
            self.re_register(registry);
        }
    }

    pub fn register(&mut self, registry: &Registry) {
        registry
            .register(&mut self.socket, self.token, Interest::READABLE | Interest::WRITABLE)
            .unwrap();
    }

    pub fn re_register(&mut self, registry: &Registry) {
        let task = registry
            .reregister(&mut self.socket, self.token, Interest::READABLE | Interest::WRITABLE);
        if task.is_ok() {
            task.unwrap();
        } else {
            error!("{}. Socket dropped",task.err().unwrap());
        }
    }

    pub fn unregister(&mut self, registry: &mio::Registry) {
        let task = registry
            .deregister(&mut self.socket);
        if task.is_ok() {
            task.unwrap();
        } else {
            error!("{}. Socket dropped",task.err().unwrap());
        }
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }
}