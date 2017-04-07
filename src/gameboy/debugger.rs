use std::io;
use std::io::prelude::*;

pub struct Debugger {
    pub step: u64,
}

impl Debugger {
    pub fn new() -> Option<Debugger> {
        Some(Debugger { step: 0 })
    }

    pub fn prompt(&mut self) -> String {
        print!("debug> ");
        let _ = io::stdout().flush();
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).unwrap();
        self.step += 1;
        cmd.trim().into()
    }
}
