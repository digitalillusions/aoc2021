use std::collections::{VecDeque, HashSet, HashMap};

pub fn day_ten(){
	let contents = std::fs::read_to_string("resources/10/example1.txt").unwrap();

	let syntax_lines: Vec<Vec<_>> = contents.split("\n").map(|x|{
		x.trim().chars().collect::<Vec<_>>()
	}).collect();

	println!("{:?}", syntax_lines);

	let braces = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

	println!("{:?}", braces);

	for line in syntax_lines{
		let mut opened_braces = VecDeque::new();
		for y in line{
			if braces.contains_key(&y){
				opened_braces.push_back(braces.get(&y).unwrap());
			} else {
				if let Some(should_close) = opened_braces.pop_back(){
					if should_close != &y{
						println!("Syntax Error: Expected {}, found {}", should_close, y);
					}
				} else {
					println!("Syntax Error: Closed brace without opening brace.")
				}
			}
		}

		if !opened_braces.is_empty(){
			println!("Corrupted Line found, following braces remain {:?}", opened_braces);
		} else {
			println!("Line OK.");
		}
	}
}