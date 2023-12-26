use crate::base_bit::{BaseBit, BASE_BITS_LENGTH};

pub fn base_to_number(numbers: &BaseBit) -> i32 {
    let mut result: u32 = 0;
    let mut exponential_base: u32 = 1;
    for &bit in numbers.bits.iter().rev() {
        if bit == 0 {
            exponential_base *= numbers.base as u32;
            continue;
        }
        result += exponential_base * bit as u32;
        exponential_base *= numbers.base as u32;
    }
    return result as i32;
}

pub fn number_to_bit_list(number: i32, base: u8) -> [u8; BASE_BITS_LENGTH] {
    let mut decimal: i32 = number;
    let mut result: [u8; BASE_BITS_LENGTH] = [0; BASE_BITS_LENGTH];

    for index in 0..BASE_BITS_LENGTH {
        if decimal <= 0 {
            break;
        }
        let remainder: u8 = (decimal as u8 % base) as u8;
        result[index] = remainder;
        // This division is used out of necessity, I will see if I can remove it in the future
        decimal /= base as i32;
    }

    result.reverse();
    return result;
}

pub fn push_base_bit_back(base_bit: &mut BaseBit) {
    base_bit.bits.rotate_right(1);
}

pub fn divide_base_bit_number_array(bits: [u8; BASE_BITS_LENGTH], base: u8) -> i32 {
    let mut pushed_bits: BaseBit = BaseBit::new(bits, base);
    push_base_bit_back(&mut pushed_bits);
    let result: i32 = base_to_number(&pushed_bits);
    return result;
}
