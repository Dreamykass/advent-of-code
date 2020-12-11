use std::collections::HashMap;

fn get_input() -> Vec<i32> {
    include_str!("../input.txt")
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

#[allow(dead_code)]
fn part_one() {
    let mut input = get_input();
    input.sort_unstable();

    let mut prev_adapter = 0i32;
    let mut diffs = HashMap::<i32, i32>::new();

    for i in input {
        let diff = i - prev_adapter;
        *diffs.entry(diff).or_insert(0) += 1;
        prev_adapter = i;
    }
    *diffs.entry(3).or_insert(0) += 1;

    for (d, n) in diffs {
        println!("part one, (d, n): ({}, {})", d, n);
    }
}

fn travel(input: &[i32], index: usize) -> u128 {
    if index >= input.len() - 1 {
        1
    } else {
        let current = unsafe { *input.get_unchecked(index) };
        input
            .iter()
            .enumerate()
            .skip(index + 1)
            .take_while(|(_, next)| **next - current <= 3)
            .map(|(next_index, _)| travel(input, next_index as usize))
            .sum()
    }
}

fn part_two() {
    let mut input = get_input();
    input.sort_unstable();

    let count = travel(&*input, 0);

    println!("part two: {}", count);
}

fn main() {
    // part_one();
    part_two();
}
