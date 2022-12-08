use std::collections::HashSet;

pub(crate) fn day_08(input: &str) -> (usize, usize) {
    let map: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let mut visible_trees = HashSet::new();

    let width = map[0].len();
    let height = map.len();

    for i in 0..width {
        visible_trees.insert((i, 0));
        visible_trees.insert((i, height - 1));
    }

    for i in 1..height - 1 {
        visible_trees.insert((0, i));
        visible_trees.insert((width - 1, i));
    }

    for y in 0..map.len() {
        let mut prev = 0;
        for x in 0..width - 1 {
            if map[y][x] > prev {
                visible_trees.insert((x, y));
                prev = map[y][x];
            }
        }

        let mut prev = 0;
        for x in (1..width).rev() {
            if map[y][x] > prev {
                visible_trees.insert((x, y));
                prev = map[y][x];
            }
        }
    }

    for x in 0..width {
        let mut prev = 0;
        for y in 0..height - 1 {
            if map[y][x] > prev {
                visible_trees.insert((x, y));
                prev = map[y][x];
            }
        }

        let mut prev = 0;
        for y in (1..height).rev() {
            if map[y][x] > prev {
                visible_trees.insert((x, y));
                prev = map[y][x];
            }
        }
    }

    let mut max_scenic_score = 0;
    for y in 1..width - 1 {
        for x in 1..height - 1 {
            let mut up = 1;
            for i in 1..(y) {
                up = i;
                if map[y - i][x] >= map[y][x] {
                    break;
                }
            }

            let mut down = 1;
            for i in 1..(height - y) {
                down = i;
                if map[y + i][x] >= map[y][x] {
                    break;
                }
            }

            let mut left = 1;
            for i in 1..(x + 1) {
                left = i;
                if map[y][x - i] >= map[y][x] {
                    break;
                }
            }

            let mut right = 1;
            for i in 1..(width - x) {
                right = i;
                if map[y][x + i] >= map[y][x] {
                    break;
                }
            }

            let scenic_score = up * left * right * down;
            max_scenic_score = scenic_score.max(max_scenic_score);
        }
    }

    (visible_trees.len(), max_scenic_score)
}