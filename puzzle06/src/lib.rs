use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;

const WIDTH: usize = 200;
const HEIGHT: usize = WIDTH / 2;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Rotation {
    CW,
    CCW,
}
use Rotation::*;

impl std::ops::Not for Rotation {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            CW => CCW,
            CCW => CW,
        }
    }
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

    let start_idx = map.iter().position(|&b| b == b'S').unwrap();
    let mut q = VecDeque::new();
    q.push_back((start_idx / WIDTH, start_idx % WIDTH, CCW));
    let mut state = HashMap::new();

    let mut lights = Vec::new();

    while let Some((y, x, r)) = q.pop_front() {
        if state.insert((y, x), r).is_some() {
            continue;
        }

        if map[y * WIDTH + x] == b'*' {
            lights.push((y, x, r == CCW));
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

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!(
                "{}",
                match state.get(&(y, x)) {
                    Some(CW) => "R",
                    Some(CCW) => "L",
                    None => ".",
                }
            );
        }
        println!();
    }

    lights
        .into_iter()
        .fold(0, |acc, (_, _, b)| (acc << 1) | (if b { 1 } else { 0 }))
}

#[inline]
pub fn solve_part2() -> impl Display {
    let map = include_str!("input.txt")
        .lines()
        .flat_map(|s| s.trim().bytes())
        .collect::<Vec<u8>>();
    let mut lights = Vec::new();

    let start_idx = map.iter().position(|&b| b == b'S').unwrap();
    let mut q = VecDeque::new();
    q.push_back((start_idx / WIDTH, start_idx % WIDTH, CCW));
    let mut state = HashMap::new();

    while !q.is_empty() {
        eprintln!("{q:?}");
        while let Some((y, x, r)) = q.pop_front() {
            if let Some(old_r) = state.insert((y, x), r) {
                assert_eq!(r, old_r);
                continue;
            }

            if map[y * WIDTH + x] == b'*' {
                lights.push((y, x, r == CCW));
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
                .filter(|&(ny, nx)| matches!(map[ny * WIDTH + nx], b'#' | b'3' | b'*' | b'a'..=b'z'))
                .map(|(ny, nx)| {
                    let n = map[ny * WIDTH + nx];
                    let nr = if n == b'S' {
                        CCW
                    } else if n.is_ascii_uppercase() {
                        let Some(i_idx) = map.iter().position(|&b| b == n.to_ascii_lowercase()) else {
                            panic!("can't find {:?}", n as char)
                        };
                        let iy = i_idx / WIDTH;
                        let ix = i_idx % WIDTH;
                        let ir = [
                            (iy.wrapping_sub(1), ix),
                            (iy.wrapping_add(1), ix),
                            (iy, ix.wrapping_sub(1)),
                            (iy, ix.wrapping_add(1)),
                        ]
                        .into_iter()
                        .find_map(|jp| state.get(&jp).copied())
                        .unwrap();
                        ir
                    } else {
                        !r
                    };

                    (ny, nx, nr)
                }),
            )
        }

        q.extend(map.iter().enumerate().filter_map(|(idx, &cell)| {
            if !cell.is_ascii_uppercase() || cell == b'S' {
                return None;
            }

            let y = idx / WIDTH;
            let x = idx % WIDTH;
            if state.contains_key(&(y, x)) {
                return None;
            }

            let i_idx = map.iter().position(|&b| b == cell.to_ascii_lowercase()).unwrap();
            let iy = i_idx / WIDTH;
            let ix = i_idx % WIDTH;
            let ir = [
                (iy.wrapping_sub(1), ix),
                (iy.wrapping_add(1), ix),
                (iy, ix.wrapping_sub(1)),
                (iy, ix.wrapping_add(1)),
            ]
            .into_iter()
            .find_map(|jp| state.get(&jp).copied())?;

            eprintln!("propagating {y} {x} {} {ir:?}", cell as char);

            Some((y, x, ir))
        }));
    }

    assert!(
        map.iter()
            .enumerate()
            .all(|(idx, &cell)| { !cell.is_ascii_uppercase() || state.contains_key(&(idx / WIDTH, idx % WIDTH)) })
    );

    lights.sort_unstable_by_key(|&(y, x, _)| (y, x));

    println!();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!(
                "{}",
                match state.get(&(y, x)) {
                    Some(CW) => "R",
                    Some(CCW) => "L",
                    None => ".",
                }
            );
        }
        println!();
    }

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
        let mut v = HashSet::new();
        q.push_back((y, x));
        while let Some((y, x)) = q.pop_front() {
            if !v.insert((y, x)) {
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

        if prime(v.len() - 1) {
            boot.push(cell);
        }
    });
    map.iter_mut().for_each(|c| {
        if c.is_ascii_alphabetic() && boot.contains(&c.to_ascii_uppercase()) {
            *c = b'.';
        }
    });

    let mut lights = Vec::new();

    let start_idx = map.iter().position(|&b| b == b'S').unwrap();
    let mut q = VecDeque::new();
    q.push_back((start_idx / WIDTH, start_idx % WIDTH, CCW));
    let mut state = HashMap::new();

    while !q.is_empty() {
        eprintln!("{q:?}");
        while let Some((y, x, r)) = q.pop_front() {
            if let Some(old_r) = state.insert((y, x), r) {
                assert_eq!(r, old_r);
                continue;
            }

            if map[y * WIDTH + x] == b'*' {
                lights.push((y, x, r == CCW));
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
                .filter(|&(ny, nx)| matches!(map[ny * WIDTH + nx], b'#' | b'3' | b'*' | b'a'..=b'z'))
                .map(|(ny, nx)| {
                    let n = map[ny * WIDTH + nx];
                    let nr = if n == b'S' {
                        CCW
                    } else if n.is_ascii_uppercase() {
                        let Some(i_idx) = map.iter().position(|&b| b == n.to_ascii_lowercase()) else {
                            panic!("can't find {:?}", n as char)
                        };
                        let iy = i_idx / WIDTH;
                        let ix = i_idx % WIDTH;
                        let ir = [
                            (iy.wrapping_sub(1), ix),
                            (iy.wrapping_add(1), ix),
                            (iy, ix.wrapping_sub(1)),
                            (iy, ix.wrapping_add(1)),
                        ]
                        .into_iter()
                        .find_map(|jp| state.get(&jp).copied())
                        .unwrap();
                        ir
                    } else {
                        !r
                    };

                    (ny, nx, nr)
                }),
            )
        }

        q.extend(map.iter().enumerate().filter_map(|(idx, &cell)| {
            if !cell.is_ascii_uppercase() || cell == b'S' {
                return None;
            }

            let y = idx / WIDTH;
            let x = idx % WIDTH;
            if state.contains_key(&(y, x)) {
                return None;
            }

            let i_idx = map.iter().position(|&b| b == cell.to_ascii_lowercase()).unwrap();
            let iy = i_idx / WIDTH;
            let ix = i_idx % WIDTH;
            let ir = [
                (iy.wrapping_sub(1), ix),
                (iy.wrapping_add(1), ix),
                (iy, ix.wrapping_sub(1)),
                (iy, ix.wrapping_add(1)),
            ]
            .into_iter()
            .find_map(|jp| state.get(&jp).copied())?;

            eprintln!("propagating {y} {x} {} {ir:?}", cell as char);

            Some((y, x, ir))
        }));
    }

    // assert!(
    //     map.iter()
    //         .enumerate()
    //         .all(|(idx, &cell)| { !cell.is_ascii_uppercase() || state.contains_key(&(idx / WIDTH, idx % WIDTH)) })
    // );

    lights.sort_unstable_by_key(|&(y, x, _)| (y, x));

    println!();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!(
                "{}",
                match state.get(&(y, x)) {
                    Some(CW) => "R",
                    Some(CCW) => "L",
                    None => ".",
                }
            );
        }
        println!();
    }

    lights
        .into_iter()
        .fold(0u64, |acc, (_, _, b)| (acc << 1) | (if b { 1 } else { 0 }))
}

fn prime(len: usize) -> bool {
    !(2..len).any(|n| len.is_multiple_of(n))
}

