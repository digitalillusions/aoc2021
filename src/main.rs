mod day_one;
mod day_three;
mod day_two;

use day_one::day_one;
use day_three::day_three;
use day_two::day_two;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(day) = args.get(1) {
        println!("Running day {}", day);
        match day.as_str() {
            "01" => day_one(),
            "02" => day_two(),
            "03" => day_three(),
            _ => println!("\tNot implemented yet"),
        };
    } else {
        println!(
            "Please specify the AOC day in the format of a two integer number. E.g. '01' or '22'."
        )
    }
}
