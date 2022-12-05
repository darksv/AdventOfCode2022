pub(crate) fn day_02(input: &str) -> (u32, u32) {
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
