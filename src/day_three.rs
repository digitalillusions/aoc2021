use std::fs;
use std::io::{self, BufRead};

pub fn day_three() {
    let file = fs::File::open("resources/03/sample.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let bitcode = lines
        .map(|line| {
            let mut chars_as_ints: Vec<u32> = line.unwrap().chars().map(|c| c.to_digit(10).unwrap()).collect();
            chars_as_ints.push(1);
            chars_as_ints
        })
        .reduce(|a, b| a.iter().zip(b).map(|(a, b)| a + b).collect())
        .unwrap();

    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    println!("Bitcode {:?}", bitcode);
    let (bit_counts, len_data) = (&bitcode[..bitcode.len()-1], bitcode[bitcode.len()-1]);
    let half_len_data = len_data as f32/2.;
    for &bit in bit_counts{
        if bit as f32 >= half_len_data {
            gamma_rate += "1";
            epsilon_rate += "0";
        } else {
            gamma_rate += "0";
            epsilon_rate += "1";
        }
    }

    let gamma_rate_dec = u32::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate_dec = u32::from_str_radix(&epsilon_rate, 2).unwrap();

    println!("Gamma rate: {}\nEpsilon rate: {}", gamma_rate, epsilon_rate);
    println!("Gamma rate: {}\nEpsilon rate: {}", gamma_rate_dec, epsilon_rate_dec);
    println!("Solution Part 1: {}", gamma_rate_dec*&epsilon_rate_dec);
}
