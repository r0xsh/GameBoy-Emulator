extern crate ws;
use self::ws::{CloseCode, Handler, Message, Result, Sender, listen};
use std::thread;

pub struct Debugger {
    pub step: u64,
}

struct Websocket(Sender);

impl Debugger {
    pub fn new() -> Debugger {
        Debugger { step: 0 }
    }

    fn listen(&self) {
        thread::spawn(move || { listen("127.0.0.1:3012", |out| Websocket(out)).unwrap(); });
    }
}


impl Handler for Websocket {
    fn on_message(&mut self, _: Message) -> Result<()> {
        unimplemented!();
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("WebSocket closing for ({:?}) {}", code, reason);
    }
}
