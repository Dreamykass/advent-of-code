use crate::parsing::{get_input, seat_id_from_seat};
use std::cmp::max;

mod parsing;

fn part_one() {
    let seats = get_input();

    let mut highest_id = 0;

    seats.iter().for_each(|s| {
        highest_id = max(highest_id, seat_id_from_seat(s));
    });

    println!("part one, highest id: {}", highest_id);
}

fn part_two() {
    let mut ids: Vec<u32> = get_input().iter().map(seat_id_from_seat).collect();
    ids.sort_unstable();
    for id in &ids {
        if matches!(ids.binary_search(&(id + 1)), Err(_))
            && matches!(ids.binary_search(&(id + 2)), Ok(_))
        {
            println!("part two, my seat id: {}", id + 1);
        }
    }
}

fn main() {
    part_one();
    part_two();
}
