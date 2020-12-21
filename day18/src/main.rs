use regex::Regex;
use std::{collections::VecDeque, unimplemented};

fn do_math(syms: &mut VecDeque<&str>) -> i64 {
    let mut op = None;
    let mut tot = 0;
    while let Some(sym) = syms.pop_front() {
        if let Some(val) = match sym {
            ")" => break,
            "(" => Some(do_math(syms)),
            "+" | "*" => {
                op = Some(sym);
                None
            }
            digits => Some(digits.parse().unwrap()),
        } {
            if let Some(op) = op.take() {
                match op {
                    "*" => tot *= val,
                    "+" => tot += val,
                    _ => unimplemented!(),
                }
            } else {
                tot = val;
            }
        }
    }
    tot
}

fn parse(input: &str) -> VecDeque<&str> {
    let re = Regex::new(r"([+*()]|\d+)").unwrap();
    re.find_iter(input).map(|m| m.as_str()).collect()
}

fn part1(input: &str) -> i64 {
    input.lines().map(|l| do_math(&mut parse(l))).sum()
}

#[derive(Debug)]
enum Node {
    Add(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Number(i64),
}

impl Node {
    fn eval(&self) -> i64 {
        match self {
            Node::Add(a, b) => a.eval() + b.eval(),
            Node::Mul(a, b) => a.eval() * b.eval(),
            Node::Number(n) => *n,
        }
    }
}

type Tokens<'a> = VecDeque<&'a str>;

fn expr(tokens: &mut Tokens) -> Option<Node> {
    if let Some("(") = tokens.front().copied() {
        tokens.pop_front();
        let result = mul(tokens);
        assert_eq!(Some(")"), tokens.pop_front());
        assert!(result.is_some());
        result
    } else {
        None
    }
}

fn term(tokens: &mut Tokens) -> Option<Node> {
    expr(tokens).or_else(|| tokens.pop_front().and_then(|digits| digits.parse::<i64>().ok()).map(|parsed| Node::Number(parsed)))
}

fn add(tokens: &mut Tokens) -> Option<Node> {
    let mut acc = None;
    while let Some(node) = term(tokens) {
        if let Some(lhs) = acc {
            acc = Some(Node::Add(Box::new(lhs), Box::new(node)));
        } else {
            acc = Some(node);
        }

        if let Some(&"+") = tokens.front() {
            tokens.pop_front();
        } else {
            break;
        }
    }
    acc
}

fn mul(tokens: &mut Tokens) -> Option<Node> {
    let mut acc = None;
    while let Some(node) = add(tokens) {
        if let Some(lhs) = acc {
            acc = Some(Node::Mul(Box::new(lhs), Box::new(node)));
        } else {
            acc = Some(node);
        }

        if let Some(&"*") = tokens.front() {
            tokens.pop_front();
        } else {
            break;
        }
    }
    acc
}

fn parse2(input: &str) -> Node {
    let mut tokens = parse(input);
    let node = mul(&mut tokens);
    // println!("{}\n{:?}\n", input, node);
    assert!(tokens.is_empty());
    node.unwrap()
}

fn part2(input: &str) -> i64 {
    input.lines().map(|line| parse2(line).eval()).sum()
}

fn main() {
    dbg!(part1(include_str!("../input")));
    dbg!(part2(include_str!("../input")));
}
