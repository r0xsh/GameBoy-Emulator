use Cpu;
use Cartridge;
use Memory;

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
