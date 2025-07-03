use core::mem;

const BITS_PER_BYTE: u8 = 8;

const fn BITS_PER_TYPE<T>() -> usize {
    mem::size_of::<T>() * BITS_PER_BYTE as usize
}

pub const BITS_PER_USIZE: usize = mem::size_of::<usize>() * BITS_PER_BYTE as usize;

const fn div_round_up(n: usize, d: usize) -> usize {
    (n + d - 1) / d
}

pub const fn bits_to_usize(nr: usize) -> usize {
    div_round_up(nr, BITS_PER_TYPE::<usize>())
}

const fn bits_to_u64(nr: usize) -> usize {
    div_round_up(nr, BITS_PER_TYPE::<u64>())
}

const fn bits_to_u32(nr: usize) -> usize {
    div_round_up(nr, BITS_PER_TYPE::<u32>())
}

const fn bits_to_bytes(nr: usize) -> usize {
    div_round_up(nr, BITS_PER_TYPE::<u8>())
}
const fn BYTES_TO_BITS(nb: usize) -> usize {
    nb * BITS_PER_BYTE as usize
}
