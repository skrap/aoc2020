use std::{
    collections::{btree_map::Range, HashMap},
    convert::TryInto,
    ops::Index,
    unimplemented,
};

use itertools::iproduct;
use regex::bytes::Regex;

struct Tile {
    id: usize,
    data: Vec<u8>,
    sides: [u16; 4], // top right bottom left
    dim: usize,
}

const TILE_SIDE: usize = 10;

fn to_tile(tile: &str) -> Option<Tile> {
    let id = tile
        .lines()
        .next()?
        .split(" ")
        .skip(1)
        .map(|id| id.split(":").next().unwrap().parse::<usize>().unwrap())
        .next()
        .unwrap();
    let data: Vec<_> = tile
        .lines()
        .skip(1)
        .map(|line| line.bytes())
        .flatten()
        .collect();
    let top = data.iter().take(TILE_SIDE).fold(0u16, |acc, ele| {
        (acc << 1) | if *ele == b'#' { 1 } else { 0 }
    });
    let bottom = data
        .iter()
        .skip(TILE_SIDE * (TILE_SIDE - 1))
        .take(TILE_SIDE)
        .fold(0u16, |acc, ele| {
            (acc << 1) | if *ele == b'#' { 1 } else { 0 }
        });
    let left = data.iter().step_by(TILE_SIDE).fold(0u16, |acc, ele| {
        (acc << 1) | if *ele == b'#' { 1 } else { 0 }
    });
    let right = data
        .iter()
        .skip(TILE_SIDE - 1)
        .step_by(TILE_SIDE)
        .fold(0u16, |acc, ele| {
            (acc << 1) | if *ele == b'#' { 1 } else { 0 }
        });
    Some(Tile {
        id,
        data,
        sides: [top, right, bottom, left],
        dim: TILE_SIDE,
    })
}

fn get_corners(tiles: &[Tile]) -> [usize; 4] {
    let mut sides: HashMap<u16, Vec<usize>> = HashMap::new();
    for tile in tiles {
        for side in tile.sides.iter() {
            sides.entry(*side).or_default().push(tile.id);
            sides
                .entry(side.reverse_bits() >> 6)
                .or_default()
                .push(tile.id);
        }
    }
    // find tiles with 2 unique sides. (really 4 due to flipping)
    let corners: Vec<_> = sides
        .iter()
        .filter_map(
            |(_side, ids)| {
                if ids.len() != 1 {
                    None
                } else {
                    Some(ids[0])
                }
            },
        )
        .fold(HashMap::new(), |mut map: HashMap<usize, usize>, id| {
            *map.entry(id).or_default() += 1;
            map
        })
        .iter()
        .filter_map(|(id, count)| if *count == 4 { Some(*id) } else { None })
        .collect();

    corners.try_into().unwrap()
}

fn part1(input: &str) -> usize {
    let tiles: Vec<_> = input.split("\n\n").flat_map(to_tile).collect();
    get_corners(&tiles).iter().product()
}

trait Flip {
    fn flipped(&self) -> u16;
}

impl Flip for u16 {
    fn flipped(&self) -> u16 {
        self.reverse_bits() >> 6
    }
}

impl Tile {
    // rotate clockwise
    fn rotate(&mut self) {
        self.sides = [
            self.sides[3].flipped(),
            self.sides[0],
            self.sides[1].flipped(),
            self.sides[2],
        ];
        let mut data = vec![];
        for x in 0..self.dim {
            for y in (0..self.dim).rev() {
                data.push(self.data[y * self.dim + x]);
            }
        }
        self.data = data;
    }

    fn flip_top_bottom(&mut self) {
        self.sides = [
            self.sides[2],
            self.sides[1].flipped(),
            self.sides[0],
            self.sides[3].flipped(),
        ];

        let mut data = vec![];
        for y in (0..self.dim).rev() {
            for x in 0..self.dim {
                data.push(self.data[y * self.dim + x]);
            }
        }
        self.data = data;
    }

    fn flip_left_right(&mut self) {
        self.sides = [
            self.sides[0].flipped(),
            self.sides[3],
            self.sides[2].flipped(),
            self.sides[1],
        ];

        let mut data = vec![];
        for y in 0..self.dim {
            for x in (0..self.dim).rev() {
                data.push(self.data[y * self.dim + x]);
            }
        }
        self.data = data;
    }

    fn top(&self) -> u16 {
        self.sides[0]
    }
    fn right(&self) -> u16 {
        self.sides[1]
    }
    fn bottom(&self) -> u16 {
        self.sides[2]
    }
    fn left(&self) -> u16 {
        self.sides[3]
    }

    fn orient_left(&mut self, left: u16) {
        assert!(self.sides.contains(&left) || self.sides.contains(&left.flipped()));
        while self.left() != left && self.left() != left.flipped() {
            self.rotate()
        }
        if self.left() == left.flipped() {
            self.flip_top_bottom()
        }
    }

    fn orient_top(&mut self, top: u16) {
        assert!(self.sides.contains(&top) || self.sides.contains(&top.flipped()));
        while self.top() != top && self.top() != top.flipped() {
            self.rotate()
        }
        if self.top() == top.flipped() {
            self.flip_left_right()
        }
    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        for y in 0..self.dim {
            s.push_str(std::str::from_utf8(&self.data[y*self.dim..(y+1)*self.dim]).unwrap());
            s.push('\n');
        }
        s
    }
}

fn part2(input: &str) -> usize {
    let mut tiles: Vec<_> = input.split("\n\n").flat_map(to_tile).collect();

    // create a map to find a specific edge
    let mut edge_to_tiles: HashMap<u16, Vec<usize>> = HashMap::new();
    for (idx, tile) in tiles.iter().enumerate() {
        for edge in tile
            .sides
            .iter()
            .map(|side| vec![*side, side.flipped()].into_iter())
            .flatten()
        {
            let tiles = edge_to_tiles.entry(edge).or_default();
            if !tiles.contains(&idx) {
                // should be very short - 1 or 2
                tiles.push(idx);
            }
        }
    }

    let start_tile_id = get_corners(&tiles)[0]; // start with 1 corner
    let (start_tile_idx, _) = tiles
        .iter_mut()
        .enumerate()
        .filter(|(_idx, t)| t.id == start_tile_id)
        .next()
        .unwrap();
    let mut pos_to_tile_idx = HashMap::new();
    pos_to_tile_idx.insert((0, 0), start_tile_idx);

    let find_matching = |edge, self_tile_idx: usize| -> Option<usize> {
        edge_to_tiles
            .get(&edge)
            .unwrap()
            .iter()
            .find(|tile_idx| **tile_idx != self_tile_idx)
            .copied()
    };

    // orient the start tile so its outer edges are up and left
    let start_tile = tiles.get_mut(start_tile_idx).unwrap();
    while !start_tile
        .sides
        .iter()
        .map(|side| find_matching(*side, start_tile_idx).is_none())
        .eq([true, false, false, true].iter().copied())
    {
        start_tile.rotate()
    }

    let dim: usize = (tiles.len() as f64).sqrt() as usize;
    assert_eq!(tiles.len(), dim*dim); // dim per side
    for pos in iproduct!(0..dim, 0..dim).skip(1) {
        if pos.1 == 0 {
            // left is paired
            let leftward_tile_idx = *pos_to_tile_idx.get(&(pos.0 - 1, pos.1)).unwrap();
            let leftward_tile = &tiles[leftward_tile_idx];
            let edge_to_left = leftward_tile.right();
            let tile_idx = find_matching(edge_to_left, leftward_tile_idx).unwrap();
            tiles.get_mut(tile_idx).unwrap().orient_left(edge_to_left);
            pos_to_tile_idx.insert(pos, tile_idx);
        } else {
            // top is paired
            let up_tile_idx = *pos_to_tile_idx.get(&(pos.0, pos.1 - 1)).unwrap();
            let up_tile = tiles.get(up_tile_idx).unwrap();
            let edge_up = up_tile.bottom();
            let tile_idx = find_matching(edge_up, up_tile_idx).unwrap();
            tiles.get_mut(tile_idx).unwrap().orient_top(edge_up);
            pos_to_tile_idx.insert(pos, tile_idx);
        }
    }

    // check that everything is right
    for pos in iproduct!(1..dim-1, 1..dim-1) {
        let tile = &tiles[*pos_to_tile_idx.get(&pos).unwrap()];
        let up_tile = &tiles[*pos_to_tile_idx.get(&(pos.0, pos.1 - 1)).unwrap()];
        let down_tile = &tiles[*pos_to_tile_idx.get(&(pos.0, pos.1 + 1)).unwrap()];
        let left_tile = &tiles[*pos_to_tile_idx.get(&(pos.0 - 1, pos.1)).unwrap()];
        let right_tile = &tiles[*pos_to_tile_idx.get(&(pos.0 + 1, pos.1)).unwrap()];
        assert_eq!(tile.top(), up_tile.bottom());
        assert_eq!(tile.left(), left_tile.right());
        assert_eq!(tile.bottom(), down_tile.top());
        assert_eq!(tile.right(), right_tile.left());
    }

    // ok so good data is 8x8 in each tile.
    // render to one large tile
    let total_width = 8 * dim;
    let mut rendered = vec![];
    for y in 0..total_width {
        for x in 0..total_width {
            let pos = (x / 8, y / 8);
            let subpos = (x % 8, y % 8);
            let tile = &tiles.get(*pos_to_tile_idx.get(&pos).unwrap()).unwrap();
            rendered.push(tile.data[(subpos.1 + 1) * TILE_SIDE + subpos.0 + 1]);
        }
    }

    let mut rendered = Tile {
        id: 0,
        data: rendered,
        sides: [0,0,0,0],
        dim: total_width,
    };

    let monster_str = 
                 "# 
#    ##    ##    ###
 #  #  #  #  #  #";
    let monster_width = monster_str.lines().map(|l| l.len()).max().unwrap();
    let monster_re_str: String = monster_str
        .bytes()
        .map(|c| match c {
            b'#' => "#".into(),
            b'\n' => format!(
                "[O#.\\n]{{{}}}",
                total_width - monster_width + 1 // +1 for line feed
            ),
            _ => ".".into(),
        })
        .collect();
    let monster_re = Regex::new(&monster_re_str).unwrap();

    orient_tile(&mut rendered, &monster_re);

    println!("{}", &rendered.to_string());

    let mut photo: Vec<_> = rendered.to_string().as_bytes().into();

    while let Some(m) = monster_re.find(&photo) {
        let mut offset = m.start();
        for b in monster_str.bytes() {
            if b == b'\n' {
                offset += total_width - monster_width;
            }
            if b == b'#' {
                assert_eq!(photo[offset], b'#');
                photo[offset] = b'O';
            }
            offset += 1;
        }
    }

    println!("{}", std::str::from_utf8(&photo[..]).unwrap());

    photo.iter().filter(|d| **d == b'#').count()
}

fn orient_tile(rendered: &mut Tile, monster_re: &Regex) {
    for _ in 0..4 {
        let matches = monster_re.find_iter(rendered.to_string().as_bytes()).count();
        if matches != 0 {
            return;
        }
        rendered.rotate();
    }
    rendered.flip_left_right();
    for _ in 0..4 {
        let matches = monster_re.find_iter(rendered.to_string().as_bytes()).count();
        if matches != 0 {
            return;
        }
        rendered.rotate();
    }
    rendered.flip_top_bottom();
    for _ in 0..4 {
        let matches = monster_re.find_iter(rendered.to_string().as_bytes()).count();
        if matches != 0 {
            return;
        }
        rendered.rotate();
    }
    rendered.flip_left_right();
    for _ in 0..4 {
        let matches = monster_re.find_iter(rendered.to_string().as_bytes()).count();
        if matches != 0 {
            return;
        }
        rendered.rotate();
    }
    unimplemented!();
}

#[test]
fn test_data() {
    assert_eq!(part2(include_str!("../test")), 273);
}

fn main() {
    dbg!(part1(include_str!("../input")));
    dbg!(part2(include_str!("../input")));
}
