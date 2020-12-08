use std::collections::HashSet;

#[derive(Debug,Clone,Copy)]
enum Ins {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn parse(input: &str) -> Vec<Ins> {
    input.lines().map(|line| {
        let mut spl = line.split_ascii_whitespace();
        if let (Some(op), Some(n)) = (spl.next(), spl.next()) {
            let n : i32 = n.parse().unwrap();
            match op {
                "acc" => Ins::Acc(n),
                "jmp" => Ins::Jmp(n),
                "nop" => Ins::Nop(n),
                other => panic!("unknown opcode {}", other),
            }
        } else {
            panic!("can't parse {}", line);
        }
    }).collect()
}

fn part1(input: &str) -> i32 {
    let code = parse(input);
    let mut acc = 0;
    let mut pc: i32 = 0;
    let mut seen : HashSet<i32> = HashSet::new();
    while !seen.contains(&pc) {
        seen.insert(pc);
        match code[pc as usize] {
            Ins::Acc(n) => {
                acc += n;
                pc += 1;
            }
            Ins::Jmp(n) => {
                pc += n;
            }
            Ins::Nop(_) => {
                pc += 1;
            }
        }
    }
    acc
}


fn run(code: &Vec<Ins>) -> Result<i32, String> {
    let mut acc = 0;
    let mut pc: i32 = 0;
    let mut seen : HashSet<i32> = HashSet::new();
    while !seen.contains(&pc) {
        seen.insert(pc);
        match code[pc as usize] {
            Ins::Acc(n) => {
                acc += n;
                pc += 1;
            }
            Ins::Jmp(n) => {
                pc += n;
            }
            Ins::Nop(_) => {
                pc += 1;
            }
        }
        if pc == code.len() as i32 {
            return Ok(acc);
        }
        if !(0..code.len() as i32).contains(&pc) {
            return Err("bad end".into());
        }
    }
    Err("loop".into())
}

fn part2(input: &str) -> i32 {
    let code = parse(input);
    for modn in 0..code.len() {
        let mut code = code.clone();
        let new = match &code[modn] {
            Ins::Jmp(n) => Ins::Nop(*n),
            Ins::Nop(n) => Ins::Jmp(*n),
            _ => continue, 
        };
        code[modn] = new;
        match run(&code) {
            Ok(acc) => return acc,
            Err(e) => println!("modn {}->{:?}: {}", modn, code[modn], e),
        }
    }
    panic!("unsolved")
}

fn main() {
    println!("Hello, world!");
    dbg!(part1(include_str!("../input")));
    dbg!(part2(include_str!("../input")));
}
