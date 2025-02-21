//use std::arch::x86_64::_pdep_u64;

pub const fn lsb(data: u64) -> u64 {
    data.trailing_zeros() as u64
}

pub const fn msb(data: u64) -> u64 {
    63 - data.leading_zeros() as u64
}

pub const fn pop_lsb(data: &mut u64) -> u64 {
    let lsb = lsb(*data);
    *data &= *data - 1;

    lsb
}

pub fn select_random_set_bit(mut data: u64) -> u64 {
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
//#[cfg(target_feature = "bmi2")]
//pub fn select_random_set_bit(data: u64) -> u64 {
//    let bit = romu::mod_u32(data.count_ones());
//    let mask = 1 << bit;
//
//    unsafe {
//        let isolated = _pdep_u64(mask, data);
//        isolated.trailing_zeros() as _
//    }
//}
