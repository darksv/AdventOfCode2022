use std::collections::HashSet;

fn find_marker(s: &str, n: usize) -> Option<usize> {
    for (idx, wnd) in s.as_bytes().windows(n).enumerate() {
        if wnd.iter().collect::<HashSet<_>>().len() == n {
            return Some(idx + n);
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