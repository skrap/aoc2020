fn code2int(line: &str) -> (u16, u16) {
    let mut row = 0;
    let mut seat = 0;
    for char in line.chars() {
        match char {
            'B' => {
                row <<= 1;
                row |= 1;
            }
            'F' => {
                row <<= 1;
            }
            'R' => {
                seat <<= 1;
                seat |= 1;
            }
            'L' => {
                seat <<= 1;
            }
            _ => {}
        }
    }
    (row, seat)
}

#[test]
fn testcode2row() {
    assert_eq!((70, 7), code2int("BFFFBBFRRR"));
}

fn part1() -> u16 {
    include_str!("../input")
        .lines()
        .map(|l| {
            let (r, s) = code2int(l);
            r * 8 + s
        })
        .max()
        .unwrap()
}

fn part2() -> u16 {
    let mut ids: Vec<_> = include_str!("../input")
        .lines()
        .map(|l| {
            let (r, s) = code2int(l);
            r * 8 + s
        })
        .collect();
    ids.sort();

    for trio in ids.windows(2) {
        if trio[1] - trio[0] == 2 {
            return trio[0] + 1;
        }
    }

    unreachable!();
}

fn main() {
    println!("Hello, world!");
    dbg!(part1());
    dbg!(part2());
}
