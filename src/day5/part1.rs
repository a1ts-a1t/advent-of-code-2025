use std::{cmp::Ordering, collections::BTreeSet, fs::read_to_string};

use crate::day5::common::{RangeCollectionBuilder, RangeSegment};

#[derive(Debug)]
enum RangeCollectionItem {
    Range(u64, u64),
    Value(u64),
}

impl PartialEq for RangeCollectionItem {
    // i don't actually think this matters
    fn eq(&self, _: &Self) -> bool {
        panic!();
    }
}

impl Eq for RangeCollectionItem {}

impl Ord for RangeCollectionItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (
                RangeCollectionItem::Range(self_low, self_high),
                RangeCollectionItem::Range(other_low, other_high),
            ) => {
                if self_high < other_low {
                    Ordering::Less
                } else if other_high < self_low {
                    Ordering::Greater
                } else {
                    panic!() // we really only care about mutually exclusive ranges
                }
            }
            (RangeCollectionItem::Value(_), RangeCollectionItem::Value(_)) => {
                panic!(); // this shouldn't really happen
            }
            (
                RangeCollectionItem::Range(self_low, self_high),
                RangeCollectionItem::Value(value),
            ) => {
                if self_low <= value && self_high >= value {
                    Ordering::Equal // hehehhehehehehe
                } else if self_high < value {
                    Ordering::Less
                } else if self_low > value {
                    Ordering::Greater
                } else {
                    panic!();
                }
            }
            (
                RangeCollectionItem::Value(value),
                RangeCollectionItem::Range(other_low, other_high),
            ) => {
                if other_low <= value && other_high >= value {
                    Ordering::Equal
                } else if value < other_low {
                    Ordering::Less
                } else if value > other_high {
                    Ordering::Greater
                } else {
                    panic!();
                }
            }
        }
    }
}

impl PartialOrd for RangeCollectionItem {
    // lmao this doesn't matter either
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl RangeCollectionItem {
    fn new_value(value: &u64) -> Self {
        Self::Value(*value)
    }

    fn new_range(low: u64, high: u64) -> Self {
        Self::Range(low, high)
    }
}

struct RangeCollection {
    set: BTreeSet<RangeCollectionItem>,
}

impl RangeCollection {
    fn new() -> Self {
        Self {
            set: BTreeSet::new(),
        }
    }

    fn insert(&mut self, range: RangeSegment) {
        let item = RangeCollectionItem::new_range(range.low, range.high);
        self.set.insert(item);
    }

    fn contains(&self, value: &u64) -> bool {
        self.set.contains(&RangeCollectionItem::new_value(value))
    }
}

impl From<RangeCollectionBuilder> for RangeCollection {
    fn from(value: RangeCollectionBuilder) -> Self {
        let mut range_collection = RangeCollection::new();
        for range in value.into_vec().into_iter() {
            range_collection.insert(range);
        }

        range_collection
    }
}

pub fn part1() {
    let body = read_to_string("inputs/day5.txt").unwrap();
    let mut sections = body.split("\n\n");

    let range_segments = sections.next().unwrap().split('\n').map(RangeSegment::from);

    let mut builder = RangeCollectionBuilder::new();
    for range_segment in range_segments {
        builder.insert(range_segment);
    }

    let range_collection = RangeCollection::from(builder);
    let count = sections
        .next()
        .unwrap()
        .split('\n')
        .map(|s| str::parse::<u64>(s).unwrap())
        .filter(|value| range_collection.contains(value))
        .count();

    print!("Part 1 answer: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_collection() {
        let mut builder = RangeCollectionBuilder::new();
        builder.insert(RangeSegment::from("10-20"));
        let range_collection = RangeCollection::from(builder);

        assert!(range_collection.contains(&15));
        assert!(!range_collection.contains(&5));
    }
}
