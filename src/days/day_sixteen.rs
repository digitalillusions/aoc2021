use std::fs;
use std::io::{self, BufRead};

pub fn day_sixteen() {
    let file = fs::File::open("resources/16/sample.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let hexcode = lines.next().unwrap().unwrap();
    let binary = hexcode
        .chars()
        .map(|x| format!("{:04b}", x.to_digit(16).unwrap()))
        .collect::<Vec<_>>()
        .join("");
    println!("Code {}", hexcode);
    println!("Binary {}", binary);

    println!("Part 1");
    let x = parse_message(&binary).unwrap();
    println!("\tSum of version numbers {}", x.1);

    println!("Part 2");
    println!("\tOutput value {}", x.2);
}

fn parse_message(current_message: &str) -> Option<(&str, u64, u64)> {
    let mut version = u64::from_str_radix(current_message.get(0..3)?, 2).ok()?;
    let typeid = u64::from_str_radix(current_message.get(3..6)?, 2).ok()?;
    let mut submessage = current_message.get(6..)?;
    let mut value = 0;
    let mut sub_values = Vec::new();
    println!("\tVersion: {}, Type ID: {}", version, typeid);

    // Part 1
    match typeid {
        4 => {
            let mut binary_value = String::new();
            while submessage.get(0..1)? == "1" {
                binary_value += submessage.get(1..5)?;
                submessage = submessage.get(5..)?;
            }
            binary_value += submessage.get(1..5)?;
            value = u64::from_str_radix(binary_value.as_str(), 2).ok()?;
            println!("\tValue: {}", value);
            return Some((submessage.get(5..)?, version, value));
        }
        _ => {
            let i = submessage.get(0..1)?;
            submessage = submessage.get(1..)?;
            match i {
                "0" => {
                    let subpacket_length = usize::from_str_radix(submessage.get(..15)?, 2).ok()?;
                    submessage = submessage.get(15..)?;
                    let mut subpacket = submessage.get(..subpacket_length)?;
                    println!("\tLength type ID: 0, Subpacket Length {}", subpacket_length);
                    while let Some((x, version_sum, value)) = parse_message(subpacket) {
                        version += version_sum;
                        subpacket = x;
                        sub_values.push(value);
                    }
                    submessage = submessage.get(subpacket_length..)?;
                }
                "1" => {
                    let num_subpackets = usize::from_str_radix(submessage.get(..11)?, 2).ok()?;
                    println!(
                        "\tLength type ID: 1, Number of Subpackets {}",
                        num_subpackets
                    );
                    submessage = submessage.get(11..)?;
                    for _ in 0..num_subpackets {
                        let x = parse_message(submessage)?;
                        submessage = x.0;
                        version += x.1;
                        sub_values.push(x.2);
                    }
                }
                _ => {}
            }
        }
    }

    // Part 2
    match typeid {
        0 => value = sub_values.iter().sum(),
        1 => value = sub_values.iter().product(),
        2 => value = *sub_values.iter().min()?,
        3 => value = *sub_values.iter().max()?,
        5 => value = (sub_values.get(0)? > sub_values.get(1)?) as u64,
        6 => value = (sub_values.get(0)? < sub_values.get(1)?) as u64,
        7 => value = (sub_values.get(0)? == sub_values.get(1)?) as u64,
        _ => {}
    }

    Some((submessage, version, value))
}
