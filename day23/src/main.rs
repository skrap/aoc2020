use std::collections::HashMap;

fn parse(input: &str) -> Vec<usize> {
    input
        .chars()
        .flat_map(|c| c.to_string().parse::<usize>())
        .collect()
}

fn do_move(cups: Vec<usize>, max_cup: usize) -> Vec<usize> {
    let mut next = vec![];
    let mut it = cups.into_iter();
    let current = it.next().unwrap();

    let picked = [it.next().unwrap(), it.next().unwrap(), it.next().unwrap()];
    let mut target = current - 1;
    if target == 0 {
        target = max_cup;
    }
    while picked.contains(&target) {
        target = target - 1;
        if target == 0 {
            target = max_cup;
        }
    }

    while let Some(num) = it.next() {
        next.push(num);
        if num == target {
            next.extend(&picked);
        }
    }
    next.push(current);

    next
}

fn part1(input: &str) -> String {
    let mut cups = parse(input);
    let max_cup = *cups.iter().max().unwrap();

    // do turns
    for _ in 0..100 {
        cups = do_move(cups, max_cup);
    }

    // find the 8 digits clockwise from 1
    cups.extend(cups.clone().iter());
    let idx = cups.iter().position(|e| *e == 1).unwrap() + 1;
    cups[idx..idx + 8]
        .iter()
        .map(|b| format!("{}", b))
        .collect()
}

fn do_move_2(current: usize, cups: &mut HashMap<usize, usize>, max_cup: usize) -> usize {
    // from "current", chop next 3 out
    let fragment_head = cups[&current];

    let mut picked = vec![fragment_head];
    for _ in 0..2 {
        picked.push(cups[picked.last().unwrap()]);
    }

    let mut target = current - 1;
    if target == 0 {
        target = max_cup;
    }
    while picked.contains(&target) {
        target = target - 1;
        if target == 0 {
            target = max_cup;
        }
    }

    // current->next is whatever last picked was pointed to
    let next_current = cups[&picked[2]];
    cups.insert(current, next_current);

    // save whatever target points to
    let after_splice = cups[&target];
    // target points to first picked
    cups.insert(target, picked[0]);
    // last picked points to what was after target
    cups.insert(picked[2], after_splice);

    next_current
}

fn part2(input: &str) -> usize {
    let cups = parse(input);
    let max_cup = *cups.iter().max().unwrap();

    let mut cup_to_next: HashMap<usize, usize> = HashMap::new();
    for cup2 in cups.windows(2) {
        cup_to_next.insert(cup2[0], cup2[1]);
    }
    cup_to_next.insert(*cups.last().unwrap(), max_cup + 1);
    for n in max_cup + 1..1_000_000 {
        cup_to_next.insert(n, n + 1);
    }
    cup_to_next.insert(1_000_000, cups[0]);
    let max_cup = 1_000_000;

    let mut current = cups[0];

    // do turns
    for _ in 0..10_000_000 {
        current = do_move_2(current, &mut cup_to_next, max_cup);
    }

    let after1 = cup_to_next[&1];
    let and_after_that = cup_to_next[&after1];
    after1 * and_after_that
}

#[test]
fn test_part1() {
    assert_eq!(&part1("389125467"), "67384529");
}

#[test]
fn test_part2() {
    assert_eq!(part2("389125467"), 149245887792);
}

fn main() {
    let input1 = "476138259";
    dbg!(part1(input1));
    dbg!(part2(input1));
}
