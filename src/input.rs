use futures::{Future, Poll};
use std::io;
use std::sync::mpsc::{self, Receiver};

pub struct ReadLine {
    recv: Receiver<io::Result<String>>,
}

impl ReadLine {
    pub fn new() -> ReadLine {
        use std::thread;

        let (tx, rx) = mpsc::channel();
        thread::spawn(move || tx.send(read_line()));

        ReadLine {
            recv: rx
        }
    }
}

impl Future for ReadLine {
    type Item = String;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use futures::{Async, task};

        match self.recv.try_recv() {
            Err(_) => {
                task::park().unpark();
                Ok(Async::NotReady)
            },

            Ok(Ok(line)) => Ok(Async::Ready(line)),
            Ok(Err(e)) => Err(e),
        }
    }
}


fn read_line() -> io::Result<String> {
    use std::io::BufRead;
    use std::io::Write;

    print!("Please enter your name: ");
    let _ = io::stdout().flush();
    let input = io::stdin();
    let mut locked = input.lock();
    let mut bfr = String::new();


    match locked.read_line(&mut bfr) {
        Ok(_) => Ok(bfr),
        Err(e) => Err(e),
    }
}
