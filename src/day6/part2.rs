// ok i know there's probably some clever math solution here
// that involves incrementing in some smart way, but
// i'm a little more curious about the top -> down cursor approach

use std::{fs::read_to_string, str::Chars};

use itertools::{Itertools, zip_eq};

use crate::day6::common::OperatedGroup;

struct TopDownCursor<'a> {
    lines: Vec<Chars<'a>>,
}

impl<'a> TopDownCursor<'a> {
    fn from(s: &'a str) -> Self {
        let lines = s.split('\n').map(str::chars).collect();

        Self { lines }
    }
}

impl<'a> Iterator for TopDownCursor<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines
            .iter_mut()
            .map(|chars| chars.next())
            .fold_options(String::new(), |mut acc, char| {
                acc.push(char);
                acc
            })
    }
}

pub fn part2() {
    let body = read_to_string("inputs/day6.txt").unwrap();
    let (value_body, operator_body) = body.rsplit_once('\n').unwrap();

    let chunks = TopDownCursor::from(value_body)
        .map(|s| s.trim().to_string())
        .chunk_by(|s| s.is_empty());

    let operands = chunks
        .into_iter()
        .filter_map(|(key, chunk)| if key { None } else { Some(chunk) })
        .map(|group| group.collect_vec());

    let operand_groups = operator_body
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(OperatedGroup::from);

    let sum = zip_eq(operands, operand_groups)
        .map(|(operand, operand_group)| {
            operand
                .iter()
                .map(|s| str::parse::<u64>(s).unwrap())
                .fold(operand_group, |acc, value| acc.apply(value))
        })
        .fold(0, |acc, og| acc + og.value());

    print!("Part 2 answer: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_down_cursor() {
        let s = "123\n456\n789";
        let mut cursor = TopDownCursor::from(s);

        assert_eq!("147", cursor.next().unwrap());
        assert_eq!("258", cursor.next().unwrap());
        assert_eq!("369", cursor.next().unwrap());
        assert!(cursor.next().is_none());
    }
}
