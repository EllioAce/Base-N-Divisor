use crate::{
    base_bit::BASE_BITS_LENGTH,
    divisor::{divide_base_bit_number_array, number_to_bit_list},
};
use std::{env, time::Instant};

mod base_bit;
mod divisor;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let base: u8 = 2;
    let number: i32 = 6;
    let converted_bits: [u8; BASE_BITS_LENGTH] = number_to_bit_list(number, base);

    let now_normal = Instant::now();
    let normal_result = normal_divison(number, base);
    let elapsed_normal = now_normal.elapsed();
    println!("Divided normally in {}s", elapsed_normal.as_secs_f32());

    let now_base = Instant::now();
    let base_result = base_divison(converted_bits, base);
    let elapsed_base = now_base.elapsed();
    println!(
        "Divided using base division in {}s",
        elapsed_base.as_secs_f32()
    );

    println!("Normal division result: {}", normal_result);
    println!("Base division result: {}", base_result);

    let execution_ratio = elapsed_normal.as_secs_f32() / elapsed_base.as_secs_f32();
    println!(
        "Base division was {} times faster than normal division",
        execution_ratio
    );
}

fn base_divison(bits: [u8; BASE_BITS_LENGTH], denominator: u8) -> i32 {
    let division: i32 = divide_base_bit_number_array(bits, denominator);
    return division;
}

fn normal_divison(number: i32, denominator: u8) -> i32 {
    let result: i32 = number / denominator as i32;
    return result;
}
