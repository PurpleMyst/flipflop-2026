use std::{fmt::Display, mem::swap};

// My puzzle input only contains b'A'..=b'J'.
const ALPHABET: usize = 10;
const PAIRS: usize = ALPHABET * ALPHABET;
type Count = u64;

#[derive(Clone, Copy, Default)]
struct Rule {
    pairs: [u8; 6],
    len: u8,
}

impl Rule {
    fn new(first: usize, second: usize, line: &str) -> Self {
        let mut pairs = [0; 6];
        let mut previous = first;
        let mut len = 0;
        for (&child, pair) in line.as_bytes()[4..].iter().step_by(2).zip(&mut pairs) {
            let child = (child - b'A') as usize;
            *pair = (previous * ALPHABET + child) as u8;
            previous = child;
            len += 1;
        }
        pairs[len] = (previous * ALPHABET + second) as u8;
        Self {
            pairs,
            len: len as u8 + 1,
        }
    }

    #[inline(always)]
    fn iter(&self) -> impl Iterator<Item = &u8> {
        self.pairs[..self.len as usize].iter()
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let mut rules = [[0; ALPHABET]; ALPHABET];
    for line in include_str!("input.txt").lines() {
        let bytes = line.as_bytes();
        let from = (bytes[0] - b'A') as usize;
        for &to in bytes[2..].iter().step_by(2) {
            rules[from][(to - b'A') as usize] += 1;
        }
    }

    let mut stoats = [0; ALPHABET];
    stoats[0] = 1;
    stoats[1] = 1;
    let mut new_stoats = [0; ALPHABET];

    for _ in 0..7 {
        for (&count, rule) in stoats.iter().zip(rules.iter()) {
            for (next, &amount) in new_stoats.iter_mut().zip(rule) {
                *next += count * amount;
            }
        }
        swap(&mut stoats, &mut new_stoats);
        new_stoats.fill(0);
    }

    stoats.into_iter().sum::<Count>()
}

#[inline]
pub fn solve_part2() -> impl Display {
    do_solve23(7)
}

#[inline]
pub fn solve_part3() -> impl Display {
    do_solve23(21)
}

#[inline]
fn do_solve23(gens: usize) -> Count {
    let mut rules = [Rule::default(); PAIRS];
    for line in include_str!("input.txt").lines() {
        let bytes = line.as_bytes();
        let first = (bytes[0] - b'A') as usize;
        let second = (bytes[2] - b'A') as usize;
        rules[first * ALPHABET + second] = Rule::new(first, second, line);
        rules[second * ALPHABET + first] = Rule::new(second, first, line);
    }

    let mut stoat_pairs = [0; PAIRS];
    stoat_pairs[1] = 1; // (A, B)
    let mut next_stoat_pairs = [0; PAIRS];

    for _ in 0..gens {
        for (&count, rule) in stoat_pairs.iter().zip(rules.iter()) {
            if count == 0 {
                continue;
            }
            for &pair in rule.iter() {
                next_stoat_pairs[pair as usize] += count;
            }
        }
        swap(&mut stoat_pairs, &mut next_stoat_pairs);
        next_stoat_pairs.fill(0);
    }

    stoat_pairs.into_iter().sum::<Count>() + 1
}
