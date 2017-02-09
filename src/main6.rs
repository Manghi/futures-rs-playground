//! This is a basic example of leveraging `UdpCodec` to create a simple UDP
//! client and server which speak a custom protocol.
//!
//! Here we're using the a custom codec to convert a UDP socket to a stream of
//! client messages. These messages are then processed and returned back as a
//! new message with a new destination. Overall, we then use this to construct a
//! "ping pong" pair where two sockets are sending messages back and forth.

extern crate tokio_core;
extern crate env_logger;
extern crate futures;

use std::io;
use std::net::SocketAddr;
use std::str;
use std::thread;

use futures::{Future, Stream, Sink};
use tokio_core::net::{UdpSocket, UdpCodec};
use tokio_core::reactor::Core;

pub struct PacketCodec;

pub struct UdpManager {
        //reader: Stream,
        //writer: Sink,
}

impl UdpCodec for PacketCodec {
    type In = (SocketAddr, Vec<u8>);
    type Out = (SocketAddr, Vec<u8>);

    fn decode(&mut self, addr: &SocketAddr, buf: &[u8]) -> io::Result<Self::In> {
        Ok((*addr, buf.to_vec()))
    }

    fn encode(&mut self, (addr, buf): Self::Out, into: &mut Vec<u8>) -> SocketAddr {
        into.extend(buf);
        addr
    }
}

fn udp_listen() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let addr: SocketAddr = "0.0.0.0:12345".parse().unwrap();

    // Bind both our sockets and then figure out what ports we got.
    let socket = UdpSocket::bind(&addr, &handle).unwrap();

    // We're parsing each socket with the `PacketCodec` defined above, and then we
    // `split` each codec into the sink/stream halves.
    let (writer, reader) = socket.framed(PacketCodec).split();

    // Listen for new messages and then send back a modified responses
    let reader = reader.and_then(|(socket, data)| {
        let stringy = String::from_utf8(data);
        println!("socket: {:?}, data: {:?}", socket, stringy);

        Ok(())
    });


    let writer = writer.send((addr, "I am a packet sent from within.".to_string().into_bytes())).then(|_| {
        println!("Sent message");
        Ok(())
    });


    let reader = reader.for_each(|_| {
        Ok(())
    });

    handle.spawn( writer);

    core.run(reader).unwrap();
}

fn main() {
    drop(env_logger::init());

    thread::spawn(|| udp_listen());

    loop {
        thread::sleep(std::time::Duration::from_secs(1));
    }
}
