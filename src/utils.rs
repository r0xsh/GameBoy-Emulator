use bitlab::InsertIntoSizedIntegerTypes;
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

#[macro_export]
macro_rules! reverse_endian {
    ($x: expr) => ({
        let low = low_byte!($x);
        let high = high_byte!($x);
        join_bytes!(high, low)
    })
}


#[macro_export]
macro_rules! cc {
    ($x: expr) => ({
        match $x {
            0 => Flag::NZ,
            1 => Flag::Z,
            2 => Flag::NC,
            3 => Flag::C,
            _ => unreachable!()
        }
    })
}

pub fn get_opcode_from_small(x: u8, y: u8, z: u8, p: Option<u8>, q: Option<u8>) -> u8 {
    let mut opcode: u8 = 0;
    opcode = opcode.set(0, 2, x).unwrap();
    opcode = opcode.set(2, 3, y).unwrap();
    opcode = opcode.set(5, 3, z).unwrap();

    if p.is_some() && q.is_some() {
        opcode = opcode.set(5, 2, p.unwrap()).unwrap();
        opcode = opcode.set(7, 1, q.unwrap()).unwrap();
    }

    opcode
}

#[test]
fn test_get_opcode_from_small() {
    assert_eq!(get_opcode_from_small(0, 3, 0, None, None), 0x18);
}