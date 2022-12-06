fn find_marker(s: &str, n: usize) -> Option<usize> {
    assert!(n > 1 && n <= 32);
    for (offset, window) in s.as_bytes().windows(n).enumerate() {
        let letters_in_window: u32 = window
            .iter()
            .copied()
            .map(|it| it - b'a')
            .fold(0, |acc, x| acc | (1 << (x as u32)));

        if letters_in_window.count_ones() == n as u32 {
            return Some(offset + n);
        }
    }

    None
}

pub(crate) fn day_06(input: &str) -> (usize, usize) {
    (
        find_marker(input, 4).unwrap(),
        find_marker(input, 14).unwrap()
    )
}