use std::{collections::VecDeque, fmt::Display};

const INPUT: &[u8] = include_bytes!("input.txt");
const WIDTH: usize = line_width(INPUT);
const HEIGHT: usize = line_count(INPUT);
const CELLS: usize = WIDTH * HEIGHT;
const WORDS: usize = CELLS.div_ceil(64);

const fn line_width(input: &[u8]) -> usize {
    let mut width = 0;
    while width < input.len() && input[width] != b'\r' && input[width] != b'\n' {
        width += 1;
    }
    assert!(width != 0, "map must not be empty");
    width
}

const fn line_count(input: &[u8]) -> usize {
    let width = line_width(input);
    let mut rows = 0;
    let mut columns = 0;
    let mut index = 0;
    while index < input.len() {
        match input[index] {
            b'\r' => {}
            b'\n' => {
                assert!(columns == width, "map rows must have equal widths");
                rows += 1;
                columns = 0;
            }
            _ => columns += 1,
        }
        index += 1;
    }
    if columns != 0 {
        assert!(columns == width, "map rows must have equal widths");
        rows += 1;
    }
    rows
}

const MAP: [u8; CELLS] = {
    let mut map = [0; CELLS];
    let mut src = 0;
    let mut dst = 0;
    while src < INPUT.len() {
        let byte = INPUT[src];
        src += 1;
        if byte != b'\r' && byte != b'\n' {
            map[dst] = byte;
            dst += 1;
        }
    }
    assert!(dst == CELLS);
    map
};
const fn find_marker(map: &[u8; CELLS], marker: u8) -> usize {
    let mut found = usize::MAX;
    let mut index = 0;
    while index < CELLS {
        if map[index] == marker {
            assert!(found == usize::MAX, "map marker must be unique");
            found = index;
        }
        index += 1;
    }
    assert!(found != usize::MAX, "map marker is missing");
    found
}

const START: usize = find_marker(&MAP, b'S');
const LETTER_POSITIONS: [[usize; 2]; 26] = {
    let mut positions = [[usize::MAX; 2]; 26];
    let mut idx = 0;
    while idx < CELLS {
        let cell = MAP[idx];
        if cell >= b'A' && cell <= b'Z' {
            positions[(cell - b'A') as usize][0] = idx;
        } else if cell >= b'a' && cell <= b'z' {
            positions[(cell - b'a') as usize][1] = idx;
        }
        idx += 1;
    }
    positions
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Rotation {
    Clockwise,
    CounterClockwise,
}
use Rotation::*;

impl std::ops::Not for Rotation {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Clockwise => CounterClockwise,
            CounterClockwise => Clockwise,
        }
    }
}

#[derive(Clone, Copy)]
struct Seen {
    bits: [u64; WORDS],
}

impl Default for Seen {
    fn default() -> Self {
        Self { bits: [0; WORDS] }
    }
}

impl Seen {
    #[inline]
    fn insert(&mut self, index: usize) -> bool {
        let word = &mut self.bits[index / 64];
        let bit = 1u64 << (index % 64);
        if *word & bit != 0 {
            false
        } else {
            *word |= bit;
            true
        }
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let map = &MAP;

    let mut q = VecDeque::new();
    let mut seen = Seen::default();
    seen.insert(START);
    q.push_back((START, CounterClockwise));
    let mut lights = Vec::new();

    while let Some((index, r)) = q.pop_front() {
        if map[index] == b'*' {
            lights.push((index, r == CounterClockwise));
            continue;
        }

        let x = index % WIDTH;
        for (in_bounds, next) in [
            (index >= WIDTH, index.wrapping_sub(WIDTH)),
            (index + WIDTH < CELLS, index + WIDTH),
            (x != 0, index.wrapping_sub(1)),
            (x + 1 < WIDTH, index + 1),
        ] {
            if in_bounds && matches!(map[next], b'#' | b'*') && seen.insert(next) {
                q.push_back((next, !r));
            }
        }
    }

    lights.sort_unstable_by_key(|&(index, _)| index);
    lights
        .into_iter()
        .fold(0u64, |acc, (_, b)| (acc << 1) | (if b { 1 } else { 0 }))
}

#[inline]
pub fn solve_part2() -> impl Display {
    do_solve(&MAP)
}

fn do_solve(map: &[u8; CELLS]) -> u64 {
    let mut lights = Vec::new();
    let mut q = VecDeque::new();
    let mut seen = Seen::default();
    seen.insert(START);
    q.push_back((START, CounterClockwise));

    while let Some((index, r)) = q.pop_front() {
        if map[index] == b'*' {
            lights.push((index, r == CounterClockwise));
            continue;
        } else if map[index].is_ascii_lowercase() {
            let output = LETTER_POSITIONS[(map[index] - b'a') as usize][0];
            if seen.insert(output) {
                q.push_back((output, !r));
            }
        } else {
            let x = index % WIDTH;
            for (in_bounds, next) in [
                (index >= WIDTH, index.wrapping_sub(WIDTH)),
                (index + WIDTH < CELLS, index + WIDTH),
                (x != 0, index.wrapping_sub(1)),
                (x + 1 < WIDTH, index + 1),
            ] {
                if in_bounds && matches!(map[next], b'#' | b'3' | b'*' | b'a'..=b'z') && seen.insert(next) {
                    q.push_back((next, !r));
                }
            }
        }
    }

    lights.sort_unstable_by_key(|&(index, _)| index);
    lights
        .into_iter()
        .fold(0u64, |acc, (_, b)| (acc << 1) | (if b { 1 } else { 0 }))
}

#[inline]
pub fn solve_part3() -> impl Display {
    let mut map = MAP;

    let mut q = VecDeque::new();
    let mut seen = [0u64; WORDS];
    for [upper_idx, lower_idx] in LETTER_POSITIONS {
        if upper_idx == usize::MAX || MAP[upper_idx] == b'S' {
            continue;
        }

        seen.fill(0);
        let mut len = 1;
        seen[upper_idx / 64] |= 1u64 << (upper_idx % 64);
        q.push_back(upper_idx);
        while let Some(index) = q.pop_front() {
            let x = index % WIDTH;
            for (in_bounds, next) in [
                (index >= WIDTH, index.wrapping_sub(WIDTH)),
                (index + WIDTH < CELLS, index + WIDTH),
                (x != 0, index.wrapping_sub(1)),
                (x + 1 < WIDTH, index + 1),
            ] {
                if in_bounds && matches!(map[next], b'#' | b'3') {
                    let word = &mut seen[next / 64];
                    let bit = 1u64 << (next % 64);
                    if *word & bit == 0 {
                        *word |= bit;
                        len += 1;
                        q.push_back(next);
                    }
                }
            }
        }

        if prime(len - 1) {
            map[upper_idx] = b'.';
            map[lower_idx] = b'.';
        }
    }

    do_solve(&map)
}

pub fn prime(len: usize) -> bool {
    if len < 3 {
        return true;
    }
    if len.is_multiple_of(2) {
        return false;
    }
    (3..)
        .step_by(2)
        .take_while(|&n| n <= len / n)
        .all(|n| !len.is_multiple_of(n))
}
