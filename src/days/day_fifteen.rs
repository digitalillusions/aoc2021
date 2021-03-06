use std::collections::HashMap;

pub fn day_fifteen() {
    let input = std::fs::read_to_string("resources/15/sample.txt").unwrap();

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

    println!("Part 1");
    find_shortest_path(&grid);

    let mut new_grid = Vec::new();

    for line in grid {
        new_grid.push(Vec::new());
        for i in 0..5 {
            new_grid.last_mut().unwrap().extend(line.iter().map(|x| {
                let new_val = (x + i) % 10;
                if new_val < *x {
                    new_val + 1
                } else {
                    new_val
                }
            }));
        }
    }

    let temp_grid = new_grid.clone();
    for i in 1..5 {
        for line in &temp_grid {
            new_grid.push(
                line.iter()
                    .map(|x| {
                        let new_val = (x + i) % 10;
                        if new_val < *x {
                            new_val + 1
                        } else {
                            new_val
                        }
                    })
                    .collect(),
            );
        }
    }

    println!("Part 2");
    find_shortest_path(&new_grid);
}

fn find_shortest_path(grid: &Vec<Vec<u32>>) {
    let mut next_node = Some((0 as usize, 0 as usize));
    let mut minimum_distance = HashMap::new();
    let mut active_set = HashMap::from([((0, 0), 0)]);
    let grid_len = grid.len() * grid.last().unwrap().len();

    while let Some(node) = next_node {
        minimum_distance.insert(node, active_set.remove(&node).unwrap());

        let cur_value = minimum_distance.get(&node).unwrap().clone();
        let mut neighbors = Vec::new();
        neighbors.push(node.0.checked_sub(1).map(|x| (x, node.1)));
        neighbors.push(node.0.checked_add(1).map(|x| (x, node.1)));
        neighbors.push(node.1.checked_sub(1).map(|y| (node.0, y)));
        neighbors.push(node.1.checked_add(1).map(|y| (node.0, y)));

        neighbors.iter().flatten().for_each(|n_node| {
            if let Some(neighbor_value) = grid.get(n_node.0).and_then(|line| line.get(n_node.1)) {
                let new_value = neighbor_value + cur_value.clone();
                if let Some(value) = active_set.get_mut(n_node) {
                    if new_value < *value {
                        *value = new_value;
                    }
                } else if !minimum_distance.contains_key(n_node) {
                    active_set.insert(*n_node, new_value);
                }
            }
        });

        next_node = active_set.iter().min_by(|x, y| x.1.cmp(y.1)).map(|x| *x.0);
        print!(
            "\r\tProgress {:.2}%",
            minimum_distance.len() as f32 / grid_len as f32 * 100.
        )
    }
    println!("");
    let lower_corner = (grid.len() - 1, grid.last().unwrap().len() - 1);
    println!(
        "\tDistance to lower right corner: {}",
        minimum_distance.get(&lower_corner).unwrap()
    );
}
