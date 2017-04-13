pub mod debugger;
use Cartridge;
use Cpu;
use Memory;

pub struct GameBoy {
    pub cpu: Box<Cpu>,
    pub cartridge: Box<Cartridge>,
    pub mem: Box<Memory>,
}


impl GameBoy {
    pub fn new(cpu: Box<Cpu>, cartridge: Box<Cartridge>, mem: Box<Memory>) -> Box<GameBoy> {
        Box::new(GameBoy {
            cpu: cpu,
            cartridge: cartridge,
            mem: mem,
        })
    }
}
