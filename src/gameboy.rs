use Cpu;
use Cartridge;
use Memory;

#[macro_export]
macro_rules! high_byte {
    ($x:expr) => { ($x >> 8) as u8 };
}

#[macro_export]
macro_rules! low_byte {
    ($x:expr) => { $x as u8 };
}

#[macro_export]
macro_rules! join_bytes {
    ($a:expr, $b:expr) => ({
        let mut join: u16 = $b as u16;
        join |= ($a as u16) << 8;
        join
    })
}

pub struct GameBoy<'a> {
    pub cpu: &'a mut Cpu,
    pub cartridge: &'a Cartridge,
    pub mem: &'a mut Memory,
}

impl<'a> GameBoy<'a> {
    pub fn new(cpu: &'a mut Cpu, cartridge: &'a Cartridge, mem: &'a mut Memory) -> GameBoy<'a> {
        GameBoy {
            cpu: cpu,
            cartridge: cartridge,
            mem: mem,
        }
    }
}
