use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> HashMap<[i32; 2], bool> {
    let mut it = input.chars();
    let mut pos = [0, 0];
    let mut flips = vec![];
    while let Some(ch) = it.next() {
        let [u, v] = match ch {
            'e' => [1, 0],
            'w' => [-1, 0],
            's' => match it.next().unwrap() {
                'e' => [1, -1],
                'w' => [0, -1],
                _ => unimplemented!(),
            },
            'n' => match it.next().unwrap() {
                'e' => [0, 1],
                'w' => [-1, 1],
                _ => unimplemented!(),
            },
            '\n' => {
                flips.push(pos);
                pos = [0, 0];
                [0, 0]
            }
            _ => unimplemented!(),
        };
        pos[0] += u;
        pos[1] += v;
    }

    let mut tiles = HashMap::new();
    for pos in flips {
        *tiles.entry(pos).or_insert(true) ^= true;
    }
    tiles
}

fn part1(input: &str) -> usize {
    let tiles = parse(input);
    tiles.values().filter(|v| !**v).count()
}

fn neighbors(pos: &[i32; 2]) -> [[i32; 2]; 6] {
    [
        [pos[0] + 1, pos[1]],
        [pos[0] - 1, pos[1]],
        [pos[0], pos[1] + 1],
        [pos[0], pos[1] - 1],
        [pos[0] - 1, pos[1] + 1],
        [pos[0] + 1, pos[1] - 1],
    ]
}

fn part2(input: &str) -> usize {
    let tiles = parse(input);

    let mut black_tiles: HashSet<[i32; 2]> = tiles
        .into_iter()
        .filter_map(|(t, v)| if !v { Some(t) } else {
             None
             })
        .collect();

    println!("day {}: {} black tiles", 0, black_tiles.len());

    for day in 1..=100 {
        let mut to_examine = black_tiles.clone();
        for tile in black_tiles.iter() {
            for neighbor in &neighbors(tile) {
                to_examine.insert(*neighbor);
            }
        }

        let mut next_tiles = HashSet::new();
        for tile in to_examine.into_iter() {
            let black_neighbors = neighbors(&tile)
                .iter()
                .filter(|n| black_tiles.contains(*n))
                .count();
            if black_tiles.contains(&tile) {
                // tile is black
                if black_neighbors == 1 || black_neighbors == 2 {
                    next_tiles.insert(tile);
                }
            } else {
                // tile is white
                if black_neighbors == 2 {
                    next_tiles.insert(tile);
                }
            }
        }
        black_tiles = next_tiles;
        println!("day {}: {} black tiles", day, black_tiles.len());
    }

    black_tiles.len()
}

#[test]
fn test_part2() {
    let input = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew
";
    assert_eq!(part1(input), 10);
    assert_eq!(part2(input), 2208);
}

fn main() {
    dbg!(part1(include_str!("../input")));
    dbg!(part2(include_str!("../input")));
}
