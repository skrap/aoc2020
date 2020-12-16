use std::{
    collections::{HashMap, VecDeque},
    ops::RangeInclusive,
};

struct Data {
    fields: Vec<(String, Vec<RangeInclusive<i32>>)>,
    mine: Vec<i32>,
    tickets: Vec<Vec<i32>>,
}

fn parse(input: &str) -> Data {
    let sections: Vec<_> = input.split("\n\n").collect();
    let mut fields = vec![];
    for line in sections[0].lines() {
        let mut it = line.split(": ");
        let name = it.next().unwrap();
        let ranges_str = it.next().unwrap();
        let ranges = ranges_str
            .split(" or ")
            .map(|f| {
                let mut both = f.split('-').flat_map(str::parse::<i32>);
                both.next().unwrap()..=both.next().unwrap()
            })
            .collect();
        fields.push((name.into(), ranges));
    }

    let mine = sections[1]
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .flat_map(str::parse::<i32>)
        .collect();

    let tickets = sections[2]
        .lines()
        .skip(1)
        .map(|ticket| ticket.split(',').flat_map(str::parse::<i32>).collect())
        .collect();

    Data {
        fields,
        mine,
        tickets,
    }
}

fn part1(input: &str) -> i32 {
    let data = parse(input);
    let mut sum_invalid = 0;
    for ticket in &data.tickets {
        for n in ticket {
            // does n fit anywhere?
            if !data
                .fields
                .iter()
                .map(|(_, v)| v.iter())
                .flatten()
                .any(|r| r.contains(n))
            {
                sum_invalid += *n;
            }
        }
    }
    sum_invalid
}

fn ticket_valid(ticket: &[i32], data: &Data) -> bool {
    for n in ticket {
        // does n fit anywhere?
        if !data
            .fields
            .iter()
            .map(|(_, v)| v.iter())
            .flatten()
            .any(|r| r.contains(n))
        {
            return false;
        }
    }
    true
}

fn part2(input: &str) -> usize {
    let mut data = parse(input);
    // only valid tickets
    data.tickets = data
        .tickets
        .iter()
        .filter(|t| ticket_valid(t, &data))
        .cloned()
        .collect();

    let mut field_to_col: HashMap<usize, usize> = HashMap::new();
    let mut unmapped_cols: VecDeque<_> = (0..data.mine.len()).collect();
    let mut unmapped_fields: Vec<_> = (0..data.fields.len()).collect();
    assert_eq!(unmapped_cols, unmapped_fields);
    while let Some(maybe_col) = unmapped_cols.pop_front() {
        let mut match_fields = vec![];
        for maybe_field in unmapped_fields.iter().copied().enumerate() {
            let (_, ranges) = &data.fields[maybe_field.1];
            if data
                .tickets
                .iter()
                .all(|t| ranges.iter().any(|range| range.contains(&t[maybe_col])))
            {
                match_fields.push(maybe_field);
            }
        }
        if match_fields.len() == 1 {
            field_to_col.insert(match_fields[0].1, maybe_col);
            unmapped_fields.swap_remove(match_fields[0].0);
        } else {
            unmapped_cols.push_back(maybe_col);
        }
    }
    field_to_col
        .iter()
        .map(|(field, col)| {
            if data.fields[*field].0.starts_with("departure") {
                data.mine[*col] as usize
            } else {
                1
            }
        })
        .fold(1, |a, ele| a * ele)
}

fn main() {
    dbg!(part1(include_str!("../input")));
    dbg!(part2(include_str!("../input")));
}
