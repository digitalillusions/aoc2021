mod day_one;

use std::env;
use day_one::day_one;

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
