use std::collections::{HashMap, HashSet, VecDeque};

use regex::Regex;

#[derive(Debug)]
struct Contains {
    bag: String,
    inside: Vec<(u32, String)>,
}

fn input_to_bags(input: &str) -> Vec<Contains> {
    let bag_re = Regex::new(r"^(\w+ \w+) bags .*").unwrap();
    let inner_re = Regex::new(r"(\d+) (\w+ \w+) bag").unwrap();
    let mut result = vec![];
    for line in input.lines() {
        let bag = bag_re.captures(line).unwrap()[1].into();
        let mut inside = vec![];
        for capture in inner_re.captures_iter(line) {
            inside.push((capture[1].parse().unwrap(), capture[2].into()));
        }
        result.push(Contains { bag, inside });
    }
    result
}

fn part1() -> usize {
    let bags = input_to_bags(include_str!("../input"));
    let mut in_to_out: HashMap<String, HashSet<String>> = HashMap::new();
    for Contains { bag, inside } in bags.into_iter() {
        for (_, inner) in inside.into_iter() {
            in_to_out.entry(inner).or_insert(HashSet::new()).insert(bag.clone());
        }
    }
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back("shiny gold");
    while let Some(color) = queue.pop_front() {
        if let Some(it) = in_to_out.get(color) {
            for out in it {
                seen.insert(out.as_str());
                queue.push_back(out.as_str());
            }
        }
    }
    seen.len()
}

fn part2() -> i32 {
    let bags = input_to_bags(include_str!("../input"));
    let mut out_to_in: HashMap<_, _> = HashMap::new();
    for Contains { bag, inside } in bags.iter() {
        for inner in inside.into_iter() {
            out_to_in.entry(bag.as_str())
                .or_insert(HashSet::new())
                .insert(inner.clone());
        }
    }
    
    let mut queue = VecDeque::new();
    queue.push_back((1, "shiny gold"));
    let mut total = 0;
    while let Some((mul, color)) = queue.pop_front() {
        total += mul;
        if let Some(inners) = out_to_in.get(color) {
            for (count, color) in inners {
                queue.push_back((*count as i32*mul, color));
            }
        }
    }
    total - 1
}

fn main() {
    dbg!(part1());
    dbg!(part2());
}
