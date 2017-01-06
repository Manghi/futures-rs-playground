extern crate futures;

use futures::{Future, Poll};
use std::io;
use std::marker::PhantomData;
use std::time::{Duration, Instant};

pub struct Timeout<T,E> {
    timestamp: Instant,
    duration: Duration,
    error: E,
    phantom: PhantomData<T>,
}

impl<T,E> Timeout<T,E>
    where E: Fn() -> io::Error
{
    pub fn new(duration: Duration, e: E) -> Timeout<T, E> {
        Timeout {
            timestamp: Instant::now(),
            duration: duration,
            phantom: PhantomData,
            error: e,
        }
    }

    pub fn is_elapsed(&self) -> bool {
        self.timestamp.elapsed() >= self.duration
    }
}

impl<T,E> Future for Timeout<T,E>
    where E: Fn() -> io::Error
{
    type Item = T;
    type Error = io::Error;


    // Return type of the Future::poll method, indicates whether a future's value is ready or not.
    //
    // Ok(Async::Ready(t)) means that a future has successfully resolved
    // Ok(Async::NotReady) means that a future is not ready to complete yet
    // Err(e) means that a future has completed with the given failure

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use futures::{Async, task};

        if self.is_elapsed() {
            Err((self.error)())
        } else {
            task::park().unpark(); // this tells the task driving the future to recheck this again
            // in the future. Otherwise poll will only be run once.
            Ok(Async::NotReady)
        }
    }
}
