
fn part1(input: &str) -> i32 {
    let mut trees = 0;
    let mut xpos = 0;
    for line in input.lines() {
        if line.as_bytes()[xpos] == b'#' {
            trees += 1;
        }
        xpos += 3;
        if xpos >= line.len() {
            xpos -= line.len();
        }
    }
    trees
}

#[test]
fn test_part1() {
    let input = r"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
    assert_eq!(part1(input), 7);
}

fn navigate(input: &str, right: usize, down: usize) -> usize {
    let mut trees = 0;
    let mut xpos = 0;
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.as_bytes()[xpos] == b'#' {
            trees += 1;
        }
        xpos += right;
        if xpos >= line.len() {
            xpos -= line.len();
        }
        if down == 2 { // not general :(
            lines.next();
        }
    }
    trees
}

fn part2(input: &str) -> usize {
    [(1,1), (3,1), (5,1), (7,1), (1,2)].iter().map(|(right,down)|{
        navigate(input, *right as usize, *down as usize)
    }).fold(1, |a,b| a*b)
}

fn main() {
    let input = include_str!("../input");
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
