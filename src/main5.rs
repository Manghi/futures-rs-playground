extern crate futures;
extern crate tokio_core;

use futures::stream::Stream;
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;

fn main() {
    let mut core = Core::new().unwrap();
    let address = "0.0.0.0:12345".parse().unwrap();
    let listener = TcpListener::bind(&address, &core.handle()).unwrap();

    let connections = listener.incoming();

    // write to the socket, returning a new Stream (Future) 'welcomes'
    // welcomes gets an item from connections, maps it through the closure
    // returning a write_all future. when the future completes, it returns the item it produced
    // as the next item of the welcomes stream
    let welcomes = connections.and_then(|(socket, _peer_addr)| {
        // writes to the socket, returning the socket and ownership
        tokio_core::io::write_all(socket, b"Hello, world!\n")
    });

    // at this point welcomes is a stream which has one socket for each connection,
    // having written "Hello, world!" to each of them
    //
    //  Here we are discarding the results of the write_all future 
    let server = welcomes.for_each(|(_socket, _welcome)| {
        Ok(())
    });

    core.run(server).unwrap();
}
