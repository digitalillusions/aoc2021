mod days;

use days::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(day) = args.get(1) {
        println!("Running day {}", day);
        match day.as_str() {
            "01" => day_one::day_one(),
            "02" => day_two::day_two(),
            "03" => day_three::day_three(),
            "04" => day_four::day_four(),
            "05" => day_five::day_five(),
            "06" => day_six::day_six(),
            "07" => day_seven::day_seven(),
            "08" => day_eight::day_eight(),
            "09" => day_nine::day_nine(),
            "10" => day_ten::day_ten(),
            "12" => day_twelve::day_twelve(),
            "13" => day_thirteen::day_thirteen(),
            "14" => day_fourteen::day_fourteen(),
            "16" => day_sixteen::day_sixteen(),
            "19" => day_nineteen::day_nineteen(),
            _ => println!("\tNot implemented yet"),
        };
    } else {
        println!(
            "Please specify the AOC day in the format of a two integer number. E.g. '01' or '22'."
        )
    }
}
