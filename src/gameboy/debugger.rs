use serde_json;
use ws::{CloseCode, Handler, Message, Result, Sender, listen};
use std::thread;

#[derive(Serialize, Deserialize, Debug)]
enum Action {
    Next,
    Step(u64),
    AddBreakPtn(u64),
    DelBreakPtn(u64)
}

#[derive(Serialize, Deserialize, Debug)]
struct Handle {
    action: Action,
    opt: u64
}

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
    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Command received: {:?}", handle(&msg));
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("WebSocket closing for ({:?}) {}", code, reason);
    }
}

fn handle(s: &Message) -> Handle {
    serde_json::from_str(s.as_text().unwrap()).unwrap()
}
