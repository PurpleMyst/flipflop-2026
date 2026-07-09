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
            line.len() as u16
                * (line.bytes().any(|c| c.is_ascii_lowercase()) as u16
                    + line.bytes().any(|c| c.is_ascii_uppercase()) as u16
                    + line.bytes().any(|c| c.is_ascii_digit()) as u16)
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
        .map(|&big_c| states.iter().map(|state| state.push(big_c).value()).sum::<u16>())
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
    run_length: u8,
    best_run_length: u8,

    color_state: ColorState,

    len: u8,
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

    fn value(self) -> u16 {
        self.len as u16
            * ((self.saw_lower as u16 + self.saw_upper as u16 + self.saw_digit as u16)
                + 7 * ((self.saw_seven && !self.saw_nonseven) as u16)
                + self.best_run_length as u16 * self.best_run_length as u16)
            * (if matches!(self.color_state, ColorState::Green | ColorState::Red | ColorState::Blue) {
                3
            } else {
                1
            })
    }
}
