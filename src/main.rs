#![feature(try_blocks)]
#![feature(iter_array_chunks)]
#![feature(let_chains)]

use std::error::Error;
use std::fmt::{Debug, Display};
use std::str::FromStr;

use crate::day01::day_01;
use crate::day02::day_02;
use crate::day03::day_03;
use crate::day04::day_04;
use crate::day05::day_05;
use crate::day06::day_06;
use crate::day07::day_07;
use crate::day08::day_08;
use crate::day09::day_09;
use crate::day10::day_10;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

fn main() -> Result<(), Box<dyn Error>> {
    for file in std::fs::read_dir("data")? {
        let item = file?;
        let name = item.path();
        let name = name.file_stem().unwrap();
        let Some(name) = name.to_str() else {
            continue;
        };

        let Some((day, name, part1, part2)) = (try {
            let mut parts = name.split('_');
            let day = parts.next()?.parse::<u32>().ok()?;
            let name = parts.next()?;
            let part1 = parts.next()?;
            let part2 = parts.next()?;
            (day, name, part1, part2)
        }) else {
            continue;
        };

        fn run<Output1: Display + FromStr + PartialEq+Debug, Output2: Display + FromStr + PartialEq+Debug>(
            solution: impl Fn(&str) -> (Output1, Output2),
            input: &str,
            (part1, part2): (&str, &str),
        ) {
            let start = std::time::Instant::now();
            let (p1, p2) = solution(&input);
            let elapsed = start.elapsed();
            let part1_ok = p1 == part1.parse::<Output1>().ok().unwrap();
            let part2_ok = p2 == part2.parse::<Output2>().ok().unwrap();

            println!("PART1: {} ({:>10}  vs  {:>10})", if part1_ok { '🟢' } else { '🔴' }, p1, part1);
            println!("PART2: {} ({:>10}  vs  {:>10}) in {:>10?}", if part2_ok { '🟢' } else { '🔴' }, p2, part2, elapsed);
        }

        println!("DAY #{:02} - {}", day, name);
        let data = std::fs::read_to_string(item.path())?;
        match day {
            1 => run(day_01, &data, (part1, part2)),
            2 => run(day_02, &data, (part1, part2)),
            3 => run(day_03, &data, (part1, part2)),
            4 => run(day_04, &data, (part1, part2)),
            5 => run(day_05, &data, (part1, part2)),
            6 => run(day_06, &data, (part1, part2)),
            7 => run(day_07, &data, (part1, part2)),
            8 => run(day_08, &data, (part1, part2)),
            9 => run(day_09, &data, (part1, part2)),
            10 => run(day_10, &data, (part1, part2)),
            _ => unimplemented!(),
        };
        println!();
    }

    Ok(())
}
