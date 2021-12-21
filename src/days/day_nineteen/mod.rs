mod vec3;

use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, BufRead};
use vec3::Vec3;

pub fn day_nineteen() {
    let file = fs::File::open("resources/19/sample.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut scanners = Vec::new();

    for line in lines {
        let line_string = line.unwrap();
        if line_string.contains("scanner") {
            scanners.push(HashSet::new());
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
                .insert(Vec3::from([xyz[0], xyz[1], xyz[2]]));
        }
    }

    let _scanner_coords: Vec<_> = scanners
        .iter()
        .enumerate()
        .map(|(i, _)| if i == 0 { Some(Vec3::zero()) } else { None })
        .collect();

    // let mut count_matching_beacons = 0;
    // let mut total_readings = 0;
    // for (i, scanner) in scanners.iter().enumerate() {
    //     total_readings += scanner.len();
    //     for (other_scanner, j) in scanners
    //         .get(i + 1..scanners.len())
    //         .unwrap()
    //         .iter()
    //         .zip(i + 1..scanners.len())
    //     {
    //         if let Some(num_match) = check_match_ij(scanner, other_scanner, i, j) {
    //             count_matching_beacons += num_match;
    //         }
    //     }
    // }

    let scanner_indices: HashSet<_> = (0..scanners.len()).collect();
    let mut total_beacons = HashSet::new();
    let mut already_known = HashSet::new();
    let mut check_next = HashSet::new();
    let mut scanner_coords = HashMap::new();

    total_beacons.extend(scanners[0].clone());
    already_known.insert(0);
    check_next.insert(0);
    scanner_coords.insert(0 as usize, Vec3::zero());

    while !check_next.is_empty() {
        let mut newly_found = HashSet::new();
        for i in check_next {
            for &j in scanner_indices.difference(&already_known) {
                println!("Comparing scanner {} and scanner {}", i, j);
                if let Some((beacons, offset_ij)) = check_match_ij(&scanners[i], &scanners[j]) {
                    println!(
                        "Found match between scanner {} and scanner {} with offset {:?}",
                        i, j, offset_ij
                    );
                    scanner_coords.insert(j, Vec3::zero() - offset_ij);
                    newly_found.insert(j);
                    total_beacons.extend(beacons.clone());
                    scanners[j] = beacons;
                }
            }
        }
        already_known.extend(newly_found.clone());
        check_next = newly_found;
    }

    println!("Already known {:?}", already_known);
    println!("Scanner Coords {:?}", scanner_coords);

    println!("Part 1");
    println!("\tTotal number of beacons {:?}", total_beacons.len());

    println!("Part 2");
    let mut manhattan_distance = 0;
    for i in scanner_coords.values() {
        for j in scanner_coords.values() {
            if (i - j).abs().sum() > manhattan_distance {
                manhattan_distance = (i - j).abs().sum();
            }
        }
    }
    println!("\tManhattan distance {:?}", manhattan_distance);

    // println!("Total number of readings {}\nNumber of beacons counted twice {}\nTotal number of beacons {}", total_readings, count_matching_beacons, total_readings-count_matching_beacons);
}

fn check_match_ij(
    scanner: &HashSet<Vec3>,
    other_scanner: &HashSet<Vec3>,
) -> Option<(HashSet<Vec3>, Vec3)> {
    let rotations_j = rotations(other_scanner);
    for other_rotated_scanner in rotations_j {
        for point_i in scanner {
            for point_j in other_rotated_scanner.iter() {
                let offset_ij = point_j - point_i;
                let count_matching_beacons = other_rotated_scanner
                    .iter()
                    .filter(|x_j| {
                        scanner
                            .iter()
                            .map(|&x_i| (x_i + offset_ij.clone() - *x_j.clone()).abs())
                            .min()
                            .unwrap()
                            == Vec3::zero()
                    })
                    .count();
                if count_matching_beacons >= 12 {
                    let transformed_beacons: HashSet<_> = other_rotated_scanner
                        .iter()
                        .map(|&x| x - offset_ij.clone())
                        .collect();
                    return Some((transformed_beacons, offset_ij));
                }
            }
        }
    }

    return None;
}

fn rotations(points: &HashSet<Vec3>) -> Vec<HashSet<Vec3>> {
    let mut rotated_points = Vec::new();
    // println!("Original {:?}", points[0]);
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
                            .collect::<HashSet<_>>(),
                    );
                    // println!(
                    //     "Axis 1 {:?}\nAxis 2 {:?}\nAxis 3 {:?}",
                    //     rot_axis,
                    //     second_axis,
                    //     rot_axis.cross(second_axis)
                    // );
                    // println!(
                    //     "{}, {:?}\n",
                    //     rotated_points.len(),
                    //     rotated_points.last().unwrap()[0]
                    // );
                }
            }
        }
    }

    rotated_points
}
