use std::io::BufRead;

fn get_input() -> std::vec::Vec<[bool; 31]> {
    std::io::BufReader::new(std::fs::File::open("input.txt").unwrap_or_else(|_| {
        panic!(
            "current dir: {}",
            std::env::current_dir().unwrap().display()
        )
    }))
    .lines()
    .map(|s| -> [bool; 31] {
        #[allow(clippy::uninit_assumed_init)]
        let mut arr: [bool; 31] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };

        s.unwrap()
            .as_bytes()
            .iter()
            .map(|c| -> bool { *c == b'#' })
            .enumerate()
            .for_each(|x| arr[x.0] = x.1);
        arr
    })
    .collect()
}

fn trees_encountered(right: usize, down: usize) -> usize {
    get_input()
        .iter()
        .enumerate()
        .filter(|(ix, _)| -> bool { ix % down == 0 })
        .filter(|(ix, arr)| -> bool { arr[(*ix * right / down) % 31] })
        .count()
}

fn main() {
    println!("part one: {}", trees_encountered(3, 1));
    println!(
        "part two: {}",
        trees_encountered(1, 1)
            * trees_encountered(3, 1)
            * trees_encountered(5, 1)
            * trees_encountered(7, 1)
            * trees_encountered(1, 2)
    );
}
