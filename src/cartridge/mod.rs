use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::io;
use std::fmt;

const TITLE: (u16, u16) = (0x0134, 0x0143);
const CARTRIDGE_TYPE: u16 = 0x0147;
const ROM_SIZE: u16 = 0x0148;
const RAM_SIZE: u16 = 0x0149;

pub struct Cartridge(Vec<u8>);

impl Cartridge {

    /// Load rom file
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Cartridge> {
        let mut rom: Vec<u8> = Vec::new();
        let mut file = File::open(path)?;
        let _ = file.read_to_end(&mut rom)?;
        Ok(Cartridge(rom))
    }

    /// Read a byte from the rom
    pub fn read_byte(&self, addr: u16) -> u8 {
        self.0[addr as usize]
    }

    /// Read a range of bytes from the rom
    pub fn read_range(&self, addr: (u16, u16)) -> Vec<u8> {
        let mut a: Vec<u8> = Vec::new();
        for x in addr.0..addr.1 {
            a.push(self.0[x as usize])
        }
        a
    }

    /// Read the rom's title
    pub fn read_title(&self) -> String {
        let mut title = String::with_capacity(16);
        for letter in self.read_range(TITLE) {
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
            self.read_byte(CARTRIDGE_TYPE),
            self.read_byte(ROM_SIZE),
            self.read_byte(RAM_SIZE))
    }
}
