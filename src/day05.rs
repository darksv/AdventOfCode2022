use arrayvec::ArrayVec;

pub(crate) fn day_05(input: &str) -> (String, String) {
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