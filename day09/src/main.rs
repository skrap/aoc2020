fn part1(input: &str) -> i32 {
    let nums : Vec<i32> = input.lines().flat_map(str::parse::<i32>).collect();
    'target: for window in nums.windows(26) {
        let target = window[25];
        let adds = &window[0..25];
        for i in 0..24 {
            for j in i+1..25 {
                if adds[i] + adds[j] == target {
                    // println!("{} = {} + {}", target, i, j);
                    continue 'target;
                }
            }
        }
        return target;
    }
    unreachable!()
}

fn part2(input: &str) -> i32 {
    let target = part1(input);
    let nums : Vec<i32> = input.lines().flat_map(str::parse::<i32>).collect();
    'winsize: for winsize in 2..nums.len() {
        for window in nums.windows(winsize) {
            if window.iter().fold(0i64, |a,n| a + *n as i64) == target as i64 {
                let mut window = window.to_vec();
                window.sort();
                return window[0] + window[window.len()-1];
            }
            if window.iter().all(|n| *n >= target) {
                continue 'winsize;
            }
        }
    }
    unreachable!()
}

fn main() {
    dbg!(part1(include_str!("../input")));
    dbg!(part2(include_str!("../input")));
}
