use std::collections::{VecDeque, HashMap};

pub fn day_ten(){
	let contents = std::fs::read_to_string("resources/10/sample.txt").unwrap();

	let syntax_lines: Vec<Vec<_>> = contents.split("\n").map(|x|{
		x.trim().chars().collect::<Vec<_>>()
	}).collect();

	let braces = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
	let penalty = HashMap::from([(')', 3 as u64), (']', 57), ('}', 1197), ('>', 25137)]);
	let score = HashMap::from([(')', 1 as u64), (']', 2), ('}', 3), ('>', 4)]);

	let mut total_penalty = 0;
	let mut completion_scores = Vec::new();
	for line in syntax_lines{
		let mut opened_braces = VecDeque::new();
		let mut line_corrupt = false;
		for y in line{
			if braces.contains_key(&y){
				opened_braces.push_back(braces.get(&y).unwrap());
			} else {
				if let Some(should_close) = opened_braces.pop_back(){
					if should_close != &y{
						// println!("Syntax Error: Expected {}, found {}", should_close, y);
						if let Some(val) = penalty.get(&y){
							total_penalty += val;
							line_corrupt = true;
						}
					}
				} else {
					println!("Syntax Error: Closed brace without opening brace.")
				}
			}
		}

		if !opened_braces.is_empty() && !line_corrupt{
			let mut total_score = 0;
			for y in opened_braces.iter().rev(){
				if let Some(val) = score.get(y){
					total_score = total_score*5 + val;
				}
			}
			completion_scores.push(total_score);
			// println!("Completion Score: {}", total_score);
		} else {
			// println!("Line OK.");
		}
	}

	completion_scores.sort();

	println!("Part 1\n\tTotal score: {}", total_penalty);
	println!("Part 2\n\tMiddle completion score: {}", completion_scores.get(completion_scores.len()/2).unwrap());
}