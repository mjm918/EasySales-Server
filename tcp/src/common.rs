use mio::{Events, Interest, Poll, Token};
use mio::net::TcpListener;

pub fn use_default_tcp_server(source: &mut TcpListener, token: Token) -> (Events, Poll) {
    let mut poll = Poll::new().unwrap();
    poll.registry()
        .register(source, token, Interest::READABLE)
        .unwrap();

    let mut events = Events::with_capacity(512);

    return (events, poll);
}