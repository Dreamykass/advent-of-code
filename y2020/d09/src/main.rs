use itertools::Itertools;
use std::collections::VecDeque;

fn get_input() -> Vec<i128> {
    include_str!("../input.txt")
        .lines()
        .map(|s| s.parse::<i128>().unwrap())
        .collect()
}

fn part_one() -> i128 {
    let nums = get_input();
    let mut prevs = VecDeque::<i128>::new();

    nums.iter()
        .enumerate()
        .take_while(|(p, _)| *p < 25)
        .for_each(|(_, e)| prevs.push_back(*e));

    #[allow(clippy::filter_next)]
    let f = nums
        .iter()
        .skip(25)
        .filter(|n| {
            let ret = prevs
                .iter()
                .cartesian_product(prevs.iter())
                .filter(|(i1, i2)| i1 != i2)
                .any(|(i1, i2)| *i1 + *i2 == **n);

            prevs.pop_front();
            prevs.push_back(**n);

            !ret
        })
        .next();

    if let Some(i) = f {
        println!("part one: {}", *i);
        *i
    } else {
        println!("part one: ???");
        -1
    }
}

fn part_two() {
    let weakness = part_one();
    let nums = get_input();

    nums.iter().enumerate().for_each(|(i, _)| {
        let mut sum = 0i128;
        let mut range: Vec<i128> = nums
            .iter()
            .skip(i)
            .take_while(|&&z| {
                sum += z;
                sum < weakness
            })
            .copied()
            .collect();

        if sum == weakness && range.len() >= 2 {
            range.sort_unstable();
            println!(
                "part two: {}",
                range.last().unwrap() + range.first().unwrap(),
            );
        }
    });
}

fn main() {
    // part_one();
    part_two();
}
