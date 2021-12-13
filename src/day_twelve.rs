use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, Error};

pub fn day_twelve() {
    let file = fs::File::open("resources/12/sample.txt").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut graph = HashMap::<String, Vec<String>>::new();
    for line in lines {
        let connection: Vec<String> = line
            .unwrap()
            .split('-')
            .map(|item| item.trim().to_string())
            .collect();

        assert!(connection.len() == 2);

        graph
            .entry(connection[0].clone())
            .or_insert_with(Vec::<String>::new)
            .push(connection[1].clone());

        graph
            .entry(connection[1].clone())
            .or_insert_with(Vec::<String>::new)
            .push(connection[0].clone());
    }

    println!("Graph {:?}", graph);

    println!("Part 1");
    let num_paths = count_paths(&graph, "start".to_string(), "start".to_string());
    println!("\tCounted Paths: {}", num_paths.unwrap());

    println!("Part 2");
    let num_paths =
        count_paths_with_small_cave_twice(&graph, "start".to_string(), "start".to_string(), "".to_string());
    println!("\tCounted Paths: {}", num_paths.unwrap());
}

fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    current_path: String,
    current_node: String,
) -> Result<u32, Error> {
    let mut num_paths = 0;

    if current_node == "end" {
        // println!("Found path: {}", current_path.clone());
        return Ok(1);
    }
    else if current_node == "start" && current_path != "start" {
        return Ok(0);
    }

    for connection in graph.get(&current_node).unwrap() {
        if (connection.to_lowercase() == connection.clone())
            && current_path.clone().contains(connection)
        {
            continue;
        }

        num_paths += count_paths(
            graph,
            current_path.clone() + "-" + connection,
            connection.clone(),
        )
        .unwrap();
    }
    Ok(num_paths)
}

fn count_paths_with_small_cave_twice(
    graph: &HashMap<String, Vec<String>>,
    current_path: String,
    current_node: String,
    visited_small_twice: String,
) -> Result<u32, Error> {
    let mut num_paths = 0;

    if current_node == "end" {
        // println!("Found path: {}, visited_twice {}", current_path.clone(), visited_small_twice);
        return Ok(1);
    }
    else if current_node == "start" && current_path != "start" {
        return Ok(0);
    }

    for connection in graph.get(&current_node).unwrap() {
        if (connection.to_lowercase() == connection.clone())
            && (current_path.contains(connection))
        {
            if visited_small_twice.clone() == "" {
                num_paths += count_paths_with_small_cave_twice(
                    graph,
                    current_path.clone() + "-" + connection,
                    connection.clone(),
                    connection.clone()
                )
                .unwrap();
                continue;
            }
            else{
                continue;
            }
        }


        num_paths += count_paths_with_small_cave_twice(
            graph,
            current_path.clone() + "-" + connection,
            connection.clone(),
			visited_small_twice.clone()
        )
        .unwrap();
    }
    Ok(num_paths)
}
