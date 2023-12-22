use crate::base_bit::{BaseBit, BASE_BITS_LENGTH};

pub fn base_to_number(numbers: BaseBit) -> i32 {
    let mut result: i32 = 0;
    let base: u32 = numbers.base as u32;
    let bits: [u8; BASE_BITS_LENGTH] = numbers.bits;
    let length_bits: i8 = BASE_BITS_LENGTH as i8 - 1;
    let mut current_power: u32 = 0;
    for i in 0..length_bits {
        let index: usize = (i * -1 + length_bits).abs() as usize;
        let bit: u8 = bits[index];
        let base_power: u32 = base.pow(current_power);
        let bit_power: u32 = base_power * (bit as u32);
        result += bit_power as i32;
        current_power += 1;
    }
    return result;
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
