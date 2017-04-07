extern crate websocket;

use std::thread;
use websocket::{Server, Message};
use websocket::message::Type;
use std::sync::Arc;
use std::sync::Mutex;

use GameBoy;

struct DebuggerInner<'a>(&'a GameBoy<'a>);

pub struct Debugger<'a> {
    inner: Arc<Mutex<DebuggerInner<'a>>>
}

impl<'a> Debugger<'a> {


    pub fn new(gb: &'a GameBoy) -> Debugger<'a> {
        Debugger {
            inner: Arc::new(Mutex::new(DebuggerInner(gb)))
        }
    }

    pub fn run(&self) {
        thread::spawn(move || {
        let server = Server::bind("127.0.0.1:3012").unwrap();

        for request in server.filter_map(Result::ok) {
                let mut client = request.use_protocol("rust-debugger").accept().unwrap();



                let message: Message = Message::text("Hello".to_string());
                client.send_message(&message).unwrap();

                let (mut receiver, mut sender) = client.split().unwrap();

                for message in receiver.incoming_messages() {
                    let message: Message = message.unwrap();

                    match message.opcode {
                        Type::Close => {
                            let message = Message::close();
                            sender.send_message(&message).unwrap();
                            return;
                        }
                        _ => sender.send_message(&message).unwrap(),
                    }
                }
        }
        });
    }

    pub fn oui(&self) {
        let a = self.clone();
        let l = a.inner.lock().unwrap();
        println!("{:?}", l.0.cpu);
    }

}

impl<'a> DebuggerInner<'a> {

    fn test(&self) -> &GameBoy {
        self.0
    }

}
