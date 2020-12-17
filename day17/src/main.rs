use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

use itertools::iproduct;

fn parse(input: &str) -> HashSet<(i32, i32, i32)> {
    let mut map = HashSet::new();
    let z = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, byte) in line.bytes().enumerate() {
            let val = match byte {
                b'#' => true,
                b'.' => false,
                _ => unimplemented!(),
            };
            if val {
                map.insert((x as i32, y as i32, z));
            }
        }
    }
    map
}

fn part1(input: &str, rounds: usize) -> usize {
    let mut map = parse(input);
    for _round in 0..rounds {
        let bounds = map
            .iter()
            .fold(
                None,
                |acc: Option<((i32, i32), (i32, i32), (i32, i32))>, pos| {
                    if let Some(mut acc) = acc {
                        acc.0 .0 = acc.0 .0.min(pos.0);
                        acc.0 .1 = acc.0 .1.max(pos.0);
                        acc.1 .0 = acc.1 .0.min(pos.1);
                        acc.1 .1 = acc.1 .1.max(pos.1);
                        acc.2 .0 = acc.2 .0.min(pos.2);
                        acc.2 .1 = acc.2 .1.max(pos.2);
                        Some(acc)
                    } else {
                        Some(((pos.0, pos.0), (pos.1, pos.1), (pos.2, pos.2)))
                    }
                },
            )
            .map(|ranges| {
                (
                    RangeInclusive::new(ranges.0 .0 - 1, ranges.0 .1 + 1),
                    RangeInclusive::new(ranges.1 .0 - 1, ranges.1 .1 + 1),
                    RangeInclusive::new(ranges.2 .0 - 1, ranges.2 .1 + 1),
                )
            })
            .unwrap();

        // for z in bounds.2.clone() {
        //     println!("z: {}", z);
        //     for y in bounds.1.clone() {
        //         for x in bounds.0.clone() {
        //             print!("{}", if map.contains(&(x,y,z)) { '#' } else {'.' } );
        //         }
        //         println!();
        //     }
        //     println!();
        // }

        let mut next_map = HashSet::new();
        for pos in iproduct!(bounds.0, bounds.1, bounds.2) {
            let neighbors = iproduct!(
                    pos.0 - 1..=pos.0 + 1,
                    pos.1 - 1..=pos.1 + 1,
                    pos.2 - 1..=pos.2 + 1
                )
                .filter(|p| {
                    *p != pos && map.contains(p)
                })
                .count();
            if map.contains(&pos) {
                if (2..=3).contains(&neighbors) {
                    next_map.insert(pos);
                }
            } else {
                if neighbors == 3 {
                    next_map.insert(pos);
                }
            }
        }
        map = next_map;
    }
    map.len()
}


fn parse2(input: &str) -> HashSet<(i32, i32, i32, i32)> {
    let mut map = HashSet::new();
    let z = 0;
    let w = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, byte) in line.bytes().enumerate() {
            let val = match byte {
                b'#' => true,
                b'.' => false,
                _ => unimplemented!(),
            };
            if val {
                map.insert((x as i32, y as i32, z, w));
            }
        }
    }
    map
}

fn part2(input: &str, rounds: usize) -> usize {
    let mut map = parse2(input);
    for _round in 0..rounds {
        let bounds = map
            .iter()
            .fold(
                None,
                |acc: Option<((i32, i32), (i32, i32), (i32, i32), (i32, i32))>, pos| {
                    if let Some(mut acc) = acc {
                        acc.0 .0 = acc.0 .0.min(pos.0);
                        acc.0 .1 = acc.0 .1.max(pos.0);
                        acc.1 .0 = acc.1 .0.min(pos.1);
                        acc.1 .1 = acc.1 .1.max(pos.1);
                        acc.2 .0 = acc.2 .0.min(pos.2);
                        acc.2 .1 = acc.2 .1.max(pos.2);
                        acc.3 .0 = acc.3 .0.min(pos.3);
                        acc.3 .1 = acc.3 .1.max(pos.3);
                        Some(acc)
                    } else {
                        Some(((pos.0, pos.0), (pos.1, pos.1), (pos.2, pos.2), (pos.3, pos.3)))
                    }
                },
            )
            .map(|ranges| {
                (
                    RangeInclusive::new(ranges.0 .0 - 1, ranges.0 .1 + 1),
                    RangeInclusive::new(ranges.1 .0 - 1, ranges.1 .1 + 1),
                    RangeInclusive::new(ranges.2 .0 - 1, ranges.2 .1 + 1),
                    RangeInclusive::new(ranges.3 .0 - 1, ranges.3 .1 + 1),
                )
            })
            .unwrap();

        let mut next_map = HashSet::new();
        for pos in iproduct!(bounds.0, bounds.1, bounds.2, bounds.3) {
            let neighbors = iproduct!(
                    pos.0 - 1..=pos.0 + 1,
                    pos.1 - 1..=pos.1 + 1,
                    pos.2 - 1..=pos.2 + 1,
                    pos.3 - 1..=pos.3 + 1
                )
                .filter(|p| {
                    *p != pos && map.contains(p)
                })
                .count();
            if map.contains(&pos) {
                if (2..=3).contains(&neighbors) {
                    next_map.insert(pos);
                }
            } else {
                if neighbors == 3 {
                    next_map.insert(pos);
                }
            }
        }
        map = next_map;
    }
    map.len()
}


#[test]
fn feature() {
    let input = ".#.
..#
###";
    assert_eq!(part1(input, 6), 112);
}

fn main() {
    dbg!(part1(include_str!("../input"), 6));
    dbg!(part2(include_str!("../input"), 6));
}
