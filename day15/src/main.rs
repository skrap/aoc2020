use std::collections::HashMap;
use std::convert::TryInto;

fn part1(input: &[u32], limit: usize) -> u32 {
    let mut spoken = HashMap::new();
    for (turn, i) in input[..input.len()-1].iter().enumerate() {
        spoken.insert(*i, turn);
    }
    let mut last = *input.last().unwrap();
    for turn in input.len()..limit {
        let next = (last,turn-1);
        if let Some(prev) = spoken.get(&last) {
            last = (turn - *prev - 1).try_into().unwrap();
        } else {
            last = 0;
            //println!("new! #{}: {}", next.1, next.0);
        }
        //println!("#{}: {}", next.1, next.0);
        spoken.insert(next.0, next.1);
    }
    last
}

#[test]
fn feature() {
    assert_eq!(part1(&[0,3,6], 2020), 436);
}

fn main() {
    dbg!(part1(&[6,3,15,13,1,0], 2020));
    dbg!(part1(&[6,3,15,13,1,0], 30_000_000));
}
