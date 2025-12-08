use std::time::SystemTime;

use structopt::StructOpt;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

// https://stackoverflow.com/questions/71515747/what-is-the-easiest-way-to-time-a-function-call-for-testing-purposes
fn timeit<F: Fn()>(f: F) {
    let start = SystemTime::now();
    f();
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!(" [{}ms]", duration.as_millis());
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short, long)]
    day: i32,
}

fn main() {
    let opt = Opt::from_args();
    match opt.day {
        1 => {
            timeit(day1::part1);
            timeit(day1::part2);
        }
        2 => {
            timeit(day2::part1);
            timeit(day2::part2);
        }
        3 => {
            timeit(day3::part1);
            timeit(day3::part2);
        }
        4 => {
            timeit(day4::part1);
            timeit(day4::part2);
        }
        5 => {
            timeit(day5::part1);
            timeit(day5::part2);
        }
        6 => {
            timeit(day6::part1);
            timeit(day6::part2);
        }
        7 => {
            timeit(day7::part1);
            timeit(day7::part2);
        }
        8 => {
            timeit(day8::part1);
            timeit(day8::part2);
        }
        _ => eprintln!("Invalid day."),
    }
}
