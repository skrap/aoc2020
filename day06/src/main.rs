fn part1(input: &str) -> u32{
    let mut group = 0u32;
    let mut total = 0u32;
    for line in input.lines() {
        if line.len() == 0 {
            total += group.count_ones();
            group = 0;
        } else {
            for c in line.bytes() {
                group |= 1 << (c - b'a');
            }
        }
    }
    total += group.count_ones();
    total
}

fn part2(input: &str) -> u32{
    let mut group = !0u32;
    let mut total = 0u32;
    for line in input.lines() {
        if line.len() == 0 {
            total += group.count_ones();
            group = !0;
        } else {
            let mut person = 0;
            for c in line.bytes() {
                person |= 1 << (c - b'a');
            }
            group &= person;
        }
    }
    total += group.count_ones();
    total
}

fn main() {
    dbg!(part1(include_str!("../input")));
    dbg!(part2(include_str!("../input")));
}
    
