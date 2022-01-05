use std::{collections::HashMap, fs};

pub fn day_six() {
    let file_str = fs::read_to_string("resources/06/example1.txt").unwrap();
    let initial_state: Vec<_> = file_str
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let child_map: HashMap<_, _> = (1..6)
        .map(|x| {
            println!("\nstart days {}", x);
            (x, recurse_lanternfish(x, 18))
        })
        .collect();
    let total_children = initial_state
        .iter()
        .map(|x| child_map.get(x).unwrap())
        .sum::<i32>();
    println!("{:?}", total_children);
    println!("{:?}", child_map);
}

fn recurse_lanternfish(internal_timer: i32, remaining_days: i32) -> i32 {
    (0..=remaining_days - internal_timer)
        .step_by(6 + 1)
        //.rev()
        .map(|i| {
            println!("8 at {} days", i);
            recurse_lanternfish(8, i)
        })
        .sum::<i32>()
        + 1
}
