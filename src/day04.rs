use std::ops::RangeInclusive;

pub(crate) fn day_04(input: &str) -> (u32, u32) {
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