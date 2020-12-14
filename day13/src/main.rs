use std::unimplemented;

fn part1(input: &str) -> i32 {
    let mut lines = input.lines();
    let now = lines.next().unwrap().parse::<i32>().unwrap();
    let bus_ids: Vec<i32> = lines
        .next()
        .unwrap()
        .split(',')
        .flat_map(|f| f.parse::<i32>())
        .collect();

    let bus_id_arrival: Vec<_> = bus_ids
        .iter()
        .map(|&id| {
            (id, ((now + id - 1) / id) * id - now) // need to round up
        })
        .collect();

    let (id, arrival) = bus_id_arrival
        .iter()
        .min_by_key(|(id, arrival)| *arrival)
        .unwrap();
    id * arrival
}

fn part2(input: &str) -> i64 {
    let mut lines = input.lines();
    let bus_ids_offsets: Vec<(i64, i64)> = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .flat_map(|(i, f)| {
            f.parse::<i64>().map(|parsed| (parsed, i as i64))
        })
        .collect();
    
    let (mut stride, mut start) = bus_ids_offsets[0];
    for (id, offset) in bus_ids_offsets.iter().skip(1) {
        let mut first = None;
        let mut maybe_time = start;
        loop {
            if (maybe_time+offset)%id == 0 {
                //println!("time {}, id {} offset {}", maybe_time, id, offset);
                if let Some(first) = first {
                    stride = maybe_time-first;
                    start = first;
                    //println!("start {}, stride {}", start, stride);
                    break;
                } else {
                    first = Some(maybe_time);
                }
            }
            maybe_time += stride;
        }
    }
    start
}

fn main() {
    dbg!(part1(include_str!("../input")));
    dbg!(part2(include_str!("../input")));
}
