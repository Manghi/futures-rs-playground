extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

use std::{io,str};
use tokio_core::io::{Codec, EasyBuf};
use tokio_proto::pipeline::ServerProto;
use tokio_service::Service;
use futures::{Future, future, BoxFuture};

pub struct LineCodec;   // Codecs may need a local state, potentially recording information about
                        // incomplete decoding/encoding

// Codec is a transport helper for serialization requests and responses to a socket
impl Codec for LineCodec {
    type In = String;
    type Out = String;

    // EasyBuf: A reference counted buffer of bytes
    // An EasyBuf is a representation of a byte buffer where sub-slices of
    // it can be handed out efficiently, each with a 'static lifetime which keeps the data alive.
    // The buffer also supports mutation but may require bytes to be copied to complete the operation.
    //
    // We return Result to convey service errors
    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        if let Some(i) = buf.as_slice().iter().position(|&b| b == b'\n') { // search for '\n' in buf'
            // remove the serialzed element frame from the buffer since we're delimiting on newline
            let line = buf.drain_to(i);
            buf.drain_to(1); // also remove the '\n'

            // Translate into a UTF8 string and return it as a frame
            return match str::from_utf8(line.as_ref()) {
                Ok(s) => Ok(Some( s.to_string() )),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid UTF-8")),
            }

        } else {
            Ok(None)
            // The server will automatically fetch more data before trying to decode again
        }
    }

    fn encode(&mut self, msg: String, buf: &mut Vec<u8>) -> io::Result<()> {
        buf.extend_from_slice(msg.as_bytes());
        buf.push(b'\n');
        Ok(())
    }
}

// Here is our protocol specification. It puts together a codec and basic info about the protocol itself

// tokio-proto can handle various protocol styles including multiplexed and streaming
// This example will use a pipelined, non-streaming protocol, ServerProto
pub struct LineProto;
use tokio_core::io::{Io, Framed};

impl<T: Io + 'static> ServerProto<T> for LineProto {
    // Request should match the 'In' type
    // Response should match the codec 'Out' type

    type Request = String;
    type Response = String;

    // This doesn't change much from instantiation to instantiation according to the tutorial
    //
    // Framed actually hides a lot of the details behind it. Really we're building a Stream + Sink...thing
    // for the Request and Response, respectively
    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LineCodec))
    }
}

// A service will /use/ the protcol we defined above. The service will know how to response to requests
// using tokio-service::Service

// We'll be echoing the Servers requests as responses
pub struct Echo;

impl Service for Echo {
    // These types must match the corresponding protocol types:
    type Request = String;
    type Response = String;

    // For non-streaming protocols, service errors are always io::Error
    type Error = io::Error;


    // The future for computing the response; box it for simplicity.
    type Future = BoxFuture<Self::Response, Self::Error>;

    // Produce a Response Future using a Request
    fn call(&self, req: Self::Request) -> Self::Future {
        let rev: String = req.chars()
                             .rev()
                             .collect();
        future::ok(rev).boxed()
    }
}





use tokio_proto::TcpServer;

fn main() {
    // Specify the localhost address
    let addr = "0.0.0.0:12345".parse().unwrap();

    // The builder requires a protocol and an address
    // We are going to use the protocol we defined above
    let server = TcpServer::new(LineProto, addr);

    // We provide a way to *instantiate* the service for each new
    // connection; here, we just immediately return a new instance.
    server.serve(|| Ok(Echo));
}
