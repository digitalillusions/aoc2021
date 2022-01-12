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
            "11" => day_eleven::day_eleven(),
            "12" => day_twelve::day_twelve(),
            "13" => day_thirteen::day_thirteen(),
            "14" => day_fourteen::day_fourteen(),
            "16" => day_sixteen::day_sixteen(),
            "19" => day_nineteen::day_nineteen(),
            "all" => run_all(),
            _ => println!("\tNot implemented yet"),
        };
    } else {
        println!(
            "Please specify the AOC day in the format of a two integer number. E.g. '01' or '22'."
        )
    }
}

fn run_all() {
    println!("Running day {}", 01);
    day_one::day_one();
    println!("Running day {}", 02);
    day_two::day_two();
    println!("Running day {}", 03);
    day_three::day_three();
    println!("Running day {}", 04);
    day_four::day_four();
    println!("Running day {}", 05);
    day_five::day_five();
    println!("Running day {}", 06);
    day_six::day_six();
    println!("Running day {}", 07);
    day_seven::day_seven();
    println!("Running day {}", 08);
    day_eight::day_eight();
    println!("Running day {}", 09);
    day_nine::day_nine();
    println!("Running day {}", 10);
    day_ten::day_ten();
    println!("Running day {}", 11);
    day_eleven::day_eleven();
    println!("Running day {}", 12);
    day_twelve::day_twelve();
    println!("Running day {}", 13);
    day_thirteen::day_thirteen();
    println!("Running day {}", 14);
    day_fourteen::day_fourteen();

    println!("Running day {}", 16);
    day_sixteen::day_sixteen();

    println!("Running day {}", 19);
    day_nineteen::day_nineteen();
}
