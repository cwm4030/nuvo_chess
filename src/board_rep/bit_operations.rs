#[inline(always)]
pub fn set_bit(bitboard: u64, square: u8) -> u64 {
    bitboard | (1 << square)
}

#[inline(always)]
pub fn clear_bit(bitboard: u64, square: u8) -> u64 {
    (bitboard | (1 << square)) ^ (1 << square)
}

#[inline(always)]
pub fn toggle_bit(bitboard: u64, square: u8) -> u64 {
    bitboard ^ (1 << square)
}

#[inline(always)]
pub fn is_bit_set(bitboard: u64, square: u8) -> bool {
    (bitboard & (1 << square)) != 0
}

#[inline(always)]
pub fn count_bits(bitboard: u64) -> u32 {
    bitboard.count_ones()
}

#[inline(always)]
pub fn first_bit(bitboard: u64) -> u8 {
    bitboard.trailing_zeros() as u8
}

#[inline(always)]
pub fn last_bit(bitboard: u64) -> u8 {
    (63 - bitboard.leading_zeros()) as u8
}

#[inline(always)]
pub fn first_bit_pop(bitboard: &mut u64) -> u8 {
    let bit = first_bit(*bitboard);
    *bitboard = clear_bit(*bitboard, bit);
    bit
}

#[inline(always)]
pub fn last_bit_pop(bitboard: &mut u64) -> u8 {
    let bit = last_bit(*bitboard);
    *bitboard = clear_bit(*bitboard, bit);
    bit
}

pub fn print_bitboard(bitboard: u64) {
    for rank in 0..8 {
        for file in 0..8 {
            let square = rank * 8 + file;
            if is_bit_set(bitboard, square as u8) {
                print!("1 ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
    println!();
}
