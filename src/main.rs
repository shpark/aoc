use clap::Parser;

use aoc25::y2025;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: u8,

    #[arg(short, long, default_value_t = 1)]
    part: u8,
}

macro_rules! dispatch_day_part {
    ($args:expr; $( $day:literal => $module:ident ),+ $(,)?) => {
        match ($args.day, $args.part) {
            $(
                ($day, 1) => y2025::$module::part1(),
                ($day, 2) => y2025::$module::part2(),
            )+
            _ => panic!("invalid (day, part)"),
        }
    };
}

fn main() {
    let args = Args::parse();

    let res = dispatch_day_part!(
        args;
        1 => day01,
        2 => day02,
        3 => day03,
        4 => day04,
        5 => day05,
        6 => day06,
        7 => day07,
        8 => day08,
        9 => day09,
        10 => day10,
        11 => day11,
        12 => day12,
    );

    println!("{}", res);
}
