pub struct Debugger {
    pub step: u64,
}

impl Debugger {
    pub fn new() -> Option<Debugger> {
        Some(Debugger { step: 0 })
    }
}
