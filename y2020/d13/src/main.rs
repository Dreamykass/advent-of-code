#[allow(dead_code)]
fn part_one() {
    fn get() -> (i64, Vec<i64>) {
        let mut lines = include_str!("../input.txt").lines();
        (
            lines.next().unwrap().parse().unwrap(),
            lines
                .next()
                .unwrap()
                .split(',')
                .filter(|&ss| ss != "x")
                .map(|ss| ss.parse().unwrap())
                .collect(),
        )
    }

    let (earliest, buses) = get();
    let mut departure_bus: Vec<(i64, i64)> = Vec::new();

    for bus in buses {
        let mut i = 0;
        while i < earliest * 2 {
            departure_bus.push((i, bus));
            i += bus;
        }
    }

    departure_bus.sort_unstable();
    let i = departure_bus.binary_search(&(earliest, -1)).unwrap_err();
    dbg!(departure_bus[i]);
    dbg!(departure_bus[i].1 * (departure_bus[i].0 - earliest));
}

fn part_two() {
    fn get() -> Vec<u64> {
        include_str!("../input.txt")
            .lines()
            .nth(1)
            .unwrap()
            .split(',')
            .map(|ss| if ss == "x" { "0" } else { ss })
            .map(|ss| ss.parse().unwrap())
            .collect()
    }

    let buses = get();
    // dbg!(buses
    //     .iter()
    //     .cloned()
    //     .enumerate()
    //     .collect::<Vec::<(usize, u64)>>());

    let mut i = 937u64;
    loop {
        let cond = buses.iter().enumerate().all(|(n, &bus)| {
            //
            if bus == 0 {
                true
            } else {
                (i + ((n) as u64) - 16) % bus == 0
            }
        });

        if cond {
            dbg!(i);
            break;
        }

        // i += first_bus;
        // i += 937;
        i += 937;
    }
}

fn main() {
    // part_one()
    part_two();
}
