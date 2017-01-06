extern crate futures;

use std::io;

mod input;
mod timeout;

fn main() {
    match read_name() {
        Err(err) => println!("{}, Hello, nameless!", err),
        Ok(name) => println!("Hello, {}!", name.trim()),
    }
}

fn read_name() -> io::Result<String> {
    use futures::Future;
    use input::ReadLine;
    use std::time::Duration;
    use timeout::Timeout;

    // .select() will return the result of the first completed future, either
    // the 'empty' or the timeout
    //
    // But we want to return something that will return your name instead of the empty future
    //
    let result = ReadLine::new()
        .select(Timeout::new(Duration::from_secs(5), || {
            io::Error::new(io::ErrorKind::Other, "Timeout elapsed... ".to_string())
        }))
        .wait();

    match result {
        // result returns a (Future_A, Future_B), where A is the first completed future
        Ok((name, _)) => Ok(name),
        Err((e,_)) => Err(e),
    }

    // We are not interested in the other future so we'll just drop it
}
