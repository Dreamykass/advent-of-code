use std::collections::HashMap;
use std::io::BufRead;

fn get_passports() -> Vec<HashMap<String, String>> {
    let lines: Vec<Vec<String>> =
        std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
            .lines()
            .filter_map(|rs| rs.ok())
            .map(|s| -> Vec<String> { s.split(' ').map(String::from).collect() })
            .filter(|v| !v.is_empty())
            .collect();

    let mut passports: Vec<HashMap<String, String>> = Vec::new();

    let mut passport: HashMap<String, String> = HashMap::new();
    for line in &lines {
        if line.first().unwrap() == "" {
            passports.push(passport);
            passport = HashMap::new();
        } else {
            line.iter()
                .filter_map(|s| {
                    let mut p = s.split(':');
                    Some((p.next()?, p.next()?))
                })
                .for_each(|p| {
                    passport.insert(p.0.to_string(), p.1.to_string());
                });
        }
    }
    passports.push(passport);

    passports
}

fn part_one() {
    let count = get_passports()
        .iter()
        .filter(|pass| {
            ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .all(|&k| pass.contains_key(k))
        })
        .count();

    println!("part one valid passwords: {}", count);
}

fn between(i: i32, a: i32, b: i32) -> Option<()> {
    if i >= a && i <= b {
        Some(())
    } else {
        None
    }
}

fn part_two() {
    let count = get_passports()
        .iter()
        .filter(|&pass| {
            ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .all(|&k| pass.contains_key(k))
        })
        // .inspect(|pass| {
        //     dbg!(pass);
        // })
        .filter_map(|pass| {
            between(pass.get("byr")?.parse::<i32>().ok()?, 1920, 2002)?;
            between(pass.get("iyr")?.parse::<i32>().ok()?, 2010, 2020)?;
            between(pass.get("eyr")?.parse::<i32>().ok()?, 2020, 2030)?;

            // dbg!("*************** byr iyr eyr: OK");
            Some(pass)
        })
        .filter_map(|pass| {
            let &hgt = &pass.get("hgt")?;
            if hgt.len() == 5
                && hgt.ends_with("cm")
                && between(
                    hgt.chars()
                        .take(3)
                        .collect::<String>()
                        .parse::<i32>()
                        .ok()?,
                    150,
                    193,
                )
                .is_some()
            {
                Some(())
            } else if hgt.len() == 4
                && hgt.ends_with("in")
                && between(
                    hgt.chars()
                        .take(2)
                        .collect::<String>()
                        .parse::<i32>()
                        .ok()?,
                    59,
                    76,
                )
                .is_some()
            {
                Some(())
            } else {
                None
            }?;

            // dbg!("hgt: OK");
            Some(pass)
        })
        .filter_map(|pass| {
            let hcl = pass.get("hcl")?;
            if hcl.starts_with('#') {
                if hcl
                    .chars()
                    .skip(1)
                    .filter(|c| c.is_ascii_hexdigit())
                    .count()
                    == 6
                    && hcl.chars().count() == 7
                {
                    Some(())
                } else {
                    None
                }
            } else {
                None
            }?;

            // dbg!("hcl: OK");
            Some(pass)
        })
        .filter_map(|pass| {
            if "amb blu brn gry grn hzl oth".contains(pass.get("ecl")?) {
                Some(())
            } else {
                None
            }?;

            // dbg!("ecl: OK");
            Some(pass)
        })
        .filter_map(|pass| {
            if pass.get("pid")?.chars().filter(|c| c.is_numeric()).count() == 9
                && pass.get("pid")?.chars().count() == 9
            {
                Some(())
            } else {
                None
            }?;

            // dbg!("hcl: OK");
            Some(pass)
        })
        // .inspect(|pass| {
        //     dbg!("-------------ALL OK");
        // })
        .count();

    println!("part two valid passwords: {}", count);
}

fn main() {
    part_one();
    part_two();
}
