use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

pub struct Channel<T> {
    send: Sender<T>,
    recv: Receiver<T>,
}

impl<T: Send> Channel<T> {
    pub fn new() -> (Channel<T>, Channel<T>) {
        let (tx, rx) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();

        (Channel {
             send: tx2,
             recv: rx,
         },
         Channel {
             send: tx,
             recv: rx2,
         })
    }

    pub fn recv(&self) -> T {
        self.recv.recv().unwrap()
    }

    pub fn send(&self, t: T) {
        self.send.send(t).unwrap();
    }
}

pub struct Bus<T> {
    pub chan1: Channel<T>,
    pub chan2: Channel<T>,
}

impl<T: Send> Bus<T> {
    pub fn new() -> Bus<T> {

        let (chan1, chan2) = Channel::new();

        Bus {
            chan1: chan1,
            chan2: chan2,
        }
    }
}