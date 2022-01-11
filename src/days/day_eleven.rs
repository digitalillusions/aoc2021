use std::collections::{HashSet, VecDeque};
use std::fs;
use std::io::{self, BufRead};

pub fn day_eleven() {
    let file = fs::File::open("resources/11/sample.txt").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut dumbo_octopuses = lines
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|y| y.to_digit(10))
                .collect::<Option<Vec<_>>>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    println!("Part 1 & 2");

    let mut num_flashes = 0;
    for iter in 0..300 {
        let mut flashes = VecDeque::new();
        let mut already_flashed = HashSet::new();

        for (i, line) in dumbo_octopuses.iter_mut().enumerate() {
            for (j, energy) in line.iter_mut().enumerate() {
                *energy += 1;
                if energy > &mut 9 {
                    flashes.push_back((i, j));
                }
            }
        }

        while let Some((i, j)) = flashes.pop_front() {
            if already_flashed.contains(&(i, j)) {
                continue;
            }
            // println!("{}, {}", i, j);
            already_flashed.insert((i, j));
            num_flashes += 1;

            for k in 0..=2 {
                if let Some((n_i, Some(line))) = (i + k)
                    .checked_sub(1)
                    .and_then(|x| Some((x, dumbo_octopuses.get_mut(x))))
                {
                    for l in 0..=2 {
                        if let Some((n_j, Some(energy))) = (j + l)
                            .checked_sub(1)
                            .and_then(|y| Some((y, line.get_mut(y))))
                        {
                            // println!("\t{}, {}", n_i, n_j);
                            *energy += 1;
                            if energy > &mut 9 && !already_flashed.contains(&(n_i, n_j)) {
                                flashes.push_back((n_i, n_j));
                            }
                        }
                    }
                }
            }
        }

        for (i, j) in already_flashed.drain() {
            *dumbo_octopuses.get_mut(i).unwrap().get_mut(j).unwrap() = 0;
        }

        if dumbo_octopuses.iter().flatten().sum::<u32>() == 0 {
            println!("\tSynchronized flashing after: {} steps", iter + 1);
        }
    }

    println!("\tNumber of flashes: {}", num_flashes);
}
