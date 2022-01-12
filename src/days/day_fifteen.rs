use std::collections::{HashMap, HashSet};

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

    let mut minimum_distance: HashMap<_, _> = grid
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(move |(j, _)| ((i, j), u32::MAX))
        })
        .collect();

    let mut next_node = Some((0 as usize, 0 as usize));
    let mut already_visited = HashSet::new();

    // Init minimum distance for starting node to 0
    *minimum_distance.get_mut(&next_node.unwrap()).unwrap() = 0;
    while let Some(node) = next_node {
        already_visited.insert(node);
        let cur_value = minimum_distance.get(&node).unwrap().clone();
        let mut neighbors = Vec::new();
        neighbors.push(node.0.checked_sub(1).map(|x| (x, node.1)));
        neighbors.push(node.0.checked_add(1).map(|x| (x, node.1)));
        neighbors.push(node.1.checked_sub(1).map(|y| (node.0, y)));
        neighbors.push(node.1.checked_add(1).map(|y| (node.0, y)));

        neighbors.iter().flatten().for_each(|n_node| {
            if let Some(value) = minimum_distance.get_mut(&n_node) {
                let new_value =
                    grid.get(n_node.0).unwrap().get(n_node.1).unwrap() + cur_value.clone();
                if *value > new_value {
                    *value = new_value;
                    already_visited.remove(n_node);
                }
            }
        });

        next_node = minimum_distance
            .iter()
            .filter(|(x, _)| !already_visited.contains(&x))
            .min()
            .map(|x| *x.0);
    }
    println!("{:?}", minimum_distance);

    let lower_corner = (grid.len() - 1, grid.last().unwrap().len() - 1);
    println!(
        "Part 1\n\tDistance to lower right corner: {}",
        minimum_distance.get(&lower_corner).unwrap()
    );
}
