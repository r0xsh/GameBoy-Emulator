
use std::thread;
use ws;
use ws::{CloseCode, Handler, Message, Sender, listen};

#[derive(Debug, PartialEq)]
enum Action {
    Next,
    Step(u64),
    AddBreakPtn(u64),
    DelBreakPtn(u64),
    Err,
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

    let cmd: &str = match split.next() {
        Some(e) => e,
        None => return Err(()),
    };

    let opt: u64 = match (split.next(), cmd) {
        (None, "Next") => 0,
        (Some(e), _) => {
            match e.parse::<u64>() {
                Ok(u) => match u {
                    0 => return Err(()),
                    _ => u
                },
                Err(_) => return Err(()),
            }
        }
        (None, _) => return Err(()),
    };

    match cmd {
        "Next" => Ok(Action::Next),
        "Step" => Ok(Action::Step(opt)),
        "AddBreakPtn" => Ok(Action::AddBreakPtn(opt)),
        "DelBreakPtn" => Ok(Action::DelBreakPtn(opt)),
        _ => Err(()),
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
