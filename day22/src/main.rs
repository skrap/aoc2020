use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::TryInto,
};

fn parse(input: &str) -> [VecDeque<usize>; 2] {
    let result: Vec<VecDeque<usize>> = input
        .split("\n\n")
        .map(|one_player| one_player.lines().flat_map(str::parse::<usize>).collect())
        .collect();
    result.try_into().unwrap()
}

fn part1(input: &str) -> usize {
    let mut decks = parse(input);
    while decks[0].len() > 0 && decks[1].len() > 0 {
        let c0 = decks[0].pop_front().unwrap();
        let c1 = decks[1].pop_front().unwrap();
        if c0 > c1 {
            decks[0].push_back(c0);
            decks[0].push_back(c1);
        } else {
            decks[1].push_back(c1);
            decks[1].push_back(c0);
        }
    }

    decks
        .iter()
        .map(|deck| {
            deck.iter()
                .rev()
                .enumerate()
                .map(|(idx, card)| (idx + 1) * card)
                .sum::<usize>()
        })
        .sum()
}

type Decks = [VecDeque<usize>; 2];

fn do_round(
    decks: &mut Decks,
    seen: &mut HashSet<Decks>,
    memos: &mut HashMap<Decks, (usize, Decks)>,
) -> Option<usize> {
    let winner;
    if seen.contains(decks) {
        // player 1 wins
        Some(0usize)
    } else {
        seen.insert(decks.clone());

        let c0 = decks[0].pop_front().unwrap();
        let c1 = decks[1].pop_front().unwrap();
        if decks[0].len() >= c0 && decks[1].len() >= c1 {
            // recurse
            let mut new_decks = [
                decks[0].iter().copied().take(c0).collect(),
                decks[1].iter().copied().take(c1).collect(),
            ];
            winner = do_game(&mut new_decks, memos);
        } else {
            winner = if c0 > c1 { 0 } else { 1 }
        }

        if winner == 0 {
            decks[0].push_back(c0);
            decks[0].push_back(c1);
        } else {
            decks[1].push_back(c1);
            decks[1].push_back(c0);
        }
        None
    }
}

fn do_game(
    decks: &mut Decks,
    memos: &mut HashMap<Decks, (usize, Decks)>,
) -> usize {
    let mut seen = HashSet::new();

    let winner = loop {
        if decks[0].is_empty() {
            break 1;
        }
        if decks[1].is_empty() {
            break 0;
        }

        if let Some(winner) = do_round(decks, &mut seen, memos) {
            break winner;
        }
    };

    winner
}

fn part2(input: &str) -> usize {
    let mut decks = parse(input);
    let mut memos = HashMap::new();

    let winner = do_game(&mut decks, &mut memos);

    decks[winner]
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, card)| (idx + 1) * card)
        .sum::<usize>()
}

#[test]
fn test_part2() {
    let input = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
";

    assert_eq!(part2(input), 291);
}

fn main() {
    dbg!(part1(include_str!("../input")));
    dbg!(part2(include_str!("../input")));
}
