use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::io;
use std::fmt;

pub struct Cartridge(pub Vec<u8>);

impl Cartridge {

    /// Load rom file
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Cartridge> {
        let mut rom: Vec<u8> = Vec::new();
        let mut file = File::open(path)?;
        let _ = file.read_to_end(&mut rom)?;
        println!("ROM LOADED");
        Ok(Cartridge(rom))
    }

    /// Read a byte from the rom
    pub fn read_byte(&self, addr: usize) -> u8 {
        self.0[addr]
    }

    /// Read a range of bytes from the rom
    pub fn read_range(&self, addr: usize, end: usize) -> Vec<u8> {
        let mut a: Vec<u8> = Vec::new();
        for x in addr..end {
            a.push(self.0[x])
        }
        a
    }

    /// Read the rom's title
    pub fn read_title(&self) -> String {
        let mut title = String::with_capacity(16);
        for letter in self.read_range(0x0134, 0x0143) {
            if letter == 0 {
                break;
            }
            title.push(letter as char)
        }
        title
    }


}


impl fmt::Debug for Cartridge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "=== ROM DEBUG ===\n\
            > title game {}\n\
            > cartridge type: {:#x}\n\
            > ROM size: {:#x}\n\
            > RAM size: {:#x}",
            self.read_title(),
            self.read_byte(0x0147),
            self.read_byte(0x148),
            self.read_byte(0x0149))
    }
}

#[test]
fn get_name() {
    let rom = Cartridge::new("./rom/super_mario.gb").unwrap();
    assert_eq!(rom.read_title(), "SUPER MARIOLAND");
}


