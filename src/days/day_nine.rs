use std::collections::{HashSet, VecDeque};
use std::fs;
use std::io::BufRead;

pub fn day_nine() {
    let file = fs::File::open("resources/09/sample.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    let contents: Vec<Vec<_>> = lines
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|x| x.to_digit(10).unwrap() as u32)
                .collect()
        })
        .collect();

    // println!("Contents: {:?}", contents);

    println!("Part 1");

    let mut low_points = Vec::new();
    for (i, line) in contents.iter().enumerate() {
        for (j, x) in line.iter().enumerate() {
            let mut neighbors = Vec::new();
            if let Some(prev_line_idx) = i.checked_sub(1) {
                neighbors.push((
                    contents.get(prev_line_idx).unwrap().get(j).unwrap(),
                    i - 1,
                    j,
                ));
            }
            if let Some(next_line) = contents.get(i + 1) {
                neighbors.push((next_line.get(j).unwrap(), i + 1, j));
            }
            if let Some(prev_item_idx) = j.checked_sub(1) {
                neighbors.push((line.get(prev_item_idx).unwrap(), i, j - 1));
            }
            if let Some(next_item) = line.get(j + 1) {
                neighbors.push((next_item, i, j + 1));
            }
            if x < neighbors.iter().min_by(|x, y| x.0.cmp(y.0)).unwrap().0 {
                low_points.push((x, i, j));
            }
        }
    }

    // println!("\tLow Points: {:?}", low_points);
    println!(
        "\tAnswer: {:?}",
        low_points.iter().map(|(x, _, _)| *x + 1).sum::<u32>()
    );

    println!("Part 2");
    let mut basins = Vec::new();

    for sink in low_points {
        let mut basin_to_check = VecDeque::new();
        let mut basin = HashSet::new();

        basin_to_check.push_back(sink);
        while let Some((_, i, j)) = basin_to_check.pop_front() {
            let line = contents.get(i).unwrap();
            let mut neighbors = Vec::new();
            if let Some(prev_line_idx) = i.checked_sub(1) {
                neighbors.push((
                    contents.get(prev_line_idx).unwrap().get(j).unwrap(),
                    i - 1,
                    j,
                ));
            }
            if let Some(next_line) = contents.get(i + 1) {
                neighbors.push((next_line.get(j).unwrap(), i + 1, j));
            }
            if let Some(prev_item_idx) = j.checked_sub(1) {
                neighbors.push((line.get(prev_item_idx).unwrap(), i, j - 1));
            }
            if let Some(next_item) = line.get(j + 1) {
                neighbors.push((next_item, i, j + 1));
            }
            for y in neighbors {
                if (y.0 < &9) && !basin.contains(&y) {
                    basin.insert(y);
                    basin_to_check.push_back(y);
                }
            }
        }

        basins.push((sink, basin.len()));
    }

    // Reverse sorting
    basins.sort_by(|x, y| y.1.cmp(&x.1));

    // First 3 elements should be largest
    let answer = basins[0..3].iter().map(|x| x.1).product::<usize>();
    println!(
        "\tLargest basins: {:?}\n\tAnswer: {}",
        &basins[0..3],
        answer
    );
}
