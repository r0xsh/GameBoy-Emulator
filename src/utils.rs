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

#[macro_export]
macro_rules! r {
    ($x: expr) => ({
        match $x {
            0 => (Some(Register8::B), None),
            1 => (Some(Register8::C), None),
            2 => (Some(Register8::D), None),
            3 => (Some(Register8::E), None),
            4 => (Some(Register8::H), None),
            5 => (Some(Register8::L), None),
            6 => (None, Some(Register16::HL)),
            7 => (Some(Register8::A), None),
            _ => unreachable!()
        }
    })
}


#[macro_export]
macro_rules! rp {
    ($x: expr) => ({
        match $x {
            0 => Register16::BC,
            1 => Register16::DE,
            2 => Register16::HL,
            3 => Register16::SP,
            _ => unreachable!()
        }
    })
}

#[macro_export]
macro_rules! rp2 {
    ($x: expr) => ({
        match $x {
            0 => Register16::BC,
            1 => Register16::DE,
            2 => Register16::HL,
            3 => Register16::AF,
            _ => unreachable!()
        }
    })
}

