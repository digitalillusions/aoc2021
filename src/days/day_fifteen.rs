use std::collections::HashMap;

pub fn day_fifteen() {
    let input = std::fs::read_to_string("resources/15/example1.txt").unwrap();

    let grid: Vec<_> = input
        .trim()
        .split("\n")
        .map(|line| {
            line.trim()
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut minimum_distance: HashMap<_, _> = grid
        .iter()
        .enumerate()
        .flat_map(|(i, line)| line.iter().enumerate().map(move |(j, _)| ((i, j), -1)))
        .collect();

    println!("{:?}", minimum_distance);
}
