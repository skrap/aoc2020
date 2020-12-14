use std::{collections::HashMap, vec};

enum Instr {
    Mask(String),
    Mem { addr: usize, val: usize },
}

fn parse(input: &str) -> Vec<Instr> {
    let re = regex::Regex::new(r"(mask|mem)(?: = (\w+)|\[(\d+)\] = (\d+))").unwrap();
    let mut result = vec![];
    for line in input.lines() {
        let capt = re.captures(line).unwrap();
        if &capt[1] == "mask" {
            result.push(Instr::Mask(capt[2].to_string()));
        } else if &capt[1] == "mem" {
            result.push(Instr::Mem {
                addr: capt[3].parse().unwrap(),
                val: capt[4].parse().unwrap(),
            })
        }
    }
    result
}

fn part1(input: &str) -> usize {
    let instrs = parse(input);
    let mut and = 0;
    let mut or = 0;
    let mut mem: HashMap<usize, usize> = HashMap::new();
    for instr in instrs {
        match instr {
            Instr::Mask(s) => {
                and = 0usize;
                or = 0usize;
                for ch in s.chars() {
                    match ch {
                        'X' => {
                            and = (and << 1) | 1;
                            or <<= 1;
                        }
                        '1' => {
                            and <<= 1;
                            or = (or << 1) | 1;
                        }
                        '0' => {
                            and <<= 1;
                            or <<= 1;
                        }
                        _ => unimplemented!(),
                    }
                }
            }
            Instr::Mem { addr, mut val } => {
                val &= and;
                val |= or;
                mem.insert(addr, val);
            }
        }
    }
    mem.iter().map(|(_, val)| *val).sum()
}

fn part2(input: &str) -> usize {
    let instrs = parse(input);
    let mut mem: HashMap<usize, usize> = HashMap::new();
    let mut mask = String::new();
    for instr in instrs {
        match instr {
            Instr::Mask(s) => {
                mask = s;
            }
            Instr::Mem { addr, val } => {
                let float_bitcount = mask.chars().filter(|c| *c == 'X').count();
                for mut float in 0usize..(1 << float_bitcount) {
                    let mut decoded_addr = 0usize;
                    for (n, ch) in mask.chars().enumerate() {
                        match ch {
                            '0' => {
                                decoded_addr <<= 1;
                                decoded_addr |= (addr >> (mask.len() - 1 - n)) & 1;
                            }
                            '1' => {
                                decoded_addr <<= 1;
                                decoded_addr |= 1;
                            }
                            'X' => {
                                decoded_addr <<= 1;
                                decoded_addr |= float & 1;
                                float >>= 1;
                            }
                            _ => unimplemented!(),
                        }
                    }
                    mem.insert(decoded_addr, val);
                }
            }
        }
    }
    mem.iter().map(|(_, val)| *val).sum()
}

fn main() {
    dbg!(part1(include_str!("../input")));
    dbg!(part2(include_str!("../input")));
}
