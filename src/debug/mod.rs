use gameboy::GameBoy;
use rustyline::Editor;
use std::fmt::format;
use std::collections::HashSet;

pub fn debug(gb: &GameBoy) {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => match line.as_str() {
                "cpu" => println!("{:?}", gb.cpu),
                "mem" => println!("{:?}", gb.mem),
                "exit" => std::process::exit(0x0),
                _ => {}
            },
            Err(_) => std::process::exit(0x1)
        }
    }
}