use std::collections::HashMap;

use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (String, String) {
    let jets = parse();
    let chamber = Chamber::new(jets);
    let part1 = part1(chamber.clone());
    let part2 = part2(chamber);
    (part1.to_string(), part2.to_string())
}

fn part1(mut part1: Chamber) -> usize {
    for _ in 0..2022 {
        part1.drop_one();
    }
    part1.height()
}

fn part2(mut part2: Chamber) -> usize {
    let mut cycle_finder = HashMap::new();
    cycle_finder.insert((part2.piecenum, part2.jetnum, 0usize), (0usize, 0usize));

    let mut drops = 0;
    loop {
        part2.drop_one();
        drops += 1;
        let height = part2.height();

        if height < 4 {
            continue;
        }

        let shape = ((part2.rocks[height - 1] as usize) << 24)
            | ((part2.rocks[height - 2] as usize) << 16)
            | ((part2.rocks[height - 3] as usize) << 8)
            | (part2.rocks[height - 4]) as usize;

        if let Some(entry) = cycle_finder.get(&(part2.piecenum, part2.jetnum, shape)) {
            //println!("piece = {}", part2.piecenum);
            //println!("jetnum = {}", part2.jetnum);
            //println!("shape = {}", shape);
            //println!("drops until start of loop = {}", entry.1);
            //println!("height of the tower when the loop started = {}", entry.0);

            let delta_height = height - entry.0;
            let delta_drops = drops - entry.1;
            //println!("There's an increase of {delta_height} rows for every {delta_drops} drops");
            let remaining_drops = Chamber::OUCH - entry.1;
            //println!("There're still {remaining_drops} left to go");

            let needed_drops = remaining_drops / delta_drops;
            let leftover_drops = remaining_drops % delta_drops;
            let integral_height = entry.0 + delta_height * needed_drops;

            for _ in 0..leftover_drops {
                part2.drop_one();
            }

            let leftover_height = part2.height() - height;

            //println!("After {leftover_drops} mmore drops , we added {leftover_height} rows");

            return integral_height + leftover_height;
        } else {
            cycle_finder.insert((part2.piecenum, part2.jetnum, shape), (height, drops));
        }
    }
}

fn parse() -> Vec<char> {
    let lines: Vec<Vec<char>> = read_one_per_line::<String>("./src/day_17/input.txt")
        .unwrap()
        .into_iter()
        .filter(|row| !row.is_empty())
        .map(|s| s.chars().collect())
        .collect();
    lines.first().unwrap().to_owned()
}

#[derive(Default, Clone)]
struct Chamber {
    jets: Vec<char>,
    rocks: Vec<u8>,
    piecenum: usize,
    jetnum: usize,
}

impl Chamber {
    const PIECES: [[u8; 4]; 5] = [
        [0b0000000, 0b0000000, 0b0000000, 0b0011110],
        [0b0000000, 0b0001000, 0b0011100, 0b0001000],
        [0b0000000, 0b0000100, 0b0000100, 0b0011100],
        [0b0010000, 0b0010000, 0b0010000, 0b0010000],
        [0b0000000, 0b0000000, 0b0011000, 0b0011000],
    ];

    const OUCH: usize = 1_000_000_000_000;

    pub fn new(jets: Vec<char>) -> Self {
        Self {
            jets,
            rocks: vec![0, 0, 0, 0, 0, 0, 0],
            piecenum: 0,
            jetnum: 0,
        }
    }

    fn drop_one(&mut self) {
        let mut piece = Self::PIECES[self.piecenum];
        self.piecenum = (self.piecenum + 1) % Self::PIECES.len();

        let mut last = self.rocks.len() - 7;
        while self.rocks[last] != 0 {
            self.rocks.push(0);
            last += 1;
        }

        let mut bottom = self.rocks.len() - 4;

        loop {
            let jet = self.jets[self.jetnum];
            self.jetnum = (self.jetnum + 1) % self.jets.len();

            match jet {
                '<' => {
                    if self.can_go_left(bottom, &piece) {
                        for p in piece.iter_mut() {
                            *p <<= 1;
                        }
                    }
                }
                '>' => {
                    if self.can_go_right(bottom, &piece) {
                        for p in piece.iter_mut() {
                            *p >>= 1;
                        }
                    }
                }
                _ => panic!("bad input '{jet}'"),
            }

            if bottom > 0 && self.can_go_to(bottom - 1, &piece) {
                bottom -= 1;
            } else {
                break;
            }
        }

        let mut prow = 4;
        while prow > 0 {
            prow -= 1;
            self.rocks[bottom] |= piece[prow];
            bottom += 1;
        }
    }

    fn can_go_left(&self, mut bottom: usize, piece: &[u8; 4]) -> bool {
        let mut prow = 4;
        while prow > 0 {
            prow -= 1;
            if (piece[prow] & 0x40) != 0 || (self.rocks[bottom] & (piece[prow] << 1)) != 0 {
                return false;
            }
            bottom += 1;
        }
        true
    }

    fn can_go_right(&self, mut bottom: usize, piece: &[u8; 4]) -> bool {
        let mut prow = 4;
        while prow > 0 {
            prow -= 1;
            if (piece[prow] & 0x01) != 0 || (self.rocks[bottom] & (piece[prow] >> 1)) != 0 {
                return false;
            }
            bottom += 1;
        }
        true
    }

    fn can_go_to(&self, mut bottom: usize, piece: &[u8; 4]) -> bool {
        let mut prow = 4;
        while prow > 0 {
            prow -= 1;
            if (self.rocks[bottom] & piece[prow]) != 0 {
                return false;
            }
            bottom += 1;
        }
        true
    }

    fn height(&self) -> usize {
        let mut top = self.rocks.len();
        while top > 0 && self.rocks[top - 1] == 0 {
            top -= 1;
        }
        top
    }

    fn _print_piece(piece: &[u8; 4]) {
        for p in piece {
            Self::_print_row(*p);
        }
    }

    fn _print_row(row: u8) {
        let mut bit = 0x40;
        while bit > 0 {
            print!("{}", if (bit & row) != 0 { '#' } else { '.' });
            bit >>= 1;
        }
    }

    fn _draw(&self) {
        let mut top = self.rocks.len();
        while top > 0 {
            top -= 1;
            print!("|");
            Self::_print_row(self.rocks[top]);
            println!("|");
        }
        println!("+-------+");
    }
}