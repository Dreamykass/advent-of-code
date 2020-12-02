use combine::{EasyParser, Parser};

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
#[display("{lowest}-{highest} {letter}: {password}")]
struct Element {
    lowest: i32,
    highest: i32,
    letter: char,
    password: String,
}

fn get_input_std() -> std::vec::Vec<Element> {
    std::include_str!("../input.txt")
        .split("\r\n")
        .map(|s| {
            let mut s = s.split(':');
            let password = s.next_back().unwrap().trim().to_string();

            let mut s = s.next_back().unwrap().split(' ');
            let letter = s.next_back().unwrap().chars().next_back().unwrap();

            let mut s = s.next_back().unwrap().split('-');
            let highest = s.next_back().unwrap().parse::<i32>().unwrap();
            let lowest = s.next_back().unwrap().parse::<i32>().unwrap();

            Element {
                lowest,
                highest,
                letter,
                password,
            }
        })
        .collect::<std::vec::Vec<Element>>()
}

fn get_input_combine1() -> std::vec::Vec<Element> {
    std::include_str!("../input.txt")
        .split("\r\n")
        .map(|initial| {
            let (lowest, rest): (&str, _) =
                combine::parser::range::take_while(|c: char| c.is_numeric())
                    .easy_parse(initial)
                    .unwrap();

            let rest = combine::skip_count(1, combine::token('-'))
                .parser
                .parse(rest)
                .unwrap()
                .1;

            let (highest, rest): (&str, _) =
                combine::parser::range::take_while(|c: char| c.is_numeric())
                    .easy_parse(rest)
                    .unwrap();

            let rest = combine::skip_count(1, combine::token(' '))
                .parser
                .parse(rest)
                .unwrap()
                .1;

            let (letter, rest): (&str, _) =
                combine::parser::range::take(1).easy_parse(rest).unwrap();

            let rest = combine::skip_count(1, combine::parser::char::string(": "))
                .parser
                .parse(rest)
                .unwrap()
                .1;

            let (password, _): (&str, _) =
                combine::parser::range::take_while(|c: char| c.is_alphabetic())
                    .easy_parse(rest)
                    .unwrap();

            Element {
                lowest: lowest.parse::<i32>().unwrap(),
                highest: highest.parse::<i32>().unwrap(),
                letter: letter.chars().next().unwrap(),
                password: password.to_string(),
            }
        })
        .collect::<std::vec::Vec<Element>>()
}

fn get_input_parse_display() -> std::vec::Vec<Element> {
    std::include_str!("../input.txt")
        .split("\r\n")
        .map(|s| s.parse::<Element>().unwrap())
        .collect::<std::vec::Vec<Element>>()
}

fn part_one() {
    let list = get_input_combine1();
    let mut valid = 0u32;

    for e in list {
        dbg!(&e);
        let count = e.password.chars().filter(|&c| c == e.letter).count() as i32;
        if count <= e.highest && count >= e.lowest {
            valid += 1;
        }
    }

    println!("part one valid passwords: {}", valid);
}

fn part_two() {
    let list = get_input_std();
    let mut valid = 0u32;

    for e in list {
        let mut chars = e.password.chars();
        let ch1 = chars.clone().nth((e.lowest - 1) as usize).unwrap();
        let ch2 = chars.nth((e.highest - 1) as usize).unwrap();

        if (ch1 == e.letter && ch2 != e.letter) || (ch1 != e.letter && ch2 == e.letter) {
            valid += 1;
        }
    }

    println!("part two valid passwords: {}", valid);
}

fn main() {
    part_one();
    part_two();
}
