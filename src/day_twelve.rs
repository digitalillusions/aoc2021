use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, Error};

pub fn day_twelve() {
    println!("Part 1");

    let file = fs::File::open("resources/12/example1.txt").unwrap();
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

        if connection[0] != "start" && connection[1] != "end" {
            graph
                .entry(connection[1].clone())
                .or_insert_with(Vec::<String>::new)
                .push(connection[0].clone());
        }
    }

    // println!("Graph {:?}", graph);

    // let num_paths = count_paths(&graph, "start".to_string(), "start".to_string());
    // println!("Counted Paths: {}", num_paths.unwrap());

    println!("Part 2");
    println!("Graph {:?}", graph);
    let num_paths =
        count_paths_with_small_cave_twice(&graph, "start".to_string(), "start".to_string(), false);
    println!("Counted Paths: {}", num_paths.unwrap());
}

fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    current_path: String,
    current_node: String,
) -> Result<u32, Error> {
    let mut num_paths = 0;

    if current_node == "end" {
        println!("Found path: {}", current_path.clone());
        return Ok(1);
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
    mut visited_small_twice: bool,
) -> Result<u32, Error> {
    let mut num_paths = 0;

    if current_node == "end" {
        println!("Found path: {}", current_path.clone());
        return Ok(1);
    }

    for connection in graph.get(&current_node).unwrap() {
        if (connection.to_lowercase() == connection.clone())
            && current_path.clone().contains(connection)
			&& !visited_small_twice
        {
			visited_small_twice = true;
        } 
		else if (connection.to_lowercase() == connection.clone())
            && current_path.clone().contains(connection)
			&& visited_small_twice
		{
			continue;
		}


        num_paths += count_paths_with_small_cave_twice(
            graph,
            current_path.clone() + "-" + connection,
            connection.clone(),
			visited_small_twice
        )
        .unwrap();
    }
    Ok(num_paths)
}
