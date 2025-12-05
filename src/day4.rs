use std::fs::read_to_string;

use itertools::Itertools;

type Vec2D<T> = Vec<Vec<T>>;

fn cols(vec2d: &Vec2D<i32>) -> usize {
    vec2d[0].len()
}

fn rows(vec2d: &Vec2D<i32>) -> usize {
    vec2d.len()
}

fn string_to_vec(s: &str) -> Vec<i32> {
    s.chars()
        .map(|c| match c {
            '@' => 1,
            '.' => 0,
            _ => panic!(),
        })
        .collect()
}

fn oaconvolve_ij(window: &Vec2D<i32>, kernel: &Vec2D<i32>, i: usize, j: usize) -> i32 {
    let mut sum = 0;
    for (ix, jx) in Itertools::cartesian_product(0..rows(kernel), 0..cols(kernel)) {
        let h = kernel[ix][jx];
        let x = if (i as i32) - (ix as i32) < 0
            || i - ix >= rows(window)
            || (j as i32) - (jx as i32) < 0
            || j - jx >= cols(window)
        {
            0
        } else {
            window[i - ix][j - jx]
        };
        sum += h * x;
    }

    sum
}

// correspondes to oaconvolve with mode='full' in scipy.signal
// here, we're assuming that the kernel is smaller than the window in each dimension
fn overlap_add_convolve(window: &Vec2D<i32>, kernel: &Vec2D<i32>) -> Vec2D<i32> {
    let rows = rows(window) + rows(kernel) - 1;
    let cols = cols(window) + cols(kernel) - 1;

    let mut convolution: Vec2D<i32> = Vec::new();
    for ni in 0..rows {
        let mut row: Vec<i32> = Vec::new();
        for nj in 0..cols {
            row.push(oaconvolve_ij(window, kernel, ni, nj));
        }
        convolution.push(row);
    }

    convolution
}

pub fn part1() {
    let body = read_to_string("inputs/day4.txt").unwrap();
    let window: Vec2D<i32> = body.split('\n').map(string_to_vec).collect();
    let kernel: Vec2D<i32> = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    let convolution = overlap_add_convolve(&window, &kernel);

    let mut count = 0;
    for (i, j) in Itertools::cartesian_product(0..rows(&window), 0..cols(&window)) {
        // since the full convolution pads out, we only want the inside bit of it
        if window[i][j] == 1 && convolution[i + 1][j + 1] < 4 {
            count += 1;
        }
    }

    print!("Part 1 answer: {}", count);
}

pub fn part2() {
    let body = read_to_string("inputs/day4.txt").unwrap();
    let mut window: Vec2D<i32> = body.split('\n').map(string_to_vec).collect();
    let kernel: Vec2D<i32> = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];

    let mut count = 0;
    loop {
        let mut replacement_count = 0;
        let convolution = overlap_add_convolve(&window, &kernel);

        for (i, j) in Itertools::cartesian_product(0..rows(&window), 0..cols(&window)) {
            if window[i][j] == 0 || convolution[i + 1][j + 1] >= 4 {
                continue;
            }

            count += 1;
            replacement_count += 1;
            window[i][j] = 0;
        }

        if replacement_count == 0 {
            break;
        }
    }

    print!("Part 2 answer: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlap_add_convolve() {
        let window = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let kernel = vec![vec![1, 2], vec![3, 4]];

        let expected = vec![
            vec![1, 4, 7, 6],
            vec![7, 23, 33, 24],
            vec![19, 53, 63, 42],
            vec![21, 52, 59, 36],
        ];

        assert_eq!(expected, overlap_add_convolve(&window, &kernel));
    }
}
