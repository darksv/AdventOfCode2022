#![feature(try_blocks)]
#![feature(iter_array_chunks)]

use std::error::Error;
use bstr::ByteSlice;

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
            let part1 = parts.next()?.parse::<u32>().ok()?;
            let part2 = parts.next()?.parse::<u32>().ok()?;
            (day, name, part1, part2)
        }) else {
            continue;
        };

        let data = std::fs::read_to_string(item.path())?;
        let func = match day {
            1 => day_01,
            2 => day_02,
            3 => day_03,
            _ => unimplemented!(),
        };

        let start = std::time::Instant::now();
        let (p1, p2) = func(&data);
        let elapsed = start.elapsed();

        dbg!(day, name, p1, p2, p1 == part1, p2 == part2, elapsed);
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
    fn encode(pack: &[u8]) -> u64 {
        let mut v = 0;
        for byte in pack {
            let val = match byte {
                b'a'..=b'z' => byte - b'a' + 1,
                b'A'..=b'Z' => byte - b'A' + 1 + 26,
                _ => unimplemented!(),
            };
            v |= (1 << val);
        }
        v
    }

    fn decode_index(val: u64) -> u32 {
        63 - val.leading_zeros()
    }

    let mut part1 = 0;
    for line in input.as_bytes().lines() {
        let n = line.len();
        let (a, b) = line.split_at(n / 2);
        let a = encode(a);
        let b = encode(b);
        part1 += decode_index(a & b);
    }

    let mut part2 = 0;
    for chunk in input.as_bytes().lines().array_chunks() {
        let [a, b, c]: [u64; 3] = chunk.map(encode);
        part2 += decode_index(a & b & c);
    }


    (part1, part2)
}