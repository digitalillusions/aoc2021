use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};

pub fn day_fourteen() {
    let file = fs::File::open("resources/14/sample.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();

    let mut template = lines
        .next()
        .unwrap()
        .unwrap()
		.trim()
        .split("")
		.filter(|x| !x.is_empty())
		.map(|x| x.parse::<char>().unwrap())
        .collect::<Vec<_>>();

    println!("Template: {:?}", template);

	lines.next();

	let mut rules = HashMap::new();

	for line in lines{
		let rule = line.unwrap().split(" -> ").map(str::to_string).collect::<Vec<_>>();
		let mut ab = rule[0].chars();
		let c = rule[1].parse::<char>().unwrap();
		rules.insert((ab.next().unwrap(), ab.next().unwrap()), c);
	}

	println!("Rules: {:?}", rules);
	
	println!("Part 1");

	let mut template_pair_counts = HashMap::new();
	for ab in template.windows(2){
		template_pair_counts.entry((ab[0], ab[1])).and_modify(|x| *x+=1).or_insert(1);
	}


	for i in 0..10{
		let mut polymer_chain = Vec::new();

		polymer_chain.push(template.first().unwrap().clone());
		for ab in template.windows(2){
			polymer_chain.extend_from_slice(&[*rules.get(&(ab[0],ab[1])).unwrap(), ab[1]]);
		}

		let mut counts = HashMap::new();
		for a in &polymer_chain{
			counts.entry(a.clone()).and_modify(|x| *x += 1).or_insert(1 as u32);
		}
		template = polymer_chain;

		let most_common = counts.iter().max_by_key(|(_, a2)| *a2).unwrap();
		let least_common = counts.iter().min_by_key(|(_, a2)| *a2).unwrap();

		println!("\tIteration {} Length {}\n\tCounts {:?}\n\tMost Common {:?}\n\tLeast Common {:?}\n\tSolution {}\n\t", i+1, template.len(), counts, most_common, least_common, most_common.1-least_common.1);
	}
}
