use crate::day2::common::{inputs, number_of_digits};

fn next_invalid(value: u64) -> u64 {
    // workaround for log stuff not playing nice with 0
    // lol, wgaf
    if value == 0 {
        return 11;
    }

    let nod = number_of_digits(value);

    // the smallest invalid value given a number of digits
    // is 100...100...
    if nod % 2 == 1 {
        let base = u64::pow(10, nod / 2);
        return base + u64::pow(10, nod);
    }

    // the smallest invalid value with the same number of digits
    // potentially builds off of the digits in the first half
    // of the number
    let base = value / u64::pow(10, nod / 2);
    let base_base = base * u64::pow(10, nod / 2) + base;

    if base_base > value {
        return base_base;
    }

    // if the given number is already ge that,
    // then it builds off of one more than the value
    // built by the digits in the first half of the number
    let base1 = base + 1;
    base1 * u64::pow(10, nod / 2) + base1
}

pub fn part1() {
    let mut total: u64 = 0;
    for range in inputs() {
        // starting from one under the start, keep
        // getting the next bigger invalid value until
        // you're out of range
        let mut current = next_invalid(range.start() - 1);
        while range.contains(&current) {
            total += current;
            current = next_invalid(current);
        }
    }

    print!("Part 1 answer: {}", total);
}
