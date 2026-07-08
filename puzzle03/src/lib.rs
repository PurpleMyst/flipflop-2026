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
        .max_by_key(|line| strength(line))
        .unwrap()
}

#[inline]
pub fn solve_part3() -> impl Display {
    b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .into_par_iter()
        .map(|&big_c| {
            include_str!("input.txt")
                .lines()
                .map(|line| {
                    let mut line = String::from(line);
                    line.push(big_c as char);
                    strength(&line)
                })
                .sum::<usize>()
        })
        .max()
        .unwrap()
}

fn strength(line: &str) -> usize {
    let mut cs = line.bytes();

    let mut k = cs.next().unwrap();
    let mut n = 1;
    let mut m = 0;
    for c in cs {
        if c != k {
            k = c;
            n = 1;
            continue;
        }
        n += 1;
        if n >= 3 {
            m = m.max(n);
        }
    }

    line.len()
        * ((line.bytes().any(|c| c.is_ascii_lowercase()) as usize
            + line.bytes().any(|c| c.is_ascii_uppercase()) as usize
            + line.bytes().any(|c| c.is_ascii_digit()) as usize)
            + 7 * ((line.bytes().any(|c| c == b'7') && line.bytes().filter(|c| c.is_ascii_digit()).all(|c| c == b'7'))
                as usize)
            + m * m)
        * (if line.contains("red") || line.contains("green") || line.contains("blue") {
            3
        } else {
            1
        })
}
