pub fn day_seven() {
    let mut horizontal_positions: Vec<_> = std::fs::read_to_string("resources/07/sample.txt")
        .unwrap()
        .trim()
        .split(",")
        .map(str::parse::<u64>)
        .collect::<Result<_, _>>()
        .unwrap();

    horizontal_positions.sort();
    let fuels_linear: Vec<_> = horizontal_positions
        .iter()
        .map(|x| {
            horizontal_positions
                .iter()
                .map(|y| (*x as i64 - *y as i64).abs() as u64)
                .sum::<u64>()
        })
        .collect();
    println!("{:?}", horizontal_positions);
    println!("{:?}", fuels_linear.iter().min().unwrap());

    let max_move_distance =
        horizontal_positions.iter().max().unwrap() - horizontal_positions.iter().min().unwrap();

    let mut last = 0;
    let mut move_distance_lookup = Vec::new();
    for i in 0..=max_move_distance {
        move_distance_lookup.push(i + last);
        last = *move_distance_lookup.last().unwrap();
    }
    let fuels_accumulating: Vec<_> = horizontal_positions
        .iter()
        .map(|x| {
            horizontal_positions
                .iter()
                .map(|y| {
                    let z = (*x as i64 - *y as i64).abs() as u64;
                    move_distance_lookup[z as usize]
                })
                .sum::<u64>()
        })
        .collect();

    println!("{:?}", move_distance_lookup);
    println!("{:?}", fuels_accumulating);
    println!("{:?}", fuels_accumulating.iter().min().unwrap());
}
