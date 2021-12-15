use std::collections::{HashMap, BTreeSet};
use std::fs;
use std::io::{self, BufRead};

pub fn day_eight(){
	let file = fs::File::open("resources/08/example1.txt").unwrap();
	let lines = io::BufReader::new(file).lines();

	let mut digits_values_map = Vec::new();

	for line in lines{
		let contents = line.unwrap().split(" | ").map(|item|{
			item.split(" ").map(|item|{
				item.chars().collect::<BTreeSet<_>>()
			}).collect::<Vec<_>>()
		}).collect::<Vec<_>>();

		digits_values_map.push((contents[0].clone(), contents[1].clone(), HashMap::new()));
	}

	for (digits, values, map) in digits_values_map.iter_mut(){
		let loc_one = digits.iter().position(|x| x.len() == 2).unwrap();
		let digit_one = digits.swap_remove(loc_one);
		map.insert(digit_one, 1);

		let loc_four = digits.iter().position(|x| x.len() == 4).unwrap();
		let digit_four = digits.swap_remove(loc_four);
		map.insert(digit_four, 4);

		let loc_seven = digits.iter().position(|x| x.len() == 3).unwrap();
		let digit_seven = digits.swap_remove(loc_seven);
		map.insert(digit_seven, 7);

		let loc_eight = digits.iter().position(|x| x.len() == 7).unwrap();
		let digit_eight = digits.swap_remove(loc_eight);
		map.insert(digit_eight, 8);

	}

	println!("Input {:?}", digits_values_map[0]);

}