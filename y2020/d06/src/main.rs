use std::collections::HashMap;

fn get_input() -> Vec<Vec<Vec<char>>> {
    include_str!("../input.txt")
        .split("\r\n\r\n")
        .map(|group| -> Vec<Vec<char>> {
            group
                .lines()
                .map(str::to_string)
                .map(|person| -> Vec<char> { person.chars().collect() })
                .collect()
        })
        .collect()
}

fn part_one() {
    let groups = get_input();

    let uniques_in_groups: Vec<i32> = groups
        .iter()
        .map(|group| -> Vec<char> {
            let mut uniq = Vec::<char>::new();
            for person in group {
                for answer in person {
                    if let Err(pos) = uniq.binary_search(answer) {
                        uniq.insert(pos, *answer)
                    }
                }
            }
            uniq
        })
        .map(|group| -> i32 { group.len() as i32 })
        .collect();

    let sum: i32 = uniques_in_groups.iter().sum();

    println!("part one: {}", sum);
}

fn part_two() {
    let groups = get_input();

    let agreements_in_group: Vec<i32> = groups
        .iter()
        .map(|group| -> Vec<char> {
            let mut all_agreed: HashMap<char, i32> = "abcdefghijklmnopqrstuvwxyz"
                .chars()
                .map(|c| (c, 0))
                .collect();
            let person_count = group.len() as i32;

            for person in group {
                for answer in person {
                    *all_agreed.get_mut(answer).unwrap() += 1;
                }
            }

            all_agreed
                .iter()
                .filter(|p| *p.1 == person_count)
                .map(|x| *x.0)
                .collect()
        })
        .map(|group| -> i32 { group.len() as i32 })
        .collect();

    let sum: i32 = agreements_in_group.iter().sum();

    println!("part two: {}", sum);
}

fn main() {
    part_one();
    part_two();
}
