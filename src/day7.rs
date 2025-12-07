use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

struct ManifoldRow {
    entry_indices: HashSet<usize>,
    splitter_indices: HashSet<usize>,
}

impl From<&str> for ManifoldRow {
    fn from(value: &str) -> Self {
        let mut entry_indices = HashSet::new();
        let mut splitter_indices = HashSet::new();

        for (idx, char) in value.char_indices() {
            match char {
                'S' => {
                    entry_indices.insert(idx);
                }
                '^' => {
                    splitter_indices.insert(idx);
                }
                _ => (),
            }
        }

        Self {
            entry_indices,
            splitter_indices,
        }
    }
}

impl ManifoldRow {
    // i realize that this is probably more efficiently done by mutating
    // a single set many times, but i wanted to try something
    // slightly more functional. sue me
    fn apply_beams(self, beam_indices: HashSet<usize>) -> (HashSet<usize>, usize) {
        let splits = beam_indices.intersection(&self.splitter_indices);

        // deeply unserious way to do this
        // gets around the fact that Iterator::count consumes itself
        let (split_beam_indices, split_count) =
            splits.fold((Vec::new(), 0), |(mut acc_vec, acc_count), idx| {
                acc_vec.push(idx - 1);
                acc_vec.push(idx + 1);
                (acc_vec, acc_count + 1)
            });

        let beam_indices: HashSet<_> = beam_indices
            .difference(&self.splitter_indices)
            .cloned() // every previous beam that wasn't split
            .chain(split_beam_indices)
            .chain(self.entry_indices) // add in new beams
            .collect();

        (beam_indices, split_count)
    }

    // timelines[beam index] -> the number of timelines at that index
    fn apply_timelines(self, timelines: &mut HashMap<usize, u64>) {
        for splitter in self.splitter_indices {
            if let Some(c) = timelines.remove(&splitter) {
                timelines
                    .entry(splitter - 1)
                    .and_modify(|e| *e += c)
                    .or_insert(c);
                timelines
                    .entry(splitter + 1)
                    .and_modify(|e| *e += c)
                    .or_insert(c);
            }
        }

        for entry in self.entry_indices {
            timelines
                .entry(entry)
                // this shouldn't happen but it's here for completeness
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    }
}

pub fn part1() {
    let body = read_to_string("inputs/day7.txt").unwrap();
    let lines = body.split('\n');

    let (_, count) = lines.map(ManifoldRow::from).fold(
        (HashSet::new(), 0),
        |(beam_indices, count), manifold_row| {
            let (b, c) = manifold_row.apply_beams(beam_indices);
            (b, count + c)
        },
    );

    print!("Part 1 answer: {}", count);
}

pub fn part2() {
    let body = read_to_string("inputs/day7.txt").unwrap();
    let lines = body.split('\n');

    let mut timelines = HashMap::new();
    for manifold_row in lines.map(ManifoldRow::from) {
        manifold_row.apply_timelines(&mut timelines);
    }

    print!(
        "Part 2 answer: {}",
        timelines.values().sum::<u64>()
    );
}
