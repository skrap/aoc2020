use std::collections::HashMap;

fn part2(input: &str) -> usize {
    let mut nums: Vec<_> = input.lines().flat_map(str::parse::<i32>).collect();
    let target = *nums.iter().max().unwrap() + 3;
    nums.push(target);
    nums.push(0);
    nums.sort();

    let mut memos = HashMap::new();

    fn ways(index: usize, nums: &[i32], memos: &mut HashMap<usize, usize>) -> usize {
        if index + 1 == nums.len() {
            return 1;
        }
        if !memos.contains_key(&index) {
            let answer = (index + 1..nums.len())
                .flat_map(|i| {
                    if nums[i] <= nums[index] + 3 {
                        Some(ways(i, nums, memos))
                    } else {
                        None
                    }
                })
                .sum();
            memos.insert(index, answer);
            // println!("ways from idx {}({}) = {}", index, nums[index], answer);
        }
        memos[&index]
    };

    ways(0, &nums, &mut memos)
}


fn part1(input: &str) -> i32 {
    let mut nums: Vec<_> = input.lines().flat_map(str::parse::<i32>).collect();
    nums.push(0);
    nums.sort();
    let mut gap1 = 0;
    let mut gap3 = 1; // from our adapter
    for pair in nums.windows(2) {
        if let &[j1, j2] = pair {
            match j2 - j1 {
                1 => gap1 += 1,
                3 => gap3 += 1,
                g => println!("other gap {}", g),
            }
        }
    }
    gap1 * gap3
}

#[test]
fn feature() {
    let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
    assert_eq!(part1(input), 22 * 10);
    assert_eq!(part2(input), 19208);
}

fn main() {
    dbg!(part1(include_str!("../input")));
    dbg!(part2(include_str!("../input")));
}
