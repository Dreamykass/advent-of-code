use crate::parsing::{get_all_bags, Bag};
use itertools::Itertools;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

mod parsing;
mod parsing_tests;

fn part_one() {
    let bags = get_all_bags();

    let bags_containing_shiny_gold = bags
        .iter()
        // .filter(|b| -> bool { bag_contains_shiny(&b.borrow().inner_bags) })
        .filter(|b| -> bool { b.borrow().contains_shiny_gold })
        .count();

    println!("part one: {}", bags_containing_shiny_gold);
}

fn part_two() {
    let bags = get_all_bags();
    let bags_inside_shiny_gold: usize = bags
        .iter()
        .find(|b| b.borrow().color == "shiny gold")
        .unwrap()
        .borrow()
        .inner_bags
        .iter()
        .map(|b| -> usize {
            fn count_inside(b: &[Rc<RefCell<Bag>>]) -> usize {
                if b.is_empty() {
                    1
                } else {
                    1usize
                        + b.iter()
                            .map(|c| count_inside(&*c.borrow().inner_bags))
                            .sum::<usize>()
                }
            };

            count_inside(&*b.borrow().inner_bags)
        })
        .sum();

    println!("part two: {}", bags_inside_shiny_gold);
}

fn main() {
    part_one();
    part_two();
}
