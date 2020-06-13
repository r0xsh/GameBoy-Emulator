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

