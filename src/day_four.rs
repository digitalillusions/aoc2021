use std::fs;
use std::io::{self, BufRead};

pub fn day_four(){
	let file = fs::File::open("resources/04/example1.txt").unwrap();
	let mut lines = io::BufReader::new(file).lines();
	let number_sequence = lines.next().unwrap().unwrap().split(" ").map(|item| item.parse::<u32>().unwrap()).collect::<Vec<_>>();
	lines.next();

	let mut boards = Vec::<[[u32; 5]; 5]>::new();

	let mut counter = 0;
	for line in lines {
		let board_line = line.unwrap().trim().split(" ").filter(|&item| item != "" ).map(|item| item.parse::<u32>().unwrap()).collect::<Vec<u32>>();
	}

	println!("Part 1");
}