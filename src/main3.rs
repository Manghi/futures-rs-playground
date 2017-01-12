extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;
use tokio_core::io::{Codec, EasyBuf};

pub struct LineCodec;   // Codecs may need a local state, potentially recording information about
                        // incomplete decoding/encoding

fn main() {

}
