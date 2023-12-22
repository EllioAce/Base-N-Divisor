use crate::base_bit::{BaseBit, BASE_BITS_LENGTH};

pub fn base_to_number(numbers: BaseBit) -> i32 {
    let mut result: u32 = 0;
    let base: u32 = numbers.base as u32;
    let bits: [u8; BASE_BITS_LENGTH] = numbers.bits;
    let mut exponential_base: u32 = 1;
    for (i, &bit) in bits.iter().rev().enumerate() {
        if bit == 0 {
            continue;
        }
        result += exponential_base;
        exponential_base *= base;
    }
    return result as i32;
}

pub fn number_to_bit_list(number: i32, base: u8) -> [u8; BASE_BITS_LENGTH] {
    let mut decimal: i32 = number;
    let mut result: [u8; BASE_BITS_LENGTH] = [0; BASE_BITS_LENGTH];

    let mut index: usize = 0;

    while decimal > 0 {
        let remainder: u8 = (decimal as u8 % base) as u8;
        result[index] = remainder;
        // This division is used out of necessity, I will see if I can remove it in the future
        decimal /= base as i32;
        index += 1;
    }

    result.reverse();
    return result;
}

pub fn push_base_bit_back(base_bit: BaseBit) -> BaseBit {
    let base: u8 = base_bit.base;
    let mut bits: [u8; BASE_BITS_LENGTH] = base_bit.bits;
    bits.rotate_right(1);
    return BaseBit::new(bits, base);
}

pub fn divide_base_bit_number_array(bits: [u8; BASE_BITS_LENGTH], base: u8) -> i32 {
    let base_bits: BaseBit = BaseBit::new(bits, base);
    let pushed_bits: BaseBit = push_base_bit_back(base_bits);
    let result: i32 = base_to_number(pushed_bits);
    return result;
}
