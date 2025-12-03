use structopt::StructOpt;

mod day1;
mod day2;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short, long)]
    day: i32,
}

fn main() {
    let opt = Opt::from_args();
    match opt.day {
        1 => {
            day1::part1();
            day1::part2();
        },
        2 => {
            day2::part1();
            day2::part2();
        },
        _ => eprintln!("Invalid day."),
    }
}
