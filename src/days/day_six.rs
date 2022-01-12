use std::{collections::HashMap, fs};

pub fn day_six() {
    let file_str = fs::read_to_string("resources/06/sample.txt").unwrap();
    let initial_state: Vec<_> = file_str
        .trim()
        .split(",")
        .map(str::parse::<i64>)
        .collect::<Result<_, _>>()
        .unwrap();
    let child_map: HashMap<_, _> = (1..6)
        .map(|x| {
            println!("\tstart days {}", x);
            (x, recurse_lanternfish(x, 256))
        })
        .collect();
    let total_children = initial_state
        .iter()
        .map(|x| child_map.get(x).unwrap())
        .sum::<i64>();
    println!("Total children: {}", total_children);
}

fn recurse_lanternfish(current_day: i64, total_days: i64) -> i64 {
    (current_day + 1..=total_days)
        .step_by(6 + 1)
        .map(|i| {
            // println!("spawn new after {} days", i);
            recurse_lanternfish(i + 8, total_days)
        })
        .sum::<i64>()
        + 1
}
