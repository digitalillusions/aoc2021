use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::io::{self, BufRead};

pub fn day_eight() {
    let file = fs::File::open("resources/08/sample.txt").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut digits_values_map = Vec::new();

    for line in lines {
        let contents = line
            .unwrap()
            .split(" | ")
            .map(|item| {
                item.split(" ")
                    .map(|item| item.chars().collect::<BTreeSet<_>>())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        digits_values_map.push((
            contents[0].clone(),
            contents[1].clone(),
            HashMap::new(),
            Vec::new(),
        ));
    }

    for (digits, _, map, _) in digits_values_map.iter_mut() {
        let loc_one = digits.iter().position(|x| x.len() == 2).unwrap();
        let digit_one = digits.swap_remove(loc_one);
        map.insert(digit_one, 1);

        let loc_four = digits.iter().position(|x| x.len() == 4).unwrap();
        let digit_four = digits.swap_remove(loc_four);
        map.insert(digit_four.clone(), 4);

        let loc_seven = digits.iter().position(|x| x.len() == 3).unwrap();
        let digit_seven = digits.swap_remove(loc_seven);
        map.insert(digit_seven.clone(), 7);

        let loc_eight = digits.iter().position(|x| x.len() == 7).unwrap();
        let digit_eight = digits.swap_remove(loc_eight);
        map.insert(digit_eight, 8);

        let loc_six = digits
            .iter()
            .position(|x| {
                (x.len() == 6) && (x.intersection(&digit_seven).collect::<Vec<_>>().len() == 2)
            })
            .unwrap();
        let digit_six = digits.swap_remove(loc_six);
        map.insert(digit_six, 6);

        let loc_nine = digits
            .iter()
            .position(|x| {
                (x.len() == 6) && (x.intersection(&digit_four).collect::<Vec<_>>().len() == 4)
            })
            .unwrap();
        let digit_nine = digits.swap_remove(loc_nine);
        map.insert(digit_nine, 9);

        let loc_zero = digits.iter().position(|x| x.len() == 6).unwrap();
        let digit_zero = digits.swap_remove(loc_zero);
        map.insert(digit_zero, 0);

        let loc_three = digits
            .iter()
            .position(|x| x.intersection(&digit_seven).collect::<Vec<_>>().len() == 3)
            .unwrap();
        let digit_three = digits.swap_remove(loc_three);
        map.insert(digit_three, 3);

        let loc_five = digits
            .iter()
            .position(|x| x.intersection(&digit_four).collect::<Vec<_>>().len() == 3)
            .unwrap();
        let digit_five = digits.swap_remove(loc_five);
        map.insert(digit_five, 5);

        map.insert(digits.remove(0), 2);
    }

    println!("Part 1");
    let mut sum_1478 = 0;
    for (_, values, map, values_dec) in digits_values_map.iter_mut() {
        *values_dec = values
            .iter()
            .map(|item| map.get(item).unwrap())
            .cloned()
            .collect::<Vec<_>>();
        sum_1478 += values_dec
            .iter()
            .map(|&x| {
                if (x == 1) || (x == 4) || (x == 7) || (x == 8) {
                    return 1;
                } else {
                    0
                }
            })
            .sum::<i32>();
    }

    println!("\tNum of 1, 4, 7 or 8: {}", sum_1478);

    println!("Part 2");
    let mut sum = 0;
    for (_, _, _, values_dec) in digits_values_map {
        sum += values_dec
            .iter()
            .map(i32::to_string)
            .collect::<Vec<_>>()
            .join("")
            .parse::<i32>()
            .unwrap();
    }

    println!("\tSum of values: {}", sum);
}
