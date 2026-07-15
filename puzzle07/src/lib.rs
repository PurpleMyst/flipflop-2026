use atoi::FromRadix10;
use std::{collections::VecDeque, fmt::Display};

const SIDE: usize = 30;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let (instrs, sushi) = include_str!("input.txt").split_once("\n\n").unwrap();

    let mut sushi = sushi
        .lines()
        .map(|l| {
            let (x_str, y_str) = l.split_once(',').unwrap();
            let (x, used_x) = usize::from_radix_10(x_str.as_bytes());
            let (y, used_y) = usize::from_radix_10(y_str.as_bytes());
            debug_assert_eq!(used_x, x_str.len());
            debug_assert_eq!(used_y, y_str.len());
            (x, y)
        })
        .peekable();

    let mut y = 0usize;
    let mut x = 0usize;

    let mut total = 0;

    for b in instrs.bytes().take(instrs.len() / 2) {
        match b {
            b'>' => x += 1,
            b'<' => x -= 1,
            b'^' => y += 1,
            b'v' => y -= 1,
            _ => unreachable!(),
        }

        if (x, y) == *sushi.peek().unwrap() {
            total += 1;
            let _ = sushi.next();
        }
    }

    total
}

#[inline]
pub fn solve_part2() -> impl Display {
    let (instrs, sushi) = include_str!("input.txt").split_once("\n\n").unwrap();

    let mut sushi = sushi
        .lines()
        .map(|l| {
            let (x_str, y_str) = l.split_once(',').unwrap();
            let (x, used_x) = usize::from_radix_10(x_str.as_bytes());
            let (y, used_y) = usize::from_radix_10(y_str.as_bytes());
            debug_assert_eq!(used_x, x_str.len());
            debug_assert_eq!(used_y, y_str.len());
            y * SIDE + x
        })
        .peekable();

    let mut segments = VecDeque::from([0]);
    let mut occupied = [false; SIDE * SIDE];
    occupied[0] = true;

    for b in instrs.bytes() {
        let mut position = *segments.back().unwrap();
        match b {
            b'>' => position += 1,
            b'<' => position -= 1,
            b'^' => position += SIDE,
            b'v' => position -= SIDE,
            _ => unreachable!(),
        }

        if position == *sushi.peek().unwrap() {
            sushi.next();
        } else {
            let tail = segments.pop_front().unwrap();
            occupied[tail] = false;
        }
        if occupied[position] {
            break;
        }
        segments.push_back(position);
        occupied[position] = true;
    }

    segments.len() + 1
}

#[inline]
pub fn solve_part3() -> impl Display {
    let (instrs, sushi) = include_str!("input.txt").split_once("\n\n").unwrap();

    let mut sushi = sushi
        .lines()
        .map(|l| {
            let (x_str, y_str) = l.split_once(',').unwrap();
            let (x, used_x) = usize::from_radix_10(x_str.as_bytes());
            let (y, used_y) = usize::from_radix_10(y_str.as_bytes());
            debug_assert_eq!(used_x, x_str.len());
            debug_assert_eq!(used_y, y_str.len());
            y * SIDE + x
        })
        .peekable();

    let mut positions = Vec::with_capacity(instrs.len() + 1);
    positions.push(0);
    let mut tail = 0;
    let mut occupied = [false; SIDE * SIDE];
    occupied[0] = true;

    let mut noms = 0;
    for b in instrs.bytes() {
        let mut position = *positions.last().unwrap();
        match b {
            b'>' => position += 1,
            b'<' => position -= 1,
            b'^' => position += SIDE,
            b'v' => position -= SIDE,
            _ => unreachable!(),
        }

        if position == *sushi.peek().unwrap() {
            sushi.next();
            positions.push(position);
            occupied[position] = true;
            continue;
        }

        if occupied[position] && positions[tail] != position {
            loop {
                let old_tail = positions[tail];
                tail += 1;
                occupied[old_tail] = false;
                if old_tail == position {
                    break;
                }
            }
            noms += 1;
        }

        occupied[positions[tail]] = false;
        tail += 1;
        positions.push(position);
        occupied[position] = true;
    }

    (positions.len() - tail) * noms
}
