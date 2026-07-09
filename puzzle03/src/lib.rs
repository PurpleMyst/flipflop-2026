use std::fmt::Display;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    include_str!("input.txt")
        .lines()
        .max_by_key(|line| {
            let flags = line.bytes().fold(0, |flags, c| flags | BYTE_FLAGS[c as usize]);

            line.len() as u16 * (flags & CLASS_MASK).count_ones() as u16
        })
        .unwrap()
}

#[inline]
pub fn solve_part2() -> impl Display {
    include_str!("input.txt")
        .lines()
        .max_by_key(|line| State::from_line(line).value())
        .unwrap()
}

#[inline]
pub fn solve_part3() -> impl Display {
    let mut states = Vec::with_capacity(100);
    for line in include_str!("input.txt").lines() {
        states.push(State::from_line(line));
    }

    b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .iter()
        .map(|&big_c| states.iter().map(|state| state.value_after_push(big_c)).sum::<u16>())
        .max()
        .unwrap()
}

#[derive(Default, Clone, Copy)]
struct State {
    flags: u8,
    run_char: u8,
    run_length: u8,
    best_run_length: u8,
    color_state: ColorState,
    len: u8,
}

const SAW_LOWER: u8 = 1 << 0;
const SAW_UPPER: u8 = 1 << 1;
const SAW_DIGIT: u8 = 1 << 2;
const SAW_SEVEN: u8 = 1 << 3;
const SAW_NONSEVEN: u8 = 1 << 4;
const CLASS_MASK: u8 = SAW_LOWER | SAW_UPPER | SAW_DIGIT;
const FLAG_MASK: u8 = CLASS_MASK | SAW_SEVEN | SAW_NONSEVEN;
const BYTE_FLAGS: [u8; 256] = {
    let mut flags = [0; 256];
    let mut c = b'0';
    while c <= b'9' {
        flags[c as usize] = SAW_DIGIT | SAW_NONSEVEN;
        c += 1;
    }
    flags[b'7' as usize] = SAW_DIGIT | SAW_SEVEN;

    c = b'A';
    while c <= b'Z' {
        flags[c as usize] = SAW_UPPER;
        c += 1;
    }

    c = b'a';
    while c <= b'z' {
        flags[c as usize] = SAW_LOWER;
        c += 1;
    }

    flags
};
const FLAG_SCORE: [u16; 32] = {
    let mut scores = [0; 32];
    let mut flags = 0;
    while flags < 32 {
        let flags_u8 = flags as u8;
        scores[flags] = (flags_u8 & SAW_LOWER != 0) as u16
            + (flags_u8 & SAW_UPPER != 0) as u16
            + (flags_u8 & SAW_DIGIT != 0) as u16
            + 7 * ((flags_u8 & SAW_SEVEN != 0 && flags_u8 & SAW_NONSEVEN == 0) as u16);
        flags += 1;
    }

    scores
};

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

impl ColorState {
    fn push(self, c: u8) -> Self {
        match (self, c) {
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

            (ColorState::Green, _) | (ColorState::Red, _) | (ColorState::Blue, _) => self,

            _ => ColorState::Idle,
        }
    }

    fn multiplier(self) -> u16 {
        if matches!(self, ColorState::Green | ColorState::Red | ColorState::Blue) {
            3
        } else {
            1
        }
    }
}

impl State {
    fn from_line(line: &str) -> Self {
        let mut state = Self::default();

        for c in line.bytes() {
            state.flags |= BYTE_FLAGS[c as usize];

            if c == state.run_char {
                state.run_length += 1;
                if state.run_length >= 3 {
                    state.best_run_length = state.best_run_length.max(state.run_length);
                }
            } else {
                state.run_char = c;
                state.run_length = 1;
            }

            state.color_state = state.color_state.push(c);
            state.len += 1;
        }

        state
    }

    fn value(self) -> u16 {
        self.len as u16
            * (FLAG_SCORE[(self.flags & FLAG_MASK) as usize]
                + self.best_run_length as u16 * self.best_run_length as u16)
            * self.color_state.multiplier()
    }

    fn value_after_push(self, c: u8) -> u16 {
        let flags = self.flags | BYTE_FLAGS[c as usize];
        let best_run_length = if c == self.run_char && self.run_length >= 2 {
            self.best_run_length.max(self.run_length + 1)
        } else {
            self.best_run_length
        };

        (self.len as u16 + 1)
            * (FLAG_SCORE[(flags & FLAG_MASK) as usize] + best_run_length as u16 * best_run_length as u16)
            * self.color_state.push(c).multiplier()
    }
}
