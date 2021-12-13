use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead};

pub fn day_thirteen() {
    let file = fs::File::open("resources/13/sample.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut coords = HashSet::<(u32, u32)>::new();
    let mut folds = Vec::<(String, u32)>::new();
    for line in lines {
        let line = line.unwrap();
        if line.contains(",") {
            let xy: Vec<u32> = line
                .split(",")
                .map(|item| item.parse::<u32>().unwrap())
                .collect();
            coords.insert((xy[0], xy[1]));
        } else if line.contains("x=") {
            let fold: Vec<&str> = line.split("=").collect();
            folds.push((
                "x".to_string(),
                fold[fold.len() - 1].parse::<u32>().unwrap(),
            ));
        } else if line.contains("y=") {
            let fold: Vec<&str> = line.split("=").collect();
            folds.push((
                "y".to_string(),
                fold[fold.len() - 1].parse::<u32>().unwrap(),
            ));
        }
    }

    println!("Part 1");

    for (i, (axis, location)) in folds.into_iter().enumerate() {
        let mut temp_set = HashSet::<(u32, u32)>::new();
        if axis == "x" {
            for (x, y) in coords.drain() {
                if x > location {
                    let offset = x - location;
                    temp_set.insert((x - 2 * offset, y));
                } else {
                    temp_set.insert((x, y));
                }
            }
        } else if axis == "y" {
            for (x, y) in coords.drain() {
                if y > location {
                    let offset = y - location;
                    temp_set.insert((x, y - 2 * offset));
                } else {
                    temp_set.insert((x, y));
                }
            }
        }
        coords = temp_set;

        println!(
            "\tFold {} at {}: {}, # of coords: {}",
            axis,
            location,
            i,
            coords.len()
        );
    }

	println!("Part 2");
    let mut sorted_coords = Vec::from_iter(coords.iter().map(|(x, y)| (y, x)));
    sorted_coords.sort();
    let mut x_last = 0;
    let mut y_last = 0;
    let mut output = String::from("\t");

    for (x, y) in sorted_coords {
        if x > &x_last {
            for _ in 0..x - x_last {
                output += "\n\t";
            }
            x_last = *x;
            y_last = 0;
        }
        for _ in 1..y - y_last {
            output += ".";
        }
        y_last = *y;
        output += "#";
    }

    println!("\tPattern:\n{}", output);
}
