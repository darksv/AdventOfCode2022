use std::collections::HashSet;

pub(crate) fn day_09(input: &str) -> (usize, usize) {
    let mut rope = [(0, 0); 10];
    let mut visited1 = HashSet::new();
    let mut visited9 = HashSet::new();

    for line in input.lines() {
        let (dir, steps) = line.split_once(' ').unwrap();
        let steps = steps.parse::<u32>().unwrap();

        let (mx, my): (i32, i32) = match dir {
            "U" => (0, -1),
            "D" => (0, 1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => unimplemented!(),
        };

        for _ in 0..steps {
            rope[0].0 += mx;
            rope[0].1 += my;

            for i in 1..rope.len() {
                let (dx, dy) = (
                    rope[i - 1].0 - rope[i].0,
                    rope[i - 1].1 - rope[i].1,
                );

                match (dx.abs(), dy.abs()) {
                    (0, 0) | (1, 0) | (0, 1) | (1, 1) => {}
                    (2, 0) | (0, 2) | (1, 2) | (2, 1) | (2, 2) => {
                        rope[i].0 += dx.signum();
                        rope[i].1 += dy.signum();
                    }
                    (dx, dy) => panic!("invalid diff {} {}", dx, dy),
                }

                visited1.insert(rope[1]);
                visited9.insert(rope[9]);
            }
        }
    }

    (visited1.len(), visited9.len())
}

#[allow(unused)]
fn print_rope(rope: &[(i32, i32)]) {
    for y in -10..=10 {
        for x in -20..=20 {
            let l = if rope[0] == (x, y) {
                'H'
            } else if let Some(p) = rope.iter().position(|&(px, py)| (px, py) == (x, y)) {
                (b'0' + p as u8) as char
            } else if (x, y) == (0, 0) {
                's'
            } else {
                '.'
            };

            print!("{}", l);
        }
        println!();
    }
}