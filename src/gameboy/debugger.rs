use ws;
use ws::{CloseCode, Handler, Message , Sender, listen};
use std::thread;

#[derive(Debug)]
enum Action {
    Next,
    Step(u64),
    AddBreakPtn(u64),
    DelBreakPtn(u64),
    Err
}

pub struct Debugger {
    pub step: u64,
}

struct Websocket(Sender);

impl Debugger {
    pub fn new() -> Debugger {
        let debugger = Debugger { step: 0 };
        debugger.listen();
        debugger
    }

    fn listen(&self) {
        thread::spawn(move || { listen("127.0.0.1:3012", |out| Websocket(out)).unwrap(); });
    }
}


impl Handler for Websocket {
    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        println!("{:?}", handle(&msg));
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("WebSocket closing for ({:?}) {}", code, reason);
    }
}


//TODO: Better error handling
fn handle(s: &Message) -> Result<Action, ()> {
    let msg = s.as_text().unwrap();
    let mut split = msg.split(",");

    match split.next().unwrap() {
        "Next" => Ok(Action::Next),
        "Step" => Ok(Action::Step(split.next().unwrap().parse::<u64>().unwrap())),
        "AddBreakPtn" => Ok(Action::AddBreakPtn(split.next().unwrap().parse::<u64>().unwrap())),
        "DelBreakPtn" => Ok(Action::DelBreakPtn(split.next().unwrap().parse::<u64>().unwrap())),
        _ => Err(())
    }
}
