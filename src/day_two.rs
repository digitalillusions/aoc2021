use std::fs;

pub fn day_two(){
    let file_contents = fs::read_to_string("resources/02/sample.txt").unwrap();
    let instructions: Vec<_> = file_contents.lines().map(|item| {
        let a: Vec<&str> = item.split_ascii_whitespace().collect();
        (a[0], a[1].parse::<usize>().unwrap())
    }).collect();

    println!("Part 1");
    let mut forward = 0;
    let mut down = 0;
    for (direction, value) in instructions.clone(){
        match direction{
            "forward" => forward += value,
            "down" => down += value,
            "up" => down -= value,
            _ => println!("Unkown command {}", direction)
        };
    }

    println!("\tForward: {}, Down: {}, Multiplied: {}", forward, down, forward * down);

    println!("Part 2");
    let mut forward = 0;
    let mut down = 0;
    let mut aim = 0;
    for (direction, value) in instructions{
        match direction{
            "forward" => {
                forward += value;
                down += value * aim;
            },
            "down" => aim += value,
            "up" => aim -= value,
            _ => println!("Unkown command {}", direction)
        };
    }

    println!("\tForward: {}, Down: {}, Aim: {}, Multiplied: {}", forward, down, aim, forward * down);

}