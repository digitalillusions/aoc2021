use std::fs;
use std::io::{self, BufRead};

pub fn day_sixteen() {
    let file = fs::File::open("resources/16/example7.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let hexcode = lines.next().unwrap().unwrap();
    let binary = hexcode
        .chars()
        .map(|x| format!("{:04b}", x.to_digit(16).unwrap()))
        .collect::<Vec<_>>()
        .join("");
    println!("Code {}", hexcode);
    println!("Binary {}", binary);

    // iterate only while there is still at least a version (3 bits) and packet id (3 bits) and at least one more bit remaining

    println!("Part 1");
    let x = parse_message(&binary).unwrap();
    println!("\tSum of version numbers {}", x.1);

    println!("Part 2");
}

fn parse_message(current_message: &str) -> Option<(&str, u32)> {
    let mut version = u32::from_str_radix(current_message.get(0..3)?, 2).ok()?;
    let typeid = u32::from_str_radix(current_message.get(3..6)?, 2).ok()?;
    let mut submessage = current_message.get(6..)?;
    println!("\tVersion: {}, Type ID: {}", version, typeid);

    match typeid {
        4 => {
            let mut binary_value = String::new();
            while submessage.get(0..1)? == "1" {
                binary_value += submessage.get(1..5)?;
                submessage = submessage.get(5..)?;
            }
            binary_value += submessage.get(1..5)?;
            let value = u32::from_str_radix(binary_value.as_str(), 2).ok()?;
            println!("\tValue: {}", value);
            return Some((submessage.get(5..)?, version));
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
                    while let Some((x, version_sum)) = parse_message(subpacket) {
                        version += version_sum;
                        subpacket = x;
                    }
                    return Some((submessage.get(subpacket_length..)?, version));
                }
                "1" => {
                    let num_subpackets = usize::from_str_radix(submessage.get(..11)?, 2).ok()?;
                    println!(
                        "\tLength type ID: 1, Number of Subpackets {}",
                        num_subpackets
                    );
                    submessage = submessage.get(11..)?;
                    for i in 0..num_subpackets {
                        let x = parse_message(submessage)?;
                        submessage = x.0;
                        version += x.1;
                    }
                    return Some((submessage, version));
                }
                _ => {}
            }
        }
    }

    Some((submessage, version))
}
