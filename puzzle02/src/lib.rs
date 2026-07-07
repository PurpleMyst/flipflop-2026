use std::{cmp::Reverse, fmt::Display};

const N: usize = 100;

fn exec(p: &mut usize, b: u8) {
    match b {
        b'>' => *p = (*p + 1) % N,
        b'<' => *p = p.checked_sub(1).unwrap_or(N - 1),
        _ => unreachable!(),
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let mut walls = [0; N];
    let mut pos = 0usize;
    for b in include_str!("input.txt").trim().bytes() {
        exec(&mut pos, b);
        walls[pos] += 1;
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
    let mut pos = 0usize;
    let mut pos2 = 0usize;
    let mut temp2 = 0;
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
    let mut pos = 0usize;
    let instrs = include_str!("input.txt").trim();
    let mut walls = [0; N];
    let mut pointers = (0..N).collect::<Vec<_>>();
    for (b, b2) in instrs.bytes().zip(instrs.bytes().rev()) {
        exec(&mut pos, b);
        match b2 {
            b'>' => pointers.rotate_right(1),
            b'<' => pointers.rotate_left(1),
            _ => unreachable!(),
        }
        walls[pointers[pos]] += 1;
    }
    let (p, t) = walls
        .into_iter()
        .enumerate()
        .max_by_key(|&(p, t)| (t, Reverse(p)))
        .unwrap();
    t * (p + 1)
}
