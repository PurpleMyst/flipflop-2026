use std::{cmp::Reverse, fmt::Display};

const N: u8 = 100;

fn exec(p: &mut u8, b: u8) {
    if b == b'>' {
        *p = (*p + 1) % N
    } else {
        *p = p.checked_sub(1).unwrap_or(N - 1)
    }
}

fn exec_rev(p: &mut u8, b: u8) {
    if b == b'<' {
        *p = (*p + 1) % N
    } else {
        *p = p.checked_sub(1).unwrap_or(N - 1)
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let mut walls = [0; N as usize];
    let mut pos = 0u8;
    for b in include_str!("input.txt").trim().bytes() {
        exec(&mut pos, b);
        walls[pos as usize] += 1;
    }
    let (p, t) = walls
        .into_iter()
        .enumerate()
        .max_by_key(|&(p, t)| (t, Reverse(p)))
        .unwrap();
    t * (p + 1)
}

#[inline]
pub fn solve_part2() -> impl Display {
    let mut pos = 0u8;
    let mut pos2 = 0u8;
    let mut temp2 = 0u16;
    let instrs = include_str!("input.txt").trim();
    for (b, b2) in instrs.bytes().zip(instrs.bytes().rev()) {
        exec(&mut pos, b);
        exec(&mut pos2, b2);
        if pos == pos2 {
            temp2 += 1;
        }
    }
    temp2
}

#[inline]
pub fn solve_part3() -> impl Display {
    let mut pos = 0u8;
    let instrs = include_str!("input.txt").trim();
    let mut walls = [0u16; N as usize];
    for (b, b2) in instrs.bytes().zip(instrs.bytes().rev()) {
        exec(&mut pos, b);
        exec_rev(&mut pos, b2);
        walls[pos as usize] += 1;
    }
    let (p, t) = walls
        .into_iter()
        .enumerate()
        .max_by_key(|&(p, t)| (t, Reverse(p)))
        .unwrap();
    t * (p as u16 + 1)
}
