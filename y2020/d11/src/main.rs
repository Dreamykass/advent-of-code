use crate::Space::*;
use std::collections::HashMap;

enum Space {
    Floor,
    Empty,
    Occupied,
}

fn get_input() -> HashMap<(i64, i64), Space> {
    let mut map = HashMap::new();

    include_str!("../input.txt")
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, ch)| {
                map.insert(
                    (x as i64, y as i64),
                    match ch {
                        '.' => Floor,
                        'L' => Empty,
                        '#' => Occupied,
                        _ => panic!(),
                    },
                );
            });
        });

    map
}

fn generation(curr: &HashMap<(i64, i64), Space>, post: &mut HashMap<(i64, i64), Space>) -> bool {
    let mut changed = false;

    let dirs = [
        (0i64, 1i64),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    curr.iter().for_each(|((x, y), space)| {
        // ------------------------------------------ part one:
        // let occupied_nearby = dirs
        //     .iter()
        //     .filter(|&&dir| matches!(curr.get(&(x + dir.0, y + dir.1)), Some(Occupied)))
        //     .count();

        // ------------------------------------------ part one:
        let occupied_nearby = dirs
            .iter()
            .filter(|&&dir| {
                let mut checked = (x + dir.0, y + dir.1);
                let mut out = false;

                while let Some(space) = curr.get(&checked) {
                    match space {
                        Occupied => {
                            out = true;
                            break;
                        }
                        Empty => {
                            break;
                        }
                        Floor => {}
                    }
                    checked = (checked.0 + dir.0, checked.1 + dir.1);
                }

                out
            })
            .count();

        match space {
            Floor => {
                post.insert((*x, *y), Floor);
            }
            Empty => {
                // if occupied_nearby == 0 {
                if occupied_nearby == 0 {
                    post.insert((*x, *y), Occupied);
                    changed = true;
                } else {
                    post.insert((*x, *y), Empty);
                }
            }
            Occupied => {
                // if occupied_nearby >= 4 {
                if occupied_nearby >= 5 {
                    post.insert((*x, *y), Empty);
                    changed = true;
                } else {
                    post.insert((*x, *y), Occupied);
                }
            }
        };
    });

    changed
}

fn main() {
    let mut curr: HashMap<(i64, i64), Space> = get_input();
    let mut post: HashMap<(i64, i64), Space> = HashMap::new();
    let mut changed = true;
    let mut generations = 0;

    while changed {
        changed = generation(&curr, &mut post);
        curr = post;
        post = HashMap::new();
        generations += 1;
    }

    let free_seats = curr.iter().filter(|&a| matches!(a.1, Occupied)).count();

    println!("part one, generations: {}", generations);
    println!("part one, free seats: {}", free_seats);
}
