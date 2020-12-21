use regex::Regex;
use std::{collections::HashMap, fmt::Write, write};

fn print_rule(w: &mut impl Write, num: usize, rules: &HashMap<usize, &str>) {
    let rule = *rules.get(&num).unwrap();
    if rule == "\"a\"" {
        write!(w, "a");
    } else if rule == "\"b\"" {
        write!(w, "b");
    } else {
        let parens = rule.contains("|");
        if parens {
            write!(w, "(?:");
        }
        for item in rule.split_whitespace() {
            if item == "|" {
                write!(w, "{}", item);
            } else {
                print_rule(w, item.parse().unwrap(), rules);
            }
        }
        if parens {
            write!(w, ")");
        }
    }
}

fn write_regex_str(input: &str, start_rule: usize, w: &mut impl Write) {
    let mut rules = HashMap::new();
    for line in input.lines() {
        let mut rulesplit = line.splitn(2, ": ");
        let rule_num: usize = rulesplit.next().unwrap().parse().unwrap();
        let rule = rulesplit.next().unwrap();
        rules.insert(rule_num, rule);
    }

    print_rule(w, start_rule, &rules);
}

fn create_regex(input: &str, start_rule: usize) -> Regex {
    let mut full_str = String::from("^");
    write_regex_str(input, start_rule, &mut full_str);
    write!(&mut full_str, "$");
    Regex::new(&full_str).unwrap()
}

fn parse(input: &str) -> (Regex, &str) {
    let parts: Vec<_> = input.split("\n\n").collect();
    (create_regex(parts[0], 0), parts[1])
}

fn part1(input: &str) -> usize {
    let (re, msgs) = parse(input);
    msgs.lines().filter(|line| re.is_match(*line)).count()
}

// fn is_match(text: &str, num: usize, rules: &HashMap<usize, &str>) -> (bool,&str) {
//     let rule = *rules.get(&num).unwrap();
//     if rule == "\"a\"" {
//         (text == "a", text[1..])
//     } else if rule == "\"b\"" {
//         (text == "b", text[1..])
//     } else {
//         for altern in rule.split(" | ") {
//             if altern.split_whitespace().all(|subrule| {

//             })
//         }
//     }
// }

// fn create_regex(input: &str) -> Regex {
//     let mut rules = HashMap::new();
//     for line in input.lines() {
//         let mut rulesplit = line.splitn(2, ": ");
//         let rule_num: usize = rulesplit.next().unwrap().parse().unwrap();
//         let rule = rulesplit.next().unwrap();
//         rules.insert(rule_num, rule);
//     }
// }

fn part2(input: &str) -> usize {
    let parts: Vec<_> = input.split("\n\n").collect();
    // observe that 0: 8 11
    // now: 8 is n 42s, n > 0
    // now: 11 is n 42s then n 31s, where n > 0
    // so we look for x 42s then y 31s, where x > y and y > 0
    let mut r42 = String::new();
    write_regex_str(parts[0], 42, &mut r42);
    let mut r31 = String::new();
    write_regex_str(parts[0], 31, &mut r31);

    let mut match_count = 0;
    let re_str = format!("^((?:{}){{2,}})((?:{})+)$", r42, r31);
    let re = Regex::new(&re_str).unwrap();
    for line in parts[1].lines() {
        if let Some(caps) = re.captures(line) {
            let most_42s = (2..)
                .filter(|n| {
                    !Regex::new(&format!("^(?:{}){{{},}}$", &r42, n))
                        .unwrap()
                        .is_match(&caps[1])
                })
                .next()
                .unwrap()
                - 1;

            let least_31s = (1..=most_42s)
                .filter(|n| {
                    !Regex::new(&format!("^(?:{}){{{},}}$", r31, n))
                    .unwrap()
                    .is_match(&caps[2])
                })
                .next()
                .map(|n| n-1);

            match least_31s {
                Some(0) => println!("^(?:{}){{{},}}$\ndoesn't match\n{}", r31, 1, &caps[2]),
                Some(least_31s) if most_42s > least_31s => {
                    match_count += 1;
                },
                _ => ()
            }
        }
    }
    match_count
}

fn main() {
    dbg!(part1(include_str!("../input")));
    dbg!(part2(include_str!("../input")));
}
