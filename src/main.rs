use clap::Parser;

use aoc25::day1;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: u8,

    #[arg(short, long, default_value_t = 1)]
    part: u8,
}

fn main() {
    let args = Args::parse();

    let res = match (args.day, args.part) {
        (1, 1) => day1::part1(),
        (1, 2) => day1::part2(),
        _ => panic!("invalid (day, part)"),
    };

    println!("{}", res);
}