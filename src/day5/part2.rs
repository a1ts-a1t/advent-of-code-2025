use std::fs::read_to_string;

use crate::day5::common::{RangeCollectionBuilder, RangeSegment};

pub fn part2() {
    let body = read_to_string("inputs/day5.txt").unwrap();
    let mut sections = body.split("\n\n");

    let range_segments = sections.next().unwrap().split('\n').map(RangeSegment::from);

    let mut builder = RangeCollectionBuilder::new();
    for range_segment in range_segments {
        builder.insert(range_segment);
    }

    let count = builder
        .into_vec()
        .iter()
        // frejya mentioned
        .fold(0u64, |acc, rs| acc + rs.high - rs.low + 1);

    print!("Part 2 answer: {}", count);
}
