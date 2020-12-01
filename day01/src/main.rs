use std::cmp::Ordering;

fn part1(target: i32) -> Option<(i32, i32)> {
    let input = include_str!("input");
    let mut nums : Vec<_> = input.lines().flat_map(|x| x.parse::<i32>()).collect();
    nums.sort();
    let mut low = nums.iter().peekable();
    let mut high = nums.iter().rev().peekable();
    let answer = loop {
        match (**low.peek()?+**high.peek()?).cmp(&target) {
            Ordering::Less => {
                low.next();
            }
            Ordering::Equal => {
                break (**low.peek().unwrap(), **high.peek().unwrap());
            }
            Ordering::Greater => {
                high.next();
            }
        }
    };
    return Some((answer.0, answer.1));
}

fn part2() -> Option<(i32,i32,i32)> {
    let input = include_str!("input");
    let mut nums : Vec<_> = input.lines().flat_map(|x| x.parse::<i32>()).collect();
    nums.sort();
    
    for num in nums {
        let target = 2020-num;
        if let Some((a,b)) = part1(target) {
            return Some((a,b,num));
        }
    }

    return None;
}

fn main() {
    let (a,b) = part1(2020).unwrap();
    println!("part 1: {}", a*b);
    let (a,b,c) = part2().unwrap();
    println!("part 2: {}", a*b*c);
}