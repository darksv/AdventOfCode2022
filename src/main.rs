#![feature(try_blocks)]
#![feature(iter_array_chunks)]

use std::error::Error;
use std::fmt::Display;
use std::ops::RangeInclusive;
use std::str::FromStr;
use arrayvec::ArrayVec;

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

        fn run<Output: Display + FromStr + PartialEq>(
            solution: impl Fn(&str) -> (Output, Output),
            input: &str,
            (part1, part2): (&str, &str),
        ) {
            let start = std::time::Instant::now();
            let (p1, p2) = solution(&input);
            let elapsed = start.elapsed();
            let part1_ok = p1 == part1.parse::<Output>().ok().unwrap();
            let part2_ok = p2 == part2.parse::<Output>().ok().unwrap();

            println!("PART1: {} ({:>10}  vs  {:>10})", if part1_ok { '🟢' } else { '🔴' }, p1, part1);
            println!("PART2: {} ({:>10}  vs  {:>10}) in {:>10?}", if part2_ok { '🟢' } else { '🔴' }, p1, part2, elapsed);
        }

        println!("DAY #{:02} - {}", day, name);
        let data = std::fs::read_to_string(item.path())?;
        match day {
            1 => run(day_01, &data, (part1, part2)),
            2 => run(day_02, &data, (part1, part2)),
            3 => run(day_03, &data, (part1, part2)),
            4 => run(day_04, &data, (part1, part2)),
            5 => run(day_05, &data, (part1, part2)),
            _ => unimplemented!(),
        };
        println!();
    }

    Ok(())
}

fn day_01(input: &str) -> (u32, u32) {
    let mut sums = [0; 3];
    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            if sum > sums[0] {
                sums[0] = sum;
                sums.sort_unstable();
            }
            sum = 0;
        } else {
            let cal: u32 = line.parse().unwrap();
            sum += cal;
        }
    }

    if sum > sums[0] {
        sums[0] = sum;
        sums.sort_unstable();
    }


    (sums[2], sums[0] + sums[1] + sums[2])
}

fn day_02(input: &str) -> (u32, u32) {
    let mut part1 = 0;
    let mut part2 = 0;

    for line in input.as_bytes().split(|b| *b == b'\n') {
        let (p1, p2) = match line {
            b"A X" => (3 + 1, 3 + 0),
            b"A Y" => (2 + 6, 1 + 3),
            b"A Z" => (3 + 0, 2 + 6),
            b"B X" => (1 + 0, 1 + 0),
            b"B Y" => (3 + 2, 2 + 3),
            b"B Z" => (3 + 6, 3 + 6),
            b"C X" => (1 + 6, 2 + 0),
            b"C Y" => (2 + 0, 3 + 3),
            b"C Z" => (3 + 3, 1 + 6),
            _ => unimplemented!(),
        };

        part1 += p1;
        part2 += p2;
    }

    (part1, part2)
}


fn day_03(input: &str) -> (u32, u32) {
    fn encode(pack: &str) -> u64 {
        let mut v = 0;
        for byte in pack.bytes() {
            let val = match byte {
                b'a'..=b'z' => byte - b'a' + 1,
                b'A'..=b'Z' => byte - b'A' + 1 + 26,
                _ => unimplemented!(),
            };
            v |= 1 << val;
        }
        v
    }

    fn decode_index(val: u64) -> u32 {
        debug_assert!(val.leading_zeros() + val.trailing_zeros() + 1 == u64::BITS);
        63 - val.leading_zeros()
    }

    let mut part1 = 0;
    for line in input.lines() {
        let n = line.len();
        let (a, b) = line.split_at(n / 2);
        let a = encode(a);
        let b = encode(b);
        part1 += decode_index(a & b);
    }

    let part2 = input
        .lines()
        .map(encode)
        .array_chunks()
        .map(|[a, b, c]| decode_index(a & b & c))
        .sum();

    (part1, part2)
}

fn day_04(input: &str) -> (u32, u32) {
    fn parse_range(elf: &str) -> Option<RangeInclusive<u8>> {
        let (start, end) = elf.split_once('-')?;
        Some(start.parse().ok()?..=end.parse().ok()?)
    }

    let mut fully_overlap = 0;
    let mut partially_overlap = 0;
    for line in input.lines() {
        let (elf1, elf2) = line.split_once(',').unwrap();
        let range1 = parse_range(elf1).unwrap();
        let range2 = parse_range(elf2).unwrap();

        let overlap_s2 = range1.contains(range2.start());
        let overlap_e2 = range1.contains(range2.end());
        let overlap_s1 = range2.contains(range1.start());
        let overlap_e1 = range2.contains(range1.end());

        if overlap_s2 && overlap_e2 || overlap_s1 && overlap_e1 {
            fully_overlap += 1;
        } else if overlap_s2 || overlap_e2 || overlap_s1 || overlap_e1 {
            partially_overlap += 1;
        }
    }
    (fully_overlap, fully_overlap + partially_overlap)
}

fn day_05(input: &str) -> (String, String) {
    let mut stacks: Vec<Vec<char>> = vec![];

    let mut lines = input.lines();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let num_stacks = (line.len() + 1) / 4;

        if stacks.len() == 0 {
            for _ in 0..num_stacks {
                stacks.push(Vec::new());
            }
        }
        for i in 0..num_stacks {
            let item = match line.as_bytes()[1 + i * 4] as char {
                item @ 'A'..='Z' => item,
                ' ' => continue,
                '0'..='9' => continue,
                other => unimplemented!("{}", other),
            };
            stacks[i].push(item);
        }
    }

    stacks.iter_mut().for_each(|s| s.reverse());

    let mut stacks1 = stacks;
    let mut stacks2 = stacks1.clone();

    for line in lines {
        let parts: ArrayVec<_, 6> = line.split(' ').collect();
        let count: usize = parts[1].parse().unwrap();
        let src: usize = parts[3].parse().unwrap();
        let dst: usize = parts[5].parse().unwrap();

        for _ in 0..count {
            let item = stacks1[src - 1].pop().unwrap();
            stacks1[dst - 1].push(item);
        }

        for _ in 0..count {
            let item = stacks2[src - 1].pop().unwrap();
            stacks2[dst - 1].push(item);
        }

        let offset = stacks2[dst - 1].len() - count;
        stacks2[dst - 1][offset..].reverse();
    }

    (
        stacks1.iter().map(|it| it.last()).flatten().collect(),
        stacks2.iter().map(|it| it.last()).flatten().collect(),
    )
}