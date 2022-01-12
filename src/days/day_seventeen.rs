pub fn day_seventeen() {
    let input = std::fs::read_to_string("resources/17/example1.txt").unwrap();
    let bounds = ["x=", "..", ", ", "y=", ".."];
    let mut input_str = input.as_str();
    for bound in bounds.windows(2) {
        let (start, end) = (
            input_str.find(bound[0]).unwrap(),
            input_str.find(bound[1]).unwrap(),
        );
        println!("{}", &input_str[start + bound[0].len()..end]);
        input_str = &input_str[end..];
    }
}
