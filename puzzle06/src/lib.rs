use std::{collections::VecDeque, fmt::Display};

const WIDTH: usize = 200;
const HEIGHT: usize = WIDTH / 2;
const CELLS: usize = WIDTH * HEIGHT;
const WORDS: usize = CELLS.div_ceil(64);

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
    len: usize,
}

impl Default for Seen {
    fn default() -> Self {
        Self {
            bits: [0; WORDS],
            len: 0,
        }
    }
}

impl Seen {
    #[inline]
    fn insert(&mut self, y: usize, x: usize) -> bool {
        let index = y * WIDTH + x;
        let word = &mut self.bits[index / 64];
        let bit = 1u64 << (index % 64);
        if *word & bit != 0 {
            false
        } else {
            *word |= bit;
            self.len += 1;
            true
        }
    }
}

#[inline]
fn letter_positions(map: &[u8]) -> [usize; 26] {
    let mut positions = [usize::MAX; 26];
    for (idx, &cell) in map.iter().enumerate() {
        if cell.is_ascii_uppercase() {
            positions[(cell - b'A') as usize] = idx;
        }
    }
    positions
}

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let map = include_str!("input.txt")
        .lines()
        .flat_map(|s| s.trim().bytes())
        .collect::<Vec<u8>>();

    let start_idx = memchr::memchr(b'S', &map).unwrap();
    let mut q = VecDeque::new();
    q.push_back((start_idx / WIDTH, start_idx % WIDTH, CounterClockwise));
    let mut state = Seen::default();
    let mut lights = Vec::new();

    while let Some((y, x, r)) = q.pop_front() {
        if !state.insert(y, x) {
            continue;
        }

        if map[y * WIDTH + x] == b'*' {
            lights.push((y, x, r == CounterClockwise));
            continue;
        }

        q.extend(
            [
                (y.wrapping_sub(1), x),
                (y.wrapping_add(1), x),
                (y, x.wrapping_sub(1)),
                (y, x.wrapping_add(1)),
            ]
            .into_iter()
            .filter(|&(ny, nx)| ny < HEIGHT && nx < WIDTH)
            .filter(|&(ny, nx)| matches!(map[ny * WIDTH + nx], b'#' | b'*'))
            .map(|(ny, nx)| (ny, nx, !r)),
        )
    }

    lights.sort_unstable_by_key(|&(y, x, _)| (y, x));
    lights
        .into_iter()
        .fold(0u64, |acc, (_, _, b)| (acc << 1) | (if b { 1 } else { 0 }))
}

#[inline]
pub fn solve_part2() -> impl Display {
    let map = include_str!("input.txt")
        .lines()
        .flat_map(|s| s.trim().bytes())
        .collect::<Vec<u8>>();
    do_solve(&map)
}

fn do_solve(map: &[u8]) -> u64 {
    let mut lights = Vec::new();
    let start_idx = memchr::memchr(b'S', map).unwrap();
    let positions = letter_positions(map);
    let mut q = VecDeque::new();
    q.push_back((start_idx / WIDTH, start_idx % WIDTH, CounterClockwise));
    let mut state = [None; CELLS];

    while let Some((y, x, r)) = q.pop_front() {
        let index = y * WIDTH + x;
        if state[index].is_some() {
            continue;
        }
        state[index] = Some(r);

        if map[index] == b'*' {
            lights.push((y, x, r == CounterClockwise));
            continue;
        } else if map[index].is_ascii_lowercase() {
            let output_idx = positions[(map[index] - b'a') as usize];
            let output_y = output_idx / WIDTH;
            let output_x = output_idx % WIDTH;
            q.push_back((output_y, output_x, !r));
        } else {
            q.extend(
                [
                    (y.wrapping_sub(1), x),
                    (y.wrapping_add(1), x),
                    (y, x.wrapping_sub(1)),
                    (y, x.wrapping_add(1)),
                ]
                .into_iter()
                .filter(|&(ny, nx)| ny < HEIGHT && nx < WIDTH)
                .filter(|&(ny, nx)| matches!(map[ny * WIDTH + nx], b'#' | b'3' | b'*' | b'a'..=b'z'))
                .map(|(ny, nx)| (ny, nx, !r)),
            )
        }
    }

    lights.sort_unstable_by_key(|&(y, x, _)| (y, x));
    lights
        .into_iter()
        .fold(0u64, |acc, (_, _, b)| (acc << 1) | (if b { 1 } else { 0 }))
}

#[inline]
pub fn solve_part3() -> impl Display {
    let mut map = include_str!("input.txt")
        .lines()
        .flat_map(|s| s.trim().bytes())
        .collect::<Vec<u8>>();

    let mut boot = Vec::new();
    map.iter().enumerate().for_each(|(idx, &cell)| {
        if !cell.is_ascii_uppercase() || cell == b'S' {
            return;
        }

        let y = idx / WIDTH;
        let x = idx % WIDTH;

        let mut q = VecDeque::new();
        let mut v = Seen::default();
        q.push_back((y, x));
        while let Some((y, x)) = q.pop_front() {
            if !v.insert(y, x) {
                continue;
            }

            q.extend(
                [
                    (y.wrapping_sub(1), x),
                    (y.wrapping_add(1), x),
                    (y, x.wrapping_sub(1)),
                    (y, x.wrapping_add(1)),
                ]
                .into_iter()
                .filter(|&(ny, nx)| ny < HEIGHT && nx < WIDTH)
                .filter(|&(ny, nx)| matches!(map[ny * WIDTH + nx], b'#' | b'3')),
            )
        }

        if prime(v.len - 1) {
            boot.push(cell);
        }
    });
    map.iter_mut().for_each(|c| {
        if c.is_ascii_alphabetic() && boot.contains(&c.to_ascii_uppercase()) {
            *c = b'.';
        }
    });

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
