use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

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

    pub fn recv(&self) -> Result<T, TryRecvError> {
        self.recv.try_recv()
    }

    pub fn sync_recv(&self) -> T {
        self.recv.recv().unwrap()
    }

    pub fn send(&self, t: T) {
        self.send.send(t).unwrap();
    }
}

pub struct Bus<T> {
    pub left: Channel<T>,
    pub right: Channel<T>,
}

impl<T: Send> Bus<T> {
    pub fn new() -> Bus<T> {

        let (left, right) = Channel::new();

        Bus {
            left: left,
            right: right,
        }
    }
}
