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
    let now = Instant::now();
    print_normal_divison(number, base);
    let mut since_division: f32 = now.elapsed().as_secs_f32();
    println!("Divided normally in {}s", since_division);
    since_division = now.elapsed().as_secs_f32() - since_division;
    print_base_divison(converted_bits, base);
    since_division = now.elapsed().as_secs_f32() - since_division;
    println!("Divided using base division in {}s", since_division);
    println!("{}", now.elapsed().as_secs_f32());
}

fn print_base_divison(bits: [u8; BASE_BITS_LENGTH], denominator: u8) {
    let division = divide_base_bit_number_array(bits, denominator);
    println!("{}", division);
}

fn print_normal_divison(number: i32, denominator: u8) {
    println!("{}", number / denominator as i32);
}
