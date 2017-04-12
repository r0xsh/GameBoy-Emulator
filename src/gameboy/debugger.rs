use ws;
use ws::{CloseCode, Handler, Message , Sender, listen};
use std::thread;

#[derive(Debug, PartialEq)]
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
        println!("{:?}", handle(msg.as_text().unwrap()));
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("WebSocket closing for ({:?}) {}", code, reason);
    }
}


//TODO: Better error handling
fn handle(s: &str) -> Result<Action, ()> {
    let mut split = s.split(",");

    match split.next().unwrap() {
        "Next" => Ok(Action::Next),
        "Step" => Ok(Action::Step(split.next().unwrap().parse::<u64>().unwrap())),
        "AddBreakPtn" => Ok(Action::AddBreakPtn(split.next().unwrap().parse::<u64>().unwrap())),
        "DelBreakPtn" => Ok(Action::DelBreakPtn(split.next().unwrap().parse::<u64>().unwrap())),
        _ => Err(())
    }
}

#[test]
fn handle_cmd() {
    assert_eq!(handle("Next"), Ok(Action::Next));
    assert_eq!(handle("Step,45"), Ok(Action::Step(45)));
    assert_eq!(handle("AddBreakPtn,678"), Ok(Action::AddBreakPtn(678)));
    assert_eq!(handle("DelBreakPtn,654"), Ok(Action::DelBreakPtn(654)));

    assert_eq!(handle("Foo"), Err(()));
    assert_eq!(handle("Step,-45"), Err(()));
    assert_eq!(handle("AddBreakPtn"), Err(()));
    assert_eq!(handle("DelBreakPtn,0"), Err(()));

}
