use std::{cmp::Ordering, collections::BinaryHeap};

// this exists for the custom Ord
#[derive(Eq, Debug)]
pub struct RangeSegment {
    pub low: u64,
    pub high: u64,
}

impl From<&str> for RangeSegment {
    fn from(value: &str) -> Self {
        // im just gonna do this in more and more cursed ways
        // who's gonna stop me
        let mut chars = value.trim().chars();
        let low_str: String = chars.by_ref().take_while(|c| *c != '-').collect();

        let high_str: String = chars.take_while(|c| *c != '-').collect();

        RangeSegment {
            low: str::parse::<u64>(&low_str).unwrap(),
            high: str::parse::<u64>(&high_str).unwrap(),
        }
    }
}

impl PartialEq for RangeSegment {
    fn eq(&self, other: &Self) -> bool {
        self.low == other.low && self.high == other.high
    }
}

impl Ord for RangeSegment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.low.cmp(&other.low).reverse() // we want a min heap
    }
}

impl PartialOrd for RangeSegment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl RangeSegment {
    fn new(low: u64, high: u64) -> Self {
        Self { low, high }
    }
}

pub struct RangeCollectionBuilder {
    heap: BinaryHeap<RangeSegment>,
}

impl RangeCollectionBuilder {
    pub fn new() -> Self {
        RangeCollectionBuilder {
            heap: BinaryHeap::new(),
        }
    }

    pub fn insert(&mut self, range_segment: RangeSegment) {
        self.heap.push(range_segment);
    }

    // combine into a bunch of non-overlapping segments
    pub fn into_vec(mut self) -> Vec<RangeSegment> {
        // @frejya - let it be known that this was a fold at one point
        //           i had to change it because aparently BinaryHeap::iter() is not ordered
        let mut acc: Vec<RangeSegment> = Vec::new();
        while let Some(range_segment) = self.heap.pop() {
            if acc.is_empty() {
                acc.push(range_segment);
                continue;
            }

            let last_range = acc.pop().unwrap();

            if range_segment.low > last_range.high + 1 {
                acc.push(last_range);
                acc.push(range_segment);
                continue;
            }

            let low = last_range.low;
            let high = u64::max(last_range.high, range_segment.high);

            acc.push(RangeSegment::new(low, high));
        }

        acc
    }
}
