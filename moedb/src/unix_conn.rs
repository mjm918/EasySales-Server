use std::io::{Read, Write};
use std::net::Shutdown;
use std::sync::Arc;
use log::{warn};
use mio::net::UnixStream;
use mio::{Interest, Registry, Token};
use mio::event::Event;
use crate::header::{Command, UnixConn};

impl UnixConn {
    pub fn new(socket: UnixStream, token: Token, command: Arc<Command>) -> Self {
        Self {
            socket,
            token,
            is_writable: false,
            closing: false,
            closed: false,
            message: "".to_string(),
            command
        }
    }

    pub fn ready(&mut self, registry: &Registry, ev: &Event) {
        if ev.is_readable() {
            let mut buffer = vec![0u8;256*1024*1024];
            let nbytes = self.socket.read(&mut buffer).expect("failed to read stream");
            if nbytes == 0 {
                self.unregister(registry);
            } else {
                self.message = String::from_utf8_lossy(&buffer[..nbytes]).to_string();
                let response = self.command.receive(self.message.clone());
                self.socket.write_all(serde_json::to_string(&response).unwrap().as_bytes()).expect(format!("failed to write {response:?}").as_str());
            }
        }
        if ev.is_writable() {
            self.is_writable = true;
        } else {
            self.closing = false;
            self.is_writable = false;
        }

        if self.closing {
            self.socket.shutdown(Shutdown::Both).expect(format!("failed to shutdown {:?}",self.socket.peer_addr()).as_str());
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
            warn!("{}. socket dropped",task.err().unwrap());
        }
    }

    pub fn unregister(&mut self, registry: &mio::Registry) {
        let task = registry
            .deregister(&mut self.socket);
        if task.is_ok() {
            task.unwrap();
        } else {
            warn!("{}. socket dropped",task.err().unwrap());
        }
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }
}
