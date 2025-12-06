use std::fs::read_to_string;

type Bank = Vec<u32>;

fn line_to_bank(s: &str) -> Bank {
    s.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

struct Joltage {
    digits: Vec<u32>,
}

impl Joltage {
    fn from_bank(bank: &mut Bank, size: usize) -> Self {
        let mut digits = Vec::with_capacity(size);
        for _ in 0..size {
            digits.push(bank.pop().unwrap());
        }

        Joltage { digits }
    }

    // trying to add a digit in, means trying to replace the
    // most significant digit
    // if that's not successful, it's done
    // if it is, the value that was previously there will
    // try to do the same process with the values before it
    fn pass_through(&mut self, new_digit: u32) {
        let mut replacer = new_digit;

        // reversing it so we can go in
        // most significant digit -> least significant digit
        // order
        for digit in self.digits.iter_mut().rev() {
            // the value that can be inserted is not larger
            // than the next most significant digit, so we're done
            if replacer < *digit {
                return;
            }

            // the replacer should take the place of this most significant digit
            // and the digit that was there should be checked against the
            // next most significant digit
            std::mem::swap(&mut (*digit), &mut replacer);
        }
    }

    // i know digits will fit into u32, but i don't know if the sum will
    fn as_value(&self) -> u64 {
        let mut total = 0;
        for (i, digit) in self.digits.iter().enumerate() {
            total += *digit as u64 * u64::pow(10, i as u32);
        }

        total
    }
}

fn get_max_joltage_value(mut bank: Vec<u32>, size: usize) -> u64 {
    let mut joltage = Joltage::from_bank(&mut bank, size);
    bank.reverse();

    for digit in bank {
        joltage.pass_through(digit);
    }

    joltage.as_value()
}

pub fn part1() {
    let body = read_to_string("inputs/day3.txt").unwrap();

    let mut total = 0;
    for line in body.split('\n') {
        let bank = line_to_bank(line);
        let value = get_max_joltage_value(bank, 2);

        total += value;
    }

    print!("Part 1 answer: {}", total);
}

pub fn part2() {
    let body = read_to_string("inputs/day3.txt").unwrap();

    let mut total = 0;
    for line in body.split('\n') {
        let bank = line_to_bank(line);
        let value = get_max_joltage_value(bank, 12);

        total += value;
    }

    print!("Part 2 answer: {}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_joltage_takes() {
        let mut bank: Vec<u32> = vec![1, 2, 3, 4, 5];
        let _ = Joltage::from_bank(&mut bank, 2);

        assert_eq!(vec![1, 2, 3], bank);
    }

    #[test]
    fn test_joltage_to_value() {
        let joltage = Joltage::from_bank(&mut vec![1, 2, 3], 3);

        assert_eq!(joltage.as_value(), 123);
    }

    #[test]
    fn test_joltage_pass_through() {
        let mut joltage = Joltage::from_bank(&mut vec![1, 2, 3], 3);
        joltage.pass_through(4);

        assert_eq!(joltage.as_value(), 423);

        let mut joltage = Joltage::from_bank(&mut vec![3, 2, 3], 3);
        joltage.pass_through(4);

        assert_eq!(joltage.as_value(), 433);

        let mut joltage = Joltage::from_bank(&mut vec![3, 3, 5], 3);
        joltage.pass_through(4);

        assert_eq!(joltage.as_value(), 435);
    }

    #[test]
    fn test_get_max_joltage_value() {
        assert_eq!(
            get_max_joltage_value(line_to_bank("987654321111111"), 2),
            98
        );
        assert_eq!(
            get_max_joltage_value(line_to_bank("811111111111119"), 2),
            89
        );
        assert_eq!(
            get_max_joltage_value(line_to_bank("234234234234278"), 2),
            78
        );
        assert_eq!(
            get_max_joltage_value(line_to_bank("818181911112111"), 2),
            92
        );
        assert_eq!(
            get_max_joltage_value(
                line_to_bank(
                    "3722443164324852429541739322454443622742537425744313396455466849784737627295682866595242427454396354"
                ),
                2
            ),
            99
        );
    }
}
