use std::io;
use std::io::prelude::*;

pub fn prompt() -> String {
        print!("debug> ");
        let _ = io::stdout().flush();
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).unwrap();
        cmd.trim().into()
}

