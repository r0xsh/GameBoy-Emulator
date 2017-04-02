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
