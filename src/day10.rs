#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
enum Instr {
    noop,
    addx(i32),
}

const DISPLAY_WIDTH: usize = 40;
const DISPLAY_HEIGHT: usize = 6;

pub(crate) fn day_10(input: &str) -> (i32, String) {
    let code: Vec<_> = input.lines()
        .map(|line| {
            match line.split_once(' ').unwrap_or((line, "")) {
                ("addx", arg) => Instr::addx(arg.parse().unwrap()),
                ("noop", _) => Instr::noop,
                instr => unimplemented!("{:?}", instr),
            }
        })
        .collect();

    let mut cpu = Cpu::new();
    cpu.run(&code);

    (cpu.strength, ocr(&cpu.pixels).into_iter().collect())
}

#[allow(unused)]
fn dump(data: &[bool]) {
    for row in data.chunks_exact(DISPLAY_WIDTH) {
        for col in row {
            print!("{}", if *col { '#' } else { '.' });
        }
        println!();
    }
}

struct Cpu {
    pc: usize,
    cycle: usize,
    reg: i32,

    strength: i32,
    pixels: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
}

impl Cpu {
    fn new() -> Self {
        Self {
            pc: 0,
            cycle: 0,
            reg: 1,
            strength: 0,
            pixels: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
        }
    }

    fn run(&mut self, code: &[Instr]) {
        let mut delayed_value = None;
        for _ in 0..DISPLAY_WIDTH * DISPLAY_HEIGHT {
            self.cycle += 1;

            let col = ((self.cycle - 1) % DISPLAY_WIDTH) as i32;
            let is_lit = (self.reg - 1..=self.reg + 1).contains(&col);
            self.pixels[self.cycle - 1] = is_lit;

            if self.cycle % DISPLAY_WIDTH == DISPLAY_WIDTH / 2 {
                self.strength += (self.cycle as i32 * self.reg);
            }

            if let Some(value) = delayed_value.take() {
                self.reg += value;
            } else {
                match code.get(self.pc) {
                    Some(Instr::addx(x)) => {
                        delayed_value = Some(*x);
                    }
                    Some(Instr::noop) => {}
                    _ => break,
                }
                self.pc += 1;
            }
        }
    }
}

/*
###..#..#..##..####..##....##.###..###..
#..#.#.#..#..#....#.#..#....#.#..#.#..#.
#..#.##...#..#...#..#..#....#.###..#..#.
###..#.#..####..#...####....#.#..#.###..
#.#..#.#..#..#.#....#..#.#..#.#..#.#.#..
#..#.#..#.#..#.####.#..#..##..###..#..#.
 */


fn ocr(data: &[bool]) -> [char; 8] {
    let mut letters = [' '; 8];
    for letter_idx in 0..8 {
        let mut coded = 0u32;
        for y in 0..6 {
            for x in (letter_idx * 5)..(letter_idx * 5 + 4) {
                coded = (coded << 1) | (data[y * DISPLAY_WIDTH + x] as u32);
            }
        }

        let letter = match coded {
            0b1110_1001_1001_1110_1010_1001 => 'R',
            0b1001_1010_1100_1010_1010_1001 => 'K',
            0b0110_1001_1001_1111_1001_1001 => 'A',
            0b1111_0001_0010_0100_1000_1111 => 'Z',
            0b0011_0001_0001_0001_1001_0110 => 'J',
            0b1110_1001_1110_1001_1001_1110 => 'B',
            _ => ' ',
        };

        letters[letter_idx] = letter;
    }

    letters
}