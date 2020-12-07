#![allow(dead_code)]

use combine::lib::cell::RefCell;
use combine::lib::collections::HashSet;
use combine::{EasyParser, Parser};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Default, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct BagDefinition {
    pub color: String,
    pub inner_bags: Vec<String>,
}

pub fn parse_bag_def(input: &str) -> BagDefinition {
    // possible formats:
    // <color> bags contain no other bags.
    // <color> bags contain [(1 <color> bag)/(N <color> bags), ]* (1 <color> bag)/(N <color> bags).

    let mut inner_bags = Vec::new();

    let (color, rest) = combine::parser::range::take_until_range(" bags contain ")
        .easy_parse(input)
        .unwrap();

    if !rest.starts_with(" bags contain no other bags.") {
        let rest = rest
            .trim_start_matches(" bags contain ")
            .trim_end_matches('.')
            .split(", ")
            .collect::<Vec<&str>>();

        for bag in rest {
            let bag = bag.trim_end_matches(" bags").trim_end_matches(" bag");

            let (number, color): (&str, &str) =
                combine::parser::range::take_while1(|c: char| c.is_digit(10))
                    .skip(combine::parser::char::space())
                    .easy_parse(bag)
                    .unwrap();

            for _ in 0..number.parse::<i32>().unwrap() {
                inner_bags.push(color.to_string());
            }
        }
    }

    BagDefinition {
        color: color.to_string(),
        inner_bags,
    }
}

pub fn get_all_bag_defs() -> Vec<BagDefinition> {
    include_str!("../input.txt")
        .lines()
        .map(|line| parse_bag_def(&line))
        .collect()
}

#[derive(Default, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Bag {
    pub color: String,
    pub inner_bags: Vec<Rc<RefCell<Bag>>>,
    pub inner_bags_flat: Vec<Rc<RefCell<Bag>>>,
    pub contains_shiny_gold: bool,
}

pub fn get_all_bags() -> Vec<Rc<RefCell<Bag>>> {
    let mut bags: Vec<Rc<RefCell<Bag>>> = Vec::new();
    let mut processed_colors: HashSet<String> = HashSet::new();
    let mut defs: Vec<Option<BagDefinition>> =
        get_all_bag_defs().into_iter().map(Option::Some).collect();

    // at first, just get all the "innermost" bags
    // so every bag that doesn't contain any other bag on the inside
    defs.iter_mut()
        .filter(|d| d.as_ref().unwrap().inner_bags.is_empty())
        .for_each(|b| {
            if let Some(x) = b.take() {
                processed_colors.insert(x.color.clone());
                bags.push(Rc::new(RefCell::new(Bag {
                    color: x.color,
                    inner_bags: Vec::new(),
                    inner_bags_flat: Vec::new(),
                    contains_shiny_gold: false,
                })));
            }
        });

    // now be going through all definitions, for as long as there's still some unprocessed
    // and if a bag definition contains bags that were already processed,
    // remove that definition, and add it to the final bag container
    while !defs.is_empty() {
        // dbg!(bags.len());

        defs.retain(Option::is_some);
        defs.iter_mut()
            .filter(|d| {
                let all_processed = d
                    .as_ref()
                    .unwrap()
                    .inner_bags
                    .iter()
                    .all(|c| -> bool { processed_colors.contains(c) });

                if all_processed {
                    processed_colors.insert(d.as_ref().unwrap().color.to_string());
                }
                all_processed
            })
            .for_each(|b| {
                if let Some(x) = b.take() {
                    let color = x.color;
                    let mut inner_bags = Vec::new();
                    let mut inner_bags_flat = Vec::new();
                    let mut contains_shiny_gold = false;

                    for color in x.inner_bags {
                        let w = bags.iter().find(|q| q.borrow().color == color).unwrap();
                        inner_bags.push(w.clone());

                        // inner_bags_flat.push(w.clone());
                        // inner_bags_flat.reserve(w.borrow().inner_bags_flat.len());
                        // for flat in &w.borrow().inner_bags_flat {
                        //     inner_bags_flat.push(flat.clone());
                        // }
                        // inner_bags_flat.sort_unstable();
                        // inner_bags_flat.dedup();

                        // if let Err(i) = inner_bags_flat.binary_search(w) {
                        //     inner_bags_flat.insert(i, w.clone());
                        // }
                        // for flat in &w.borrow().inner_bags_flat {
                        //     // if let Err(i) = inner_bags_flat.binary_search(flat) {
                        //     //     inner_bags_flat.insert(i, flat.clone());
                        //     // }
                        // }
                        //
                        // inner_bags_flat.sort_unstable();
                        // inner_bags_flat.dedup();

                        if w.borrow().contains_shiny_gold || w.borrow().color == "shiny gold" {
                            contains_shiny_gold = true;
                        }
                    }
                    bags.push(Rc::new(RefCell::new(Bag {
                        color,
                        inner_bags,
                        inner_bags_flat,
                        contains_shiny_gold,
                    })));
                }
            });
    }

    bags
}
