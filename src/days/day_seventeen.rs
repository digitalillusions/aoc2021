pub fn day_seventeen() {
    let input = std::fs::read_to_string("resources/17/example1.txt").unwrap();

    // Parse the input string
    let bounds = ["x=", "..", ", ", "y=", ".."];
    let mut input_str = input.as_str();
    let mut ranges: Vec<_> = bounds
        .windows(2)
        .map(|bound| {
            let (start, end) = (
                input_str.find(bound[0]).unwrap(),
                input_str.find(bound[1]).unwrap(),
            );
            let ret = &input_str[start + bound[0].len()..end];
            input_str = &input_str[end..];
            return ret;
        })
        .filter(|x| !x.is_empty())
        .collect();
    ranges.push(&input_str[bounds.last().unwrap().len()..]);

    // Get x and y ranges
    let x_range = ranges[0].parse::<i32>().unwrap()..ranges[1].parse::<i32>().unwrap();
    let y_range = ranges[2].parse::<i32>().unwrap()..ranges[3].parse::<i32>().unwrap();

    println!("{:?}, {:?}", x_range, y_range);
}
