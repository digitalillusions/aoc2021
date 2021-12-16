use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};

pub fn day_fourteen() {
    let file = fs::File::open("resources/14/sample.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();

    let template = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split("")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<char>().unwrap())
        .collect::<Vec<_>>();

    lines.next();

    let mut rules = HashMap::new();

    for line in lines {
        let rule = line
            .unwrap()
            .split(" -> ")
            .map(str::to_string)
            .collect::<Vec<_>>();
        let mut ab = rule[0].chars();
        let c = rule[1].parse::<char>().unwrap();
        rules.insert((ab.next().unwrap(), ab.next().unwrap()), c);
    }

    println!("Part 1 & 2");

    let mut template_pair_counts = HashMap::new();
    for ab in template.windows(2) {
        template_pair_counts
            .entry((ab[0], ab[1]))
            .and_modify(|x| *x += 1)
            .or_insert(1 as u64);
    }

    let iter_count = 40;
    for i in 0..iter_count {
        let mut polymer_chain_pair_counts = HashMap::new();

        for ((a, b), cnt) in template_pair_counts {
            let c = rules.get(&(a, b)).unwrap().clone();
            polymer_chain_pair_counts
                .entry((a, c))
                .and_modify(|x| *x += cnt)
                .or_insert(cnt);
            polymer_chain_pair_counts
                .entry((c, b))
                .and_modify(|x| *x += cnt)
                .or_insert(cnt);
        }

        let mut counts = HashMap::new();
        for ((a, _), cnt) in polymer_chain_pair_counts.clone() {
            counts.entry(a).and_modify(|x| *x += cnt).or_insert(cnt);
        }
        counts
            .entry(template.last().unwrap().clone())
            .and_modify(|x| *x += 1)
            .or_insert(1);

        template_pair_counts = polymer_chain_pair_counts;

        let most_common = counts.iter().max_by_key(|(_, a2)| *a2).unwrap();
        let least_common = counts.iter().min_by_key(|(_, a2)| *a2).unwrap();

        if i == 39 || i == 9 {
            println!("\tIteration {}\n\tCounts {:?}\n\tMost Common {:?}\n\tLeast Common {:?}\n\tSolution {}\n\t", i+1, counts, most_common, least_common, most_common.1-least_common.1);
        }
    }
}
