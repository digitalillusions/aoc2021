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

    println!("Part 1");
    println!("Part 2");
}
