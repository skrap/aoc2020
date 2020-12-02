use lazy_static::lazy_static;
use regex::Regex;

fn check_pw(line: &str) -> bool {
    // 1-3 a: abcde
    // 1-3 b: cdefg
    // 2-9 c: ccccccccc
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    }
    let caps = RE.captures(line).unwrap();
    let (min, max, letter, pw) = (
        caps[1].parse::<usize>().unwrap(),
        caps[2].parse::<usize>().unwrap(),
        &caps[3],
        &caps[4],
    );
    let count = pw.matches(letter).count();
    return min <= count && count <= max;
}

fn part1() {
    println!(
        "part 1: {}",
        include_str!("../input").lines().filter(|l| check_pw(l)).count()
    );
}


fn check_pw_2(line: &str) -> bool {
    // 1-3 a: abcde
    // 1-3 b: cdefg
    // 2-9 c: ccccccccc
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    }
    let caps = RE.captures(line).unwrap();
    let (p1, p2, letter, pw) = (
        caps[1].parse::<usize>().unwrap(),
        caps[2].parse::<usize>().unwrap(),
        &caps[3],
        &caps[4],
    );
    pw.split_at(p1-1).1.starts_with(letter) != pw.split_at(p2-1).1.starts_with(letter)
}

fn part2() {
    println!(
        "part 2: {}",
        include_str!("../input").lines().filter(|l| check_pw_2(l)).count()
    );

}

fn main() {
    part1();
    part2();
}
