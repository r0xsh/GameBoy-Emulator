 extern crate websocket;

use std::thread;
use websocket::{Server, Message};
use websocket::message::Type;

pub fn run() {
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


