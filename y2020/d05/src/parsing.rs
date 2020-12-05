use std::io::BufRead;

pub struct Seat {
    row: u32,
    col: u32,
}

pub fn seat_id_from_seat(seat: &Seat) -> u32 {
    seat.row * 8 + seat.col
}

fn map_booleans_to_seat(input: ([bool; 7], [bool; 3])) -> Seat {
    let mut row = 0;
    let mut col = 0;

    input.0.iter().enumerate().for_each(|(i, b)| {
        if *b {
            row += 2u32.pow(i as u32);
        }
    });

    input.1.iter().enumerate().for_each(|(i, b)| {
        if *b {
            col += 2u32.pow(i as u32);
        }
    });

    Seat { row, col }
}

fn map_string_to_booleans(input: String) -> ([bool; 7], [bool; 3]) {
    let (mut row, mut col) = ([false; 7], [false; 3]);

    assert_eq!(input.len(), 10);

    input.chars().take(7).enumerate().for_each(|(i, c)| {
        let i = 6 - i;
        row[i] = map_char_to_boolean(c)
    });

    input
        .chars()
        .skip(7)
        .take(3)
        .enumerate()
        .for_each(|(i, c)| {
            let i = 2 - i;
            col[i] = map_char_to_boolean(c)
        });

    (row, col)
}

fn map_char_to_boolean(c: char) -> bool {
    match c {
        'F' => false,
        'B' => true,
        'L' => false,
        'R' => true,
        _ => panic!("invalid char"),
    }
}

#[test]
fn mapping_tests_simple() {
    let f = map_booleans_to_seat(map_string_to_booleans("FFFFFFFLLL".to_string()));
    assert_eq!(f.col, 0);
    assert_eq!(f.row, 0);

    let f = map_booleans_to_seat(map_string_to_booleans("FFFFFFFLLR".to_string()));
    assert_eq!(f.col, 1);
    assert_eq!(f.row, 0);

    let f = map_booleans_to_seat(map_string_to_booleans("FFFFFFBLLR".to_string()));
    assert_eq!(f.col, 1);
    assert_eq!(f.row, 1);
}

#[test]
fn mapping_tests_example_cases() {
    [
        ("FBFBBFFRLR", 357),
        ("BFFFBBFRRR", 567),
        ("FFFBBBFRRR", 119),
        ("BBFFBBFRLL", 820),
    ]
    .iter()
    .for_each(|s| {
        let id = s.1;
        let s = map_string_to_booleans(s.0.to_string());
        let s = map_booleans_to_seat(s);
        let id2 = seat_id_from_seat(&s);
        assert_eq!(id2, id);
    })
}

pub fn get_input() -> Vec<Seat> {
    std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap)
        .map(map_string_to_booleans)
        .map(map_booleans_to_seat)
        .collect()
}
