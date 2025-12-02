use std::fs::read_to_string;

enum RotationDirection {
    Left,
    Right,
}

struct Rotation {
    direction: RotationDirection,
    amount: i32,
}

impl From<&str> for Rotation {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();

        let dir_string: String = chars.by_ref().take(1).collect();
        let direction = match dir_string.as_str() {
            "L" => RotationDirection::Left,
            "R" => RotationDirection::Right,
            _ => panic!("{}", value),
        };

        let amount_string: String = chars.collect();
        let amount = amount_string.parse::<i32>().unwrap();

        Rotation { direction, amount }
    }
}

pub fn part1() {
    let body = read_to_string("inputs/day1.txt").unwrap();
    let lines: Vec<&str> = body.split('\n').collect();
    let rotations: Vec<Rotation> = lines.into_iter().map(Rotation::from).collect();

    let mut zero_count = 0;
    let mut current_position = 50;
    for rotation in rotations {
        match rotation.direction {
            RotationDirection::Left => {
                current_position = (current_position - rotation.amount).rem_euclid(100);
            }
            RotationDirection::Right => {
                current_position = (current_position + rotation.amount).rem_euclid(100);
            }
        }

        if current_position == 0 {
            zero_count += 1;
        }
    }

    println!("Part 1 answer: {}", zero_count);
}

pub fn part2() {
    let body = read_to_string("inputs/day1.txt").unwrap();
    let lines: Vec<&str> = body.split('\n').collect();
    let rotations: Vec<Rotation> = lines.into_iter().map(Rotation::from).collect();

    let mut zero_count = 0;
    let mut current_position = 50;
    for rotation in rotations {
        zero_count += rotation.amount.div_euclid(100);
        let normalized_amount = rotation.amount.rem_euclid(100);

        // we've done all the clicking past 0 we can
        // if we don't cut it off here, this'll just reclick when we don't want to
        if normalized_amount == 0 {
            continue;
        }

        let position = match rotation.direction {
            RotationDirection::Left => {
                let position = current_position - normalized_amount;

                // if the current position is already 0, adding a count
                // would count both landing on zero and going off of zero
                // so we skip it
                if position <= 0 && current_position != 0 {
                    zero_count += 1;
                }

                position
            }
            RotationDirection::Right => {
                let position = current_position + normalized_amount;
                if position >= 100 && current_position != 0 {
                    zero_count += 1;
                }

                position
            }
        };

        current_position = position.rem_euclid(100);
    }

    println!("Part 2 answer: {}", zero_count);
}
