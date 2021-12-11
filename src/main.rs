use core::panic;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(day) = args.get(1) {
        println!("Running day {}", day);
        match day.as_str() {
            "01" => day_one(),
            _ => println!("\tNot implemented yet"),
        };
    } else {
        println!(
            "Please specify the AOC day in the format of a two integer number. E.g. '01' or '22'."
        )
    }
}

fn day_one() {
let contents: Vec<usize> = fs::read_to_string("resources/01/sample.txt")
	.unwrap()
	.lines()
	.map(|line| line.parse::<usize>().unwrap())
	.collect();

println!("Part 1");

if contents.len() < 2 {
	panic!("Contents must be longer than 1!")
}

let mut increases = 0;
let mut decreases = 0;
for (a, b) in contents[0..contents.len()-1].iter().zip(&contents[1..]){
	if a < b {
	increases += 1;
	} else if a > b {
	decreases += 1;
	}
}

println!("\tIncreases: {}, Decreases {}", increases, decreases);

println!("Part 2");

if contents.len() < 3 {
	panic!("Contents must be longer than 2!")
}

let mut increases = 0;
let mut decreases = 0;
let iters: Vec<_> = (0..4).map(|offset| &contents[offset..contents.len()-3+offset]).collect();
let iter_1 = iters[0].iter().zip(iters[1]).zip(iters[2]);
let iter_2 = iters[1].iter().zip(iters[2]).zip(iters[3]);

for (a, b) in iter_1.zip(iter_2){
	let ((a_1, a_2), a_3) = a;
	let ((b_1, b_2), b_3) = b;

	if (a_1 + a_2 + a_3) < (b_1 + b_2 + b_3){
	increases += 1;
	} else if (a_1 + a_2 + a_3) > (b_1 + b_2 + b_3){

	decreases += 1;
	}
}

println!("\tIncreases: {}, Decreases {}", increases, decreases);
}
