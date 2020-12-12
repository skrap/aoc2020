enum Nav {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32)
}

fn parse(input: &str) -> Vec<Nav> {
    let mut result = vec![];
    for line in input.lines() {
        let (abc, num) = line.split_at(1);
        let parsed = match (abc, num.parse::<i32>().unwrap()) {
            ("F", n) => Nav::Forward(n),
            ("L", n) => Nav::Left(n),
            ("R", n) => Nav::Right(n),
            ("N", n) => Nav::North(n),
            ("S", n) => Nav::South(n),
            ("E", n) => Nav::East(n),
            ("W", n) => Nav::West(n),
            _ => panic!("unparseable: {}", line),
        };
        result.push(parsed)
    }
    result
}

struct Ship {
    pos: [i32;2],
    dir: Dir,
}

impl Ship {
    fn go(&mut self, nav: &Nav) {
        match nav {
            Nav::North(n) => { self.pos[1] -= n }
            Nav::East(n) => { self.pos[0] += n}
            Nav::South(n) => { self.pos[1] += n }
            Nav::West(n) => { self.pos[0] -= n }
            Nav::Left(n) => { for _ in 0..n/90 { self.dir = self.dir.left()}  }
            Nav::Right(n) => { for _ in 0..n/90 { self.dir = self.dir.right() }} 
            Nav::Forward(n) => { match self.dir {
                Dir::N => { self.pos[1] -= n}
                Dir::E => { self.pos[0]  += n}
                Dir::S => { self.pos[1] += n }
                Dir::W => { self.pos[0] -= n } 
            }}
        }
    }
}

enum Dir {
    N, E, S, W
}

impl Dir {
    fn left(&self) -> Self {
        match self {
            Dir::N => { Dir::W }
            Dir::E => { Dir::N }
            Dir::S => { Dir::E }
            Dir::W => { Dir::S }
        }
    }
    fn right(&self) -> Self {
        match self {
            Dir::N => { Dir::E }
            Dir::E => { Dir::S }
            Dir::S => { Dir::W }
            Dir::W => { Dir::N }
        }
    }
}

fn part1(input: &str) -> i32 {
    let navs = parse(input);
    let mut ship = Ship { pos: [0,0], dir: Dir::E };
    for nav in navs {
        ship.go(&nav);
    }
    ship.pos[0].abs() + ship.pos[1].abs()
}

fn part2(input: &str) -> i32 {
    let mut waypoint = Ship { pos: [10,-1], dir: Dir::E };
    let mut ship = Ship { pos: [0,0], dir: Dir::E };
    for nav in parse(input) {
        match nav {
            Nav::North(_) | Nav::South(_) | Nav::East(_) | Nav::West(_) => {
                waypoint.go(&nav);
            },
            Nav::Left(n) => {
                for _ in 0..n/90 {
                    waypoint.pos = [waypoint.pos[1], -waypoint.pos[0]];
                }
            }
            Nav::Right(n) => {
                for _ in 0..n/90 {
                    waypoint.pos = [-waypoint.pos[1], waypoint.pos[0]];
                }
            }
            Nav::Forward(n) => {
                ship.pos[0] += waypoint.pos[0]*n;
                ship.pos[1] += waypoint.pos[1]*n;
            }
        }
    }
    ship.pos[0].abs() + ship.pos[1].abs()
}

#[test]
fn test_part2() {
    assert_eq!(part2("F10
N3
F7
R90
F11"), 286);
}

fn main() {
    dbg!(part1(include_str!("../input")));
    dbg!(part2(include_str!("../input")));
}
