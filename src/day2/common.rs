use std::{fs::read_to_string, ops::RangeInclusive};

fn string_to_range(s: &str) -> RangeInclusive<u64> {
    let mut endpoint_strings = s.splitn(2, '-');
    let start = str::parse::<u64>(endpoint_strings.next().unwrap().trim()).unwrap();
    let end = str::parse::<u64>(endpoint_strings.next().unwrap().trim()).unwrap();
    start..=end
}

pub fn inputs() -> Vec<RangeInclusive<u64>> {
    let body = read_to_string("inputs/day2.txt").unwrap();
    body.split(',').map(string_to_range).collect()
}

pub fn number_of_digits(value: u64) -> u32 {
    value.ilog10() + 1
}
