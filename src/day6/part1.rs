use std::{collections::VecDeque, fs::read_to_string};

use crate::day6::common::OperatedGroup;

pub fn part1() {
    let body = read_to_string("inputs/day6.txt").unwrap();
    let mut lines = body
        .split('\n')
        .map(|line| {
            line.split(' ')
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>()
        })
        .rev();

    let mut operand_groups: VecDeque<_> = lines
        .next()
        .unwrap()
        .into_iter()
        .map(OperatedGroup::from)
        .collect();

    let flat_values =
        lines.flat_map(|values| values.into_iter().map(|s| str::parse::<u64>(s).unwrap()));

    for value in flat_values {
        let operand_group = operand_groups.pop_front().unwrap().apply(value);
        operand_groups.push_back(operand_group);
    }

    let sum: u64 = operand_groups.iter().fold(0, |acc, og| acc + og.value());

    print!("Part 1 answer: {}", sum);
}
