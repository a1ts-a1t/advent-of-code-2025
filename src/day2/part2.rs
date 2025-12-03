use crate::day2::common::{inputs, number_of_digits};

// this is only ever running on the number of digits (n < 21 for u64)
// so it's nothing fancy
// realisically i could amortize this somehow but idrc
// this is ordered from greatest to least
fn divisors(value: u32) -> Vec<u32> {
    let mut low: Vec<u32> = vec![];
    let mut high: Vec<u32> = vec![];
    for i in 1..=((value as f64).sqrt().floor() as u32) {
        if value % i == 0 {
            low.push(i);
            high.push(value / i);
        }
    }

    low.reverse();
    high.append(&mut low);

    high
}

// takes digits and repeats them
// repeat_digitis(1234, 2) == 12341234
// i just don't wanna do string concatenation
fn repeat_digits(value: u64, times: u32) -> u64 {
    let nod = number_of_digits(value);
    let multiplier = u64::pow(10, nod);
    let mut total = 0;
    for _ in 0..times {
        total = value + multiplier * total;
    }

    total
}

fn is_invalid(value: u64) -> bool {
    let nod = number_of_digits(value);
    if nod == 1 {
        return false;
    }

    let mut divisors = divisors(nod).into_iter();
    divisors.next(); // don't care about the divisor that is itself

    for divisor in divisors {
        let base = value / u64::pow(10, nod - divisor);
        if repeat_digits(base, nod / divisor) == value {
            return true;
        }
    }

    false
}

pub fn part2() {
    let mut total: u64 = 0;
    for range in inputs() {
        for id in range {
            if is_invalid(id) {
                total += id;
            }
        }
    }

    print!("Part 2 answer: {}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_divisors() {
        assert_eq!(vec![24, 12, 8, 6, 4, 3, 2, 1], divisors(24));
    }

    #[test]
    fn test_repeat_digits() {
        assert_eq!(123412341234, repeat_digits(1234, 3));
    }

    #[test]
    fn test_is_invalid() {
        assert!(is_invalid(474474474));
    }
}
