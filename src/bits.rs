pub fn lsb(data: u64) -> u64 {
    data.trailing_zeros() as u64
}

pub fn msb(data: u64) -> u64 {
    63 - data.leading_zeros() as u64
}

pub fn pop_lsb(data: &mut u64) -> u64 {
    let lsb = lsb(*data);
    *data &= *data - 1;

    lsb
}
