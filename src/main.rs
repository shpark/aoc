use clap::Parser;

use aoc25::y2025;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    year: u16,

    #[arg(short, long)]
    day: u8,

    #[arg(short, long, default_value_t = 1)]
    part: u8,
}

macro_rules! aoc_dispatch {
    ($year:tt, $day:expr, $part:expr, max_day = $max:tt) => {
        paste::paste! {
            seq_macro::seq!(D in 01..=$max {
                match ($day, $part) {
                    #((D, 1) => [<y $year>]::day~D::part1(),)*
                    #((D, 2) => [<y $year>]::day~D::part2(),)*
                    _ => panic!("Day {} part {} not implemented", $day, $part),
                }
            })
        }
    };
}

fn main() {
    let args = Args::parse();

    let res = match args.year {
        2025 => aoc_dispatch!(2025, args.day, args.part, max_day = 12),
        _ => panic!("Year {} not implemented", args.year),
    };

    println!("{}", res);
}
