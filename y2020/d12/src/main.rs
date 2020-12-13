#[derive(Debug)]
enum Action {
    MoveAbsolute((i64, i64)),
    Turn(i64),
    MoveForward(i64),
}

fn get_input() -> Vec<Action> {
    include_str!("../input.txt")
        .lines()
        .map(|l| -> Action {
            let ch = l.chars().next().unwrap();
            let num = l.split_at(1).1.parse::<i64>().unwrap();
            match ch {
                'N' => Action::MoveAbsolute((0, num)),
                'S' => Action::MoveAbsolute((0, -num)),
                'E' => Action::MoveAbsolute((num, 0)),
                'W' => Action::MoveAbsolute((-num, 0)),
                'L' => Action::Turn(-num),
                'R' => Action::Turn(num),
                'F' => Action::MoveForward(num),
                _ => panic!(),
            }
        })
        .collect()
}

#[allow(dead_code)]
fn part_one() {
    let input = get_input();

    let mut turn_direction = 90i64;
    let mut position = (0i64, 0i64);

    for action in input {
        match action {
            Action::MoveAbsolute((x, y)) => {
                let (a, b) = position;
                position = (a + x, b + y);
            }
            Action::Turn(t) => {
                turn_direction += t;
                if turn_direction >= 360 {
                    turn_direction -= 360;
                }
                if turn_direction < 0 {
                    turn_direction += 360;
                }
            }
            Action::MoveForward(change) => {
                let (x, y) = position;
                match turn_direction {
                    0 => {
                        position = (x, y + change);
                    }
                    90 => {
                        position = (x + change, y);
                    }
                    180 => {
                        position = (x, y - change);
                    }
                    270 => {
                        position = (x - change, y);
                    }
                    _ => panic!(),
                }
            }
        }
    }

    // dbg!(&position);
    position = (position.0, -position.1);
    dbg!(position);
    dbg!(position.0.abs() + position.1.abs());
}

fn main() {
    let input = get_input();

    let mut ship_position = (0i64, 0i64);
    // let mut turn_direction = 90i64;
    let mut waypoint_position = (10i64, 1i64);

    for action in input {
        match action {
            Action::MoveAbsolute((x, y)) => {
                let (a, b) = waypoint_position;
                waypoint_position = (a + x, b + y);
            }
            Action::Turn(t) => {
                let mut turn_direction = t;
                if turn_direction >= 360 {
                    turn_direction -= 360;
                }
                if turn_direction < 0 {
                    turn_direction += 360;
                }

                let (x, y) = waypoint_position;

                match turn_direction {
                    0 => {
                        waypoint_position = (x, y);
                    }
                    90 => {
                        waypoint_position = (y, -x);
                    }
                    180 => {
                        waypoint_position = (-x, -y);
                    }
                    270 => {
                        waypoint_position = (-y, x);
                    }
                    _ => panic!(),
                }
            }
            Action::MoveForward(change) => {
                for _ in 0..change {
                    let (x, y) = ship_position;
                    let (a, b) = waypoint_position;
                    ship_position = (x + a, y + b);
                }
            }
        }
    }

    // dbg!(&position);
    ship_position = (ship_position.0, -ship_position.1);
    dbg!(ship_position);
    dbg!(ship_position.0.abs() + ship_position.1.abs());
}
