pub(crate) fn day_03(input: &str) -> (u32, u32) {
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
