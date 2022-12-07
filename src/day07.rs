#[derive(Debug)]
enum Entry<'i> {
    File(&'i str, usize),
    Directory(&'i str, Vec<Entry<'i>>),
}

impl<'i> Entry<'i> {
    fn is_dir(&self) -> bool {
        matches!(self, Entry::Directory(..))
    }

    fn name(&self) -> &str {
        match self {
            Entry::File(s, _) => s,
            Entry::Directory(s, _) => s,
        }
    }

    fn size(&self) -> usize {
        match self {
            Entry::File(_, size) => *size,
            Entry::Directory(_, entries) => {
                entries.iter().map(|it| it.size()).sum()
            }
        }
    }

    fn entries_mut(&mut self) -> Option<&mut Vec<Entry<'i>>> {
        match self {
            Entry::File(_, _) => None,
            Entry::Directory(_, entries) => Some(entries)
        }
    }

    fn entries(&self) -> &[Entry<'i>] {
        match self {
            Entry::File(_, _) => &[],
            Entry::Directory(_, entries) => entries,
        }
    }

    fn iter(&self) -> Box<dyn Iterator<Item=&Entry> + '_> {
        Box::new(std::iter::once(self)
            .chain(self.entries().iter().flat_map(|it| it.iter())))
    }
}

fn infer_filesystem_structure(input: &str) -> Entry<'_> {
    let mut cwd = Vec::new();
    let mut tree = Entry::Directory("/", Vec::new());

    let mut iter = input.lines().peekable();
    while let Some(line) = iter.by_ref().next() {
        let Some(command) = line.strip_prefix("$ ") else {
            continue;
        };

        let (cmd, arg) = match command.split_once(' ') {
            Some((cmd, arg)) => (cmd, Some(arg)),
            None => (command, None),
        };

        match (cmd, arg) {
            ("cd", Some("/")) => cwd.clear(),
            ("cd", Some("..")) => {
                cwd.pop();
            }
            ("cd", Some(name)) => cwd.push(name),
            ("ls", None) => {
                let mut items = Vec::new();

                while let Some(line) = iter.next_if(|line| !line.starts_with("$")) {
                    // Process command output

                    match line.split_once(' ') {
                        Some(("dir", name)) => {
                            items.push(Entry::Directory(name, Vec::new()));
                        }
                        Some((size, name)) => {
                            items.push(Entry::File(name, size.parse::<usize>().unwrap()));
                        }
                        _ => {}
                    }
                }

                let mut current = &mut tree;
                for path in &cwd {
                    match current {
                        Entry::Directory(_, children) => {
                            current = children.iter_mut().find(|entry| entry.name() == *path).unwrap();
                        }
                        Entry::File(_, _) => panic!("must be a directory"),
                    }
                }

                *current.entries_mut().unwrap() = items;
            }
            _ => unimplemented!("unsupported {:?}", (cmd, arg))
        }
    }

    tree
}

pub(crate) fn day_07(input: &str) -> (usize, usize) {
    let tree = infer_filesystem_structure(&input);

    let part1 = tree.iter()
        .filter(|e| e.is_dir())
        .map(|e| e.size())
        .filter(|&size| size <= 100000)
        .sum();

    let part2 = {
        let total = 70000000;
        let used = tree.size();
        let required = 30000000;
        let to_cleanup = used - (total - required);

        let mut candidates: Vec<_> = tree.iter()
            .filter(|e| e.is_dir())
            .map(|e| e.size())
            .collect();
        candidates.sort_unstable();
        candidates.iter().find(|&size| *size >= to_cleanup).copied().unwrap()
    };

    (part1, part2)
}