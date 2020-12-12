use itertools::iproduct;
use std::{fmt::Display, unimplemented};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Sym {
    Floor,
    Empty,
    Occupied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Map {
    cells: Vec<Sym>,
    width: isize,
    height: isize,
}

impl Map {
    fn from(input: &str) -> Self {
        let height = input.lines().count() as isize;
        let width = input.lines().next().unwrap().len() as isize;
        let cells = input
            .chars()
            .flat_map(|c| match c {
                'L' => Some(Sym::Empty),
                '.' => Some(Sym::Floor),
                '\n' => None,
                _ => unimplemented!(),
            })
            .collect();
        Self {
            height,
            width,
            cells,
        }
    }

    fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut Sym> {
        if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            self.cells.get_mut((y * self.width + x) as usize)
        } else {
            None
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<Sym> {
        if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            self.cells.get((y * self.width + x) as usize).copied()
        } else {
            None
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let ch = match self.get(x, y).unwrap() {
                    Sym::Floor => '.',
                    Sym::Empty => 'L',
                    Sym::Occupied => '#',
                };
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[test]
fn feature() {
    let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";
    assert_eq!(part1(input), 37);
}

fn part1(input: &str) -> usize {
    fn next_state(map: &Map, x: isize, y: isize) -> Sym {
        let prev_state: Sym = map.get(x, y).unwrap();
        let occ = iproduct!(x - 1..=x + 1, y - 1..=y + 1)
            .filter(|xy| *xy != (x, y))
            .flat_map(|(x, y)| map.get(x, y))
            .filter(|sym| matches!(*sym, Sym::Occupied))
            .count();
        match (prev_state, occ) {
            (Sym::Empty, 0) => Sym::Occupied,
            (Sym::Occupied, i) if i >= 4 => Sym::Empty,
            _ => prev_state,
        }
    }

    let mut map = Map::from(input);
    loop {
        let mut next = map.clone();
        for x in 0..map.width as isize {
            for y in 0..map.height as isize {
                *next.get_mut(x, y).unwrap() = next_state(&map, x, y);
            }
        }
        if next == map {
            //println!("{}", map);
            break;
        }
        map = next;
    }
    map.cells.iter().filter(|f| **f == Sym::Occupied).count()
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    DownRight,
    UpLeft,
    DownLeft,
}

impl Dir {
    fn go(&self, (x, y): (isize, isize)) -> (isize, isize) {
        match *self {
            Dir::Up => (x, y - 1),
            Dir::Down => (x, y + 1),
            Dir::Left => (x - 1, y),
            Dir::Right => (x + 1, y),
            Dir::UpRight => (x + 1, y - 1),
            Dir::DownRight => (x + 1, y + 1),
            Dir::UpLeft => (x - 1, y - 1),
            Dir::DownLeft => (x - 1, y + 1),
        }
    }
}

fn part2(input: &str) -> usize {
    fn next_state(map: &Map, x: isize, y: isize) -> Sym {
        let prev_state: Sym = map.get(x, y).unwrap();
        let mut seen = 0;

        use Dir::*;
        for dir in &[Up, Down, Left, Right, UpRight, DownRight, UpLeft, DownLeft] {
            let mut pos = dir.go((x, y));
            while let Some(sym) = map.get(pos.0, pos.1) {
                match sym {
                    Sym::Floor => {
                        pos = dir.go(pos);
                    }
                    Sym::Empty => {
                        break;
                    }
                    Sym::Occupied => {
                        seen += 1;
                        break;
                    }
                }
            }
        }

        match (prev_state, seen) {
            (Sym::Empty, 0) => Sym::Occupied,
            (Sym::Occupied, i) if i >= 5 => Sym::Empty,
            _ => prev_state,
        }
    }

    let mut map = Map::from(input);
    loop {
        let mut next = map.clone();
        for x in 0..map.width as isize {
            for y in 0..map.height as isize {
                *next.get_mut(x, y).unwrap() = next_state(&map, x, y);
            }
        }
        if next == map {
            //println!("{}", map);
            break;
        }
        map = next;
    }
    map.cells.iter().filter(|f| **f == Sym::Occupied).count()
}

#[test]
fn feature() {
    let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";
    assert_eq!(part1(input), 37);
}
fn main() {
    dbg!(part1(include_str!("../input")));
    dbg!(part2(include_str!("../input")));
}
