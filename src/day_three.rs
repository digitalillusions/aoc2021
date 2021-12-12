use std::fs;
use std::io::{self, BufRead};

pub fn day_three() {
    let file = fs::File::open("resources/03/sample.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let bitcode: Vec<Vec<u32>> = lines
        .map(|line| {
            let chars_as_ints: Vec<u32> = line
                .unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect();
            chars_as_ints
        })
        .collect();

    println!("Part 1");
    let bit_counts = bitcode
        .clone()
        .into_iter()
        .reduce(|a, b| a.iter().zip(b).map(|(a, b)| a + b).collect())
        .unwrap();

    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    let half_len_data = bitcode.len() as f32 / 2.;
    for bit in bit_counts {
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

    println!(
        "\tGamma rate binary: {}\n\tEpsilon rate binary: {}",
        gamma_rate, epsilon_rate
    );
    println!(
        "\tGamma rate decimal: {}\n\tEpsilon rate decimal: {}",
        gamma_rate_dec, epsilon_rate_dec
    );
    println!("\tSolution Part 1: {}", gamma_rate_dec * &epsilon_rate_dec);

    println!("Part 2");
    let mut filter_oxygen = bitcode.clone();
    for idx in 0..bitcode[0].len() {
        if filter_oxygen.len() == 1 {
            break;
        }

        let count_ones: u32 = filter_oxygen.iter().map(|a| a[idx]).sum();
        let oxygen_generator: u32 = if (count_ones as f32) > (filter_oxygen.len() as f32 / 2.) {
            1
        } else if (count_ones as f32) < (filter_oxygen.len() as f32 / 2.) {
            0
        } else {
            1
        };

        filter_oxygen = filter_oxygen
            .into_iter()
            .filter_map(|item| {
                if item[idx] == oxygen_generator {
                    Some(item)
                } else {
                    None
                }
            })
            .collect();
    }

    let mut filter_co2 = bitcode.clone();
    for idx in 0..bitcode[0].len() {
        if filter_co2.len() == 1 {
            break;
        }

        let count_ones: u32 = filter_co2.iter().map(|a| a[idx]).sum();
        let co2_scrubber: u32 = if (count_ones as f32) > (filter_co2.len() as f32 / 2.) {
            0
        } else if (count_ones as f32) < (filter_co2.len() as f32 / 2.) {
            1
        } else {
            0
        };

        filter_co2 = filter_co2
            .into_iter()
            .filter_map(|item| {
                if item[idx] == co2_scrubber {
                    Some(item)
                } else {
                    None
                }
            })
            .collect();
    }

    let oxygen_generator_str: String = filter_oxygen[0].iter().map(|item| item.to_string()).collect::<Vec<String>>().join("");
    let co2_scrubber_str: String = filter_co2[0].iter().map(|item| item.to_string()).collect::<Vec<String>>().join("");

    let oxygen_generator_dec = u32::from_str_radix(&oxygen_generator_str, 2).unwrap();
    let co2_scrubber_dec = u32::from_str_radix(&co2_scrubber_str, 2).unwrap();

    println!("\tOxygen Generator: {}", oxygen_generator_str);
    println!("\tCO2 Scrubber: {}", co2_scrubber_str);
    println!("\tSolution Part 2: {}", oxygen_generator_dec * &co2_scrubber_dec);
}
