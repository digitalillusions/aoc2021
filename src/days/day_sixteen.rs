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
    let mut i = 0;
    while i < binary.len() - 7 {
        let version = u32::from_str_radix(&binary[i..i + 3], 2).unwrap();
        let typeid = u32::from_str_radix(&binary[i + 3..i + 6], 2).unwrap();

        println!("Version: {}, Type ID: {}", version, typeid);
        if typeid == 4 {
            let mut j = 0;
            let mut leading_digit = "1";
            let mut binary_value = String::new();
            while leading_digit == "1" {
                leading_digit = &binary[i + j + 6..i + j + 7];
                binary_value += &binary[i + j + 7..i + j + 11].to_string();
                j += 5;
            }
            i += j + 6;
            println!("Value: {}", binary_value);
            println!(
                "Value dec: {}",
                u32::from_str_radix(&binary_value, 2).unwrap()
            );
        } else {
        }
    }

    println!("Part 1");
    println!("Part 2");
}
