use std::arch::x86_64::_pdep_u32;

pub const fn lsb(data: u32) -> u32 {
    data.trailing_zeros()
}

pub const fn msb(data: u32) -> u32 {
    31 - data.leading_zeros()
}

pub const fn pop_lsb(data: &mut u32) -> u32 {
    let lsb = lsb(*data);
    *data &= *data - 1;

    lsb
}

#[cfg(not(feature = "bmi2"))]
pub fn select_random_set_bit(mut data: u32) -> u32 {
    let bit = romu::mod_u32(data.count_ones());
    for _ in 0..bit {
        data &= data - 1;
    }

    lsb(data)
}

// why is this slower?
// see https://godbolt.org/z/bqjTo8TGE
// perhaps it's just zen2? https://xcancel.com/trav_downs/status/1202793097962364928#m
//
#[cfg(feature = "bmi2")]
pub fn select_random_set_bit(data: u32) -> u32 {
    let bit = romu::mod_u32(data.count_ones());
    let mask = 1 << bit;

    unsafe {
        let isolated = _pdep_u32(mask, data);
        isolated.trailing_zeros() as _
    }
}
