use std::io;
use std::io::{Read, Write};
use std::net;

use log::{debug, error};
use mio::net::TcpStream;
use mio::Token;
use rustls::{ServerConfig, ServerConnection};

use shared::validator::try_read;

use crate::header::SecureTcpConnection;

impl SecureTcpConnection {
    pub fn new(socket: TcpStream, token: Token, tls_conn: ServerConnection) -> Self {
        Self {
            socket,
            token,
            is_writable: false,
            closing: false,
            closed: false,
            message: "".to_string(),
            tls_conn
        }
    }

    pub fn ready(&mut self, registry: &mio::Registry, ev: &mio::event::Event) {
        if ev.is_readable() {
            self.do_tls_read();
            self.try_plain_read();
        }

        if ev.is_writable() {
            self.do_tls_write_and_handle_error();
        }

        if self.closing {
            let _ = self
                .socket
                .shutdown(net::Shutdown::Both);
            self.closed = true;
            self.unregister(registry);
        } else {
            self.re_register(registry);
        }
    }

    pub fn do_tls_read(&mut self) {
        match self.tls_conn.read_tls(&mut self.socket) {
            Err(err) => {
                if let io::ErrorKind::WouldBlock = err.kind() {
                    return;
                }
                error!("read error {:?}", err);
                self.closing = true;
                return;
            }
            Ok(0) => {
                debug!("disconnected!");
                self.closing = true;
                return;
            }
            Ok(_) => {}
        };

        if let Err(err) = self.tls_conn.process_new_packets() {
            error!("cannot process packet: {:?}", err);
            self.do_tls_write_and_handle_error();
            self.closing = true;
        }
    }

    pub fn try_plain_read(&mut self) {
        if let Ok(io_state) = self.tls_conn.process_new_packets() {
            if io_state.plaintext_bytes_to_read() > 0 {
                let mut buf = Vec::new();
                buf.resize(io_state.plaintext_bytes_to_read(), 0u8);

                self.tls_conn
                    .reader()
                    .read_exact(&mut buf)
                    .unwrap();

                debug!("plaintext read {:?}", buf.len());
                self.incoming_plaintext(&buf);
            }
        }
    }

    pub fn incoming_plaintext(&mut self, buf: &[u8]) {
        self.tls_conn
            .writer()
            .write_all(buf)
            .unwrap();
    }

    pub fn tls_write(&mut self) -> io::Result<usize> {
        self.tls_conn
            .write_tls(&mut self.socket)
    }

    pub fn do_tls_write_and_handle_error(&mut self) {
        let rc = self.tls_write();
        if rc.is_err() {
            error!("write failed {:?}", rc);
            self.closing = true;
        }
    }

    pub fn register(&mut self, registry: &mio::Registry) {
        let event_set = self.event_set();
        registry
            .register(&mut self.socket, self.token, event_set)
            .unwrap();
    }

    pub fn re_register(&mut self, registry: &mio::Registry) {
        let event_set = self.event_set();
        registry
            .reregister(&mut self.socket, self.token, event_set)
            .unwrap();
    }

    pub fn unregister(&mut self, registry: &mio::Registry) {
        registry
            .deregister(&mut self.socket)
            .unwrap();
    }

    pub fn event_set(&self) -> mio::Interest {
        let rd = self.tls_conn.wants_read();
        let wr = self.tls_conn.wants_write();

        if rd && wr {
            mio::Interest::READABLE | mio::Interest::WRITABLE
        } else if wr {
            mio::Interest::WRITABLE
        } else {
            mio::Interest::READABLE
        }
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }
}