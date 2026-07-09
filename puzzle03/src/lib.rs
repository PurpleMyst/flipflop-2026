use std::fmt::Display;

use rayon::prelude::*;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    include_str!("input.txt")
        .lines()
        .max_by_key(|line| {
            line.len()
                * (line.bytes().any(|c| c.is_ascii_lowercase()) as usize
                    + line.bytes().any(|c| c.is_ascii_uppercase()) as usize
                    + line.bytes().any(|c| c.is_ascii_digit()) as usize)
        })
        .unwrap()
}

#[inline]
pub fn solve_part2() -> impl Display {
    include_str!("input.txt")
        .lines()
        .max_by_key(|line| State::default().push_many(line.bytes()).value())
        .unwrap()
}

#[inline]
pub fn solve_part3() -> impl Display {
    let states = include_str!("input.txt")
        .par_lines()
        .map(|line| State::default().push_many(line.bytes()))
        .collect::<Vec<_>>();

    b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .into_par_iter()
        .map(|&big_c| states.iter().map(|state| state.push(big_c).value()).sum::<usize>())
        .max()
        .unwrap()
}

#[derive(Default, Clone, Copy)]
struct State {
    saw_lower: bool,
    saw_upper: bool,
    saw_digit: bool,

    saw_seven: bool,
    saw_nonseven: bool,

    run_char: u8,
    run_length: usize,
    best_run_length: usize,

    color_state: ColorState,

    len: usize,
}

#[derive(Default, Clone, Copy)]
enum ColorState {
    #[default]
    Idle,

    G,
    Gr,
    Gre,
    Gree,
    Green,

    R,
    Re,
    Red,

    B,
    Bl,
    Blu,
    Blue,
}

impl State {
    fn push(self, c: u8) -> Self {
        let mut new = self;
        if c.is_ascii_lowercase() {
            new.saw_lower = true;
        } else if c.is_ascii_uppercase() {
            new.saw_upper = true;
        } else if c.is_ascii_digit() {
            new.saw_digit = true;

            if c == b'7' {
                new.saw_seven = true;
            } else {
                new.saw_nonseven = true;
            }
        }

        if c == self.run_char {
            new.run_length += 1;
            if new.run_length >= 3 {
                new.best_run_length = new.best_run_length.max(new.run_length);
            }
        } else {
            new.run_char = c;
            new.run_length = 1;
        }

        new.color_state = match (self.color_state, c) {
            (ColorState::Idle, b'g') => ColorState::G,

            (ColorState::G, b'r') => ColorState::Gr,
            (ColorState::Gr, b'e') => ColorState::Gre,
            (ColorState::Gre, b'e') => ColorState::Gree,
            (ColorState::Gree, b'n') => ColorState::Green,

            (ColorState::Gre, b'd') => ColorState::Red,

            (ColorState::Idle, b'r') => ColorState::R,
            (ColorState::R, b'e') => ColorState::Re,
            (ColorState::Re, b'd') => ColorState::Red,

            (ColorState::Idle, b'b') => ColorState::B,
            (ColorState::B, b'l') => ColorState::Bl,
            (ColorState::Bl, b'u') => ColorState::Blu,
            (ColorState::Blu, b'e') => ColorState::Blue,

            (ColorState::Green, _) | (ColorState::Red, _) | (ColorState::Blue, _) => self.color_state,

            _ => ColorState::Idle,
        };

        new.len += 1;

        new
    }

    fn push_many(self, s: impl IntoIterator<Item = u8>) -> Self {
        s.into_iter().fold(self, |state, c| state.push(c))
    }

    fn value(self) -> usize {
        self.len
            * ((self.saw_lower as usize + self.saw_upper as usize + self.saw_digit as usize)
                + 7 * ((self.saw_seven && !self.saw_nonseven) as usize)
                + self.best_run_length * self.best_run_length)
            * (if matches!(self.color_state, ColorState::Green | ColorState::Red | ColorState::Blue) {
                3
            } else {
                1
            })
    }
}
