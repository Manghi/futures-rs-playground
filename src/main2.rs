extern crate futures;

use std::io::{self, Result};
use std::thread;
use futures::{Future, Oneshot};

fn main() {
    match read_name() {
        Err(_) => println!("Hey there stranger..."),
        Ok(name) => println!("Hello, {}!", name.trim()),
    }
}

// In this example we'll be using the oneshot future
fn read_name() -> Result<String> {
    match read_input().select(timeout()).wait() {
        // Error type is Task::Cancelled so we're going to assume that is never going to happen
        Err(_) => unreachable!(),
        Ok((complete, _)) => complete, // remember select will chose two, and give you both ordered
    }
}

fn read_input() -> Oneshot<Result<String>> {
    use std::io::BufRead;


    let (c, p) = futures::oneshot();
    // This is similar to std's channel, returns a Sender and Receiver halves
    // Sender signals a value compution completion and provides it to the Receiver
    // The Receiver implements the Future which is waiting on the computation completion

    thread::spawn(|| {
        let input = io::stdin();
        let mut input = input.lock();
        let mut buf = String::new();

        let value = input.read_line(&mut buf).map(|_| buf);
        c.complete(value);
    });

    p
}

fn timeout() -> Oneshot<Result<String>> {
    use std::time::Duration;

    let (c,p) = futures::oneshot();
    thread::spawn(|| {
        thread::sleep(Duration::from_secs(5));
        c.complete(Err(io::Error::new(io::ErrorKind::Other, "Timeout elapsed...")));
    });

    p
}
