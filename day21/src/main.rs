use regex::Regex;
use std::collections::{HashMap, HashSet};

fn part1(input: &str) {
    let mut all_ingreds = HashSet::new();
    let mut all_allergens: HashMap<&str, Vec<_>> = HashMap::new();
    let mut all_foods = vec![];

    let line_re = Regex::new(r"(?m)^(.+) \(contains (.+)\)$").unwrap();
    for m in line_re.captures_iter(input) {
        let ingreds: HashSet<&str> = m
            .get(1)
            .unwrap()
            .as_str()
            .split_ascii_whitespace()
            .collect();
        for allergen in m.get(2).unwrap().as_str().split(", ") {
            all_allergens
                .entry(allergen)
                .or_default()
                .push(ingreds.clone());
        }
        all_ingreds = all_ingreds.union(&ingreds).copied().collect();
        all_foods.push(ingreds);
    }

    let mut might_be_allergens = HashSet::new();
    for (_allergen, ingreds) in all_allergens.iter() {
        let ingreds_in_all = ingreds
            .iter()
            .fold(None, |acc: Option<HashSet<&str>>, next| match acc {
                Some(acc) => Some(acc.intersection(next).copied().collect()),
                None => Some(next.clone()),
            })
            .unwrap();
        might_be_allergens = might_be_allergens.union(&ingreds_in_all).copied().collect();
    }

    let cannot_be_allergens: HashSet<_> = all_ingreds
        .difference(&might_be_allergens)
        .copied()
        .collect();

    println!(
        "part 1: {}",
        all_foods
            .iter()
            .map(|food| cannot_be_allergens.intersection(food).count())
            .sum::<usize>()
    );

    let mut done_ingreds = cannot_be_allergens.clone();
    let mut allergen_might_be = HashMap::new();
    for (allergen, ingreds) in all_allergens.iter() {
        let ingreds_in_all = ingreds
            .iter()
            .fold(None, |acc: Option<HashSet<&str>>, next| match acc {
                Some(acc) => Some(acc.intersection(next).copied().collect()),
                None => Some(next.clone()),
            })
            .unwrap();
        allergen_might_be.insert(*allergen, ingreds_in_all);
    }

    let mut resolved = vec![];
    while done_ingreds != all_ingreds {
        let (&allergen, ingred) = allergen_might_be
            .iter()
            .find_map(|(k, v)| {
                let mut it = v.difference(&done_ingreds);
                let first = *it.next()?;
                if it.next().is_none() { Some((k,first)) } else { None }
            })
            .unwrap();
        resolved.push((allergen, ingred));
        done_ingreds.insert(ingred);
        allergen_might_be.remove(&*allergen);
    }
    resolved.sort();
    println!(
        "part 2: {}",
        resolved.iter().map(|r| r.1).collect::<Vec<_>>().join(",")
    );
}

fn main() {
    dbg!(part1(include_str!("../input")));
}
