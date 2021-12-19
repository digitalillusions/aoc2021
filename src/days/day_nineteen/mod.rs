mod vec3;

use std::fs;
use std::io::{self, BufRead};
use vec3::Vec3;

pub fn day_nineteen() {
    let file = fs::File::open("resources/19/example1.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut scanners = Vec::new();

    for line in lines {
        let line_string = line.unwrap();
        if line_string.contains("scanner") {
            scanners.push(Vec::new());
        } else if line_string.is_empty() {
            continue;
        } else {
            let xyz: Vec<i32> = line_string
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            scanners
                .last_mut()
                .unwrap()
                .push(Vec3::from([xyz[0], xyz[1], xyz[2]]));
        }
    }

    let mut scanner_coords: Vec<_> = scanners
        .iter()
        .enumerate()
        .map(|(i, _)| if i == 0 { Some((0, 0, 0)) } else { None })
        .collect();

    for (i, scanner) in scanners.iter().enumerate() {
        for (j, other_scanner) in scanners.iter().enumerate() {
            if i == j {
                continue;
            }
        }
    }

    /*
    Algorithm idea
    for all points in i
        for all possible rotations r
            for all rotated points in j
                compute translation from i to j
                recurse into remaining points in i and j

    Recursion
    for all points in i
        for all points in j
            compute difference of i and j
            if zero
                recurse without i and j


    */
    println!("Scanners {:?}", scanner_coords);
}

fn rotations(points: &Vec<[i32; 3]>) {
    let mut iterators = Vec::new();
    for i in 0..3 {
        let mut rot_axis = Vec3::zero();
        rot_axis[i] = 1;
        iterators.push(points.iter().map(|x| {
            let mut y = x;
            for j in 0..3 {}
        }))
    }
}
