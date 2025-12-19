use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;

fn min_max<T: Ord + Copy>(p: T, q: T) -> (T, T) {
    if p == T::min(p, q) { (p, q) } else { (q, p) }
}

fn as_point(s: &str) -> (u64, u64) {
    let (x, y) = s.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

// solution adapted from here: https://github.com/sleekmountaincat/aoc2025/blob/main/src/day9/q2.ts
// good god this was taking me too long to do.
// this ends up just being a flood fill and border check with the stipulation
// that the spaces between points are compressed, so iteration is
// less painful
struct Shape {
    x_map: HashMap<u64, usize>,
    y_map: HashMap<u64, usize>,
    grid: Vec<Vec<bool>>,
}

impl From<&Vec<(u64, u64)>> for Shape {
    fn from(points: &Vec<(u64, u64)>) -> Self {
        let xs = points.iter().map(|p| p.0).sorted().dedup().collect_vec();

        let ys = points.iter().map(|p| p.1).sorted().dedup().collect_vec();

        let x_len = xs.len();
        let y_len = ys.len();

        let x_map: HashMap<_, _> = xs.iter().enumerate().map(|(i, x)| (*x, i)).collect();
        let y_map: HashMap<_, _> = ys.iter().enumerate().map(|(i, y)| (*y, i)).collect();

        let mut grid = (0..x_len)
            .map(|_| (0..y_len).map(|_| false).collect_vec())
            .collect_vec();
        let grid_points = points
            .iter()
            .map(|(x, y)| (x_map[x], y_map[y]))
            .inspect(|(x, y)| {
                grid[*x][*y] = true;
            })
            .collect_vec(); // fill grid points

        // fill lines
        for (p1, p2) in grid_points.iter().circular_tuple_windows() {
            if p1.0 == p2.0 {
                // vertical line
                let (y_min, y_max) = min_max(p1.1, p2.1);

                for y in y_min..=y_max {
                    grid[p1.0][y] = true;
                }
            } else if p1.1 == p2.1 {
                // horizontal line
                let (x_min, x_max) = min_max(p1.0, p2.0);

                #[allow(clippy::needless_range_loop)] // this is fine
                for x in x_min..=x_max {
                    grid[x][p1.1] = true;
                }
            } else {
                panic!("Diagonal line");
            }
        }

        // pick initial point to flood fill from
        let p0 = Itertools::cartesian_product(0..x_len, 0..y_len)
            .find(|(x, y)| {
                if grid[*x][*y] {
                    return false; // don't want a point on the border
                }

                let mut border_crosses = 0;
                let mut in_border = false;

                for i in (0..=*x).rev() {
                    let point_i = grid[i][*y];
                    if point_i != in_border {
                        border_crosses += 1;
                    }
                    in_border = point_i;
                }

                border_crosses % 2 != 0
            })
            .unwrap();

        // flood fill
        let mut fill_candidates = vec![p0];
        while let Some((x, y)) = fill_candidates.pop() {
            if grid[x][y] {
                continue;
            }
            grid[x][y] = true;

            if x + 1 < x_len {
                fill_candidates.push((x + 1, y));
            }

            if x > 1 {
                fill_candidates.push((x - 1, y));
            }

            if y + 1 < y_len {
                fill_candidates.push((x, y + 1));
            }

            if y > 0 {
                fill_candidates.push((x, y - 1));
            }
        }

        Self { x_map, y_map, grid }
    }
}

impl Shape {
    fn contains(&self, p1: &(u64, u64), p2: &(u64, u64)) -> bool {
        let (x_min, x_max) = min_max(self.x_map[&p1.0], self.x_map[&p2.0]);
        let (y_min, y_max) = min_max(self.y_map[&p1.1], self.y_map[&p2.1]);

        for x in x_min..=x_max {
            if !(self.grid[x][y_min] && self.grid[x][y_max]) {
                return false;
            }
        }

        for y in y_min..=y_max {
            if !(self.grid[x_min][y] && self.grid[x_max][y]) {
                return false;
            }
        }

        true
    }
}

// naive solution because im out of town rn :)
pub fn part1() {
    let body = read_to_string("inputs/day9.txt").unwrap();
    let max_area = body
        .split('\n')
        .map(as_point)
        .combinations(2)
        .map(|points| {
            let p0 = &points[0];
            let p1 = &points[1];
            let delta_x = u64::abs_diff(p0.0, p1.0) + 1;
            let delta_y = u64::abs_diff(p0.1, p1.1) + 1;
            delta_x * delta_y
        })
        .max()
        .unwrap();

    print!("Part 1 answer: {}", max_area);
}

pub fn part2() {
    let body = read_to_string("inputs/day9.txt").unwrap();
    let points = body.split('\n').map(as_point).collect_vec();

    let shape = Shape::from(&points);

    let (_, _, max_area) = points
        .into_iter()
        .combinations(2)
        .map(|points| {
            let p1 = &points[0];
            let p2 = &points[1];
            let delta_x = u64::abs_diff(p1.0, p2.0) + 1;
            let delta_y = u64::abs_diff(p1.1, p2.1) + 1;
            (*p1, *p2, delta_x * delta_y)
        })
        .sorted_by(|a, b| b.2.cmp(&a.2))
        .find(|(p1, p2, _)| shape.contains(p1, p2))
        .unwrap();

    print!("Part 2 answer: {}", max_area);
}
