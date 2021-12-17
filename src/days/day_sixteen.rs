use std::fs;
use std::io::{self, BufRead};

pub fn day_sixteen() {
    let file = fs::File::open("resources/16/example1.txt").unwrap();
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
    parse_message(&binary);

    println!("Part 1");
    println!("Part 2");
}

fn parse_message(current_message: &str) -> Option<u32> {
    let version = u32::from_str_radix(current_message.get(0..3)?, 2).ok()?;
    let typeid = u32::from_str_radix(current_message.get(3..6)?, 2).ok()?;
    let mut submessage = current_message.get(6..)?;

    match typeid {
        4 => {
            let mut binary_value = String::new();
            while submessage.get(0..1)? == "1" {
                binary_value += submessage.get(1..5)?;
                submessage = submessage.get(5..)?;
            }
            binary_value += submessage.get(1..5)?;
            let value = u32::from_str_radix(binary_value.as_str(), 2).ok()?;
            println!("Value: {}", value);
        }
        _ => println!("Do other"),
    }
    Some(0)
}
