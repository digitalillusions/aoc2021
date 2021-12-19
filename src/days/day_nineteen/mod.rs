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
            let rotations_j = rotations(other_scanner);
            println!("Number of rotations {}", rotations_j.len());
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
}

fn rotations(points: &Vec<Vec3>) -> Vec<Vec<Vec3>> {
    let mut rotated_points = Vec::new();
    // All possible combinations of 2 axes
    for i in 0..3 {
        for j in 0..3 {
            if i == j {
                continue;
            }
            // With all combinations of orientations, the third axis is computed using th right hand rule from a cross product
            for k in [-1, 1] {
                for l in [-1, 1] {
                    let mut rot_axis = Vec3::zero();
                    rot_axis[i] = k;
                    let mut second_axis = Vec3::zero();
                    second_axis[j] = l;
                    rotated_points.push(
                        points
                            .iter()
                            .map(|x| x.rotate(rot_axis.clone(), second_axis.clone()))
                            .collect::<Vec<_>>(),
                    );
                }
            }
        }
    }

    rotated_points
}
