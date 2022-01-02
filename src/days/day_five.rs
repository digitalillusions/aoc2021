use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};

pub fn day_five() {
    let file = fs::File::open("resources/05/sample.txt").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut start_end = Vec::new();
    for line in lines {
        let start_end_coords: Vec<_> = line
            .unwrap()
            .split(" -> ")
            .map(|x| x.split(","))
            .flatten()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        start_end.push((
            (start_end_coords[0], start_end_coords[1]),
            (start_end_coords[2], start_end_coords[3]),
        ));
    }
    // println!("{:?}", start_end);
    println!("Part 1&2");

    let mut straight_line_counts = HashMap::new();
    let mut line_counts = HashMap::new();
    for (start, end) in start_end {
        if (start.0 == end.0) || (start.1 == end.1) {
            let x0 = start.0.min(end.0);
            let x1 = start.0.max(end.0) + 1;
            for i in x0..x1 {
                let y0 = start.1.min(end.1);
                let y1 = start.1.max(end.1) + 1;
                for j in y0..y1 {
                    straight_line_counts
                        .entry((i, j))
                        .and_modify(|x| *x += 1)
                        .or_insert(1 as u32);
                    line_counts
                        .entry((i, j))
                        .and_modify(|x| *x += 1)
                        .or_insert(1 as u32);
                }
            }
        } else {
            let rangex: Vec<_> = if start.0 < end.0 {
                (start.0.min(end.0)..=start.0.max(end.0)).collect()
            } else {
                (start.0.min(end.0)..=start.0.max(end.0)).rev().collect()
            };
            let rangey: Vec<_> = if start.1 < end.1 {
                (start.1.min(end.1)..=start.1.max(end.1)).collect()
            } else {
                (start.1.min(end.1)..=start.1.max(end.1)).rev().collect()
            };

            for (&i, &j) in rangex.iter().zip(rangey.iter()) {
                line_counts
                    .entry((i, j))
                    .and_modify(|x| *x += 1)
                    .or_insert(1 as u32);
            }
        }
    }

    // println!("\t{:?}", straight_line_counts);
    println!(
        "\tStraight line counts >1: {:?}",
        straight_line_counts
            .iter()
            .filter(|(_, value)| **value > 1)
            .count()
    );

    println!(
        "\tAll line counts >1: {:?}",
        line_counts.iter().filter(|(_, value)| **value > 1).count()
    );
}
