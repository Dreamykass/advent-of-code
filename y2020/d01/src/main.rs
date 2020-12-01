fn get_input() -> Vec<u64> {
    let vec: Vec<u64> = include_str!("../input.txt")
        .split("\r\n")
        .map(|s| {
            // dbg!(s);
            s.parse().unwrap()
        })
        .collect();

    assert_eq!(vec.len(), 200);

    vec
}

fn part_one() {
    let mut vec = get_input();

    vec.sort_unstable();

    for n in &vec {
        if let Ok(i) = vec.binary_search(&(2020u64 - n)) {
            let m = vec.get(i).unwrap();
            println!(
                "first: {}; second: {}; first+second: {}; multiplied: {}",
                n,
                m,
                n + m,
                n * m
            );
            return;
        }
    }
}

fn part_two() {
    let mut vec = get_input();

    vec.sort_unstable();

    for n in &vec {
        for m in &vec {
            if let Ok(i) = vec.binary_search(&(2020u64 - n - m)) {
                let o = vec.get(i).unwrap();
                println!(
                    "first: {}; second: {}; third: {}; first+second+third: {}; multiplied: {}",
                    n,
                    m,
                    o,
                    n + m + o,
                    n * m * o
                );
                return;
            }
        }
    }
}

fn main() {
    part_one();
    part_two();
}
