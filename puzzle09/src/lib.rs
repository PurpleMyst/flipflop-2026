use std::{collections::VecDeque, fmt::Display};

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let maze = include_str!("input.txt")
        .bytes()
        .filter(|b| !b.is_ascii_whitespace())
        .collect::<Vec<_>>();
    let h = include_str!("input.txt").lines().count();
    let w = maze.len() / h;

    let mut start = (0, 0);
    for (i, b) in maze.iter().enumerate() {
        if b == &b'S' {
            start = (i / w, i % w)
        }
    }

    let mut q = VecDeque::new();
    q.push_back((start, 0));

    let mut visited = vec![false; maze.len()];

    while let Some(((y, x), d)) = q.pop_front() {
        let i = y * w + x;
        if visited[i] {
            continue;
        }
        visited[i] = true;

        if maze[i] == b'E' {
            return d;
        }

        q.extend(
            [
                (y.wrapping_sub(1), x),
                (y.wrapping_add(1), x),
                (y, x.wrapping_sub(1)),
                (y, x.wrapping_add(1)),
            ]
            .into_iter()
            .filter(|&(y, x)| y < h && x < w)
            .filter(|&(y, x)| maze[y * w + x] != b'#')
            .map(|(y, x)| ((y, x), d + 1)),
        );
    }

    0
}

#[inline]
pub fn solve_part2() -> impl Display {
    let maze = include_str!("input.txt")
        .bytes()
        .filter(|b| !b.is_ascii_whitespace())
        .collect::<Vec<_>>();
    let h = include_str!("input.txt").lines().count();
    let w = maze.len() / h;

    let mut start = (0, 0);
    for (i, b) in maze.iter().enumerate() {
        if b == &b'S' {
            start = (i / w, i % w)
        }
    }

    let mut q = VecDeque::new();
    q.push_back((start, 0));

    let mut visited = vec![false; maze.len()];

    while let Some(((y, x), d)) = q.pop_front() {
        let i = y * w + x;
        if visited[i] {
            continue;
        }
        visited[i] = true;

        if maze[i] == b'E' {
            return d;
        }

        q.extend(
            [
                (y.wrapping_sub(1), x),
                (y.wrapping_add(1), x),
                (y, x.wrapping_sub(1)),
                (y, x.wrapping_add(1)),
                (y, (x..w).take_while(|x| maze[y * w + x] != b'#').last().unwrap()),
                (y, (0..=x).rev().take_while(|x| maze[y * w + x] != b'#').last().unwrap()),
                ((y..h).take_while(|y| maze[y * w + x] != b'#').last().unwrap(), x),
                ((0..=y).rev().take_while(|y| maze[y * w + x] != b'#').last().unwrap(), x),
            ]
            .into_iter()
            .filter(|&(y, x)| y < h && x < w)
            .filter(|&(y, x)| maze[y * w + x] != b'#')
            .map(|(y, x)| ((y, x), d + 1)),
        );
    }

    0
}

#[inline]
pub fn solve_part3() -> impl Display {
    let maze = include_str!("input.txt")
        .bytes()
        .filter(|b| !b.is_ascii_whitespace())
        .collect::<Vec<_>>();
    let maze = &maze;
    let h = include_str!("input.txt").lines().count();
    let w = maze.len() / h;

    let mut start = (0, 0);
    for (i, b) in maze.iter().enumerate() {
        if b == &b'S' {
            start = (i / w, i % w)
        }
    }

    let (_steps, cost) = pathfinding::prelude::dijkstra(
        &(start, false),
        |&((y, x), has_portal)| {
            [
                (y.wrapping_sub(1), x),
                (y.wrapping_add(1), x),
                (y, x.wrapping_sub(1)),
                (y, x.wrapping_add(1)),
            ]
            .into_iter()
            .filter(|&(y, x)| y < h && x < w)
            .filter(move |(y, x)| maze[y * w + x] != b'#')
            .map(move |(y, x)| (((y, x), false), 1))
            .chain(
                [
                    (y, (x..w).take_while(|x| maze[y * w + x] != b'#').last().unwrap()),
                    (y, (0..=x).rev().take_while(|x| maze[y * w + x] != b'#').last().unwrap()),
                    ((y..h).take_while(|y| maze[y * w + x] != b'#').last().unwrap(), x),
                    ((0..=y).rev().take_while(|y| maze[y * w + x] != b'#').last().unwrap(), x),
                ]
                .into_iter()
                .filter(move |_| {
                    [
                        (y.wrapping_sub(1), x),
                        (y.wrapping_add(1), x),
                        (y, x.wrapping_sub(1)),
                        (y, x.wrapping_add(1)),
                    ]
                    .into_iter()
                    .any(|(y, x)| maze[y * w + x] == b'#')
                })
                .map(move |(y, x)| (((y, x), true), if has_portal { 2 } else { 3 })),
            )
        },
        |&((y, x), ..)| maze[y * w + x] == b'E',
    )
    .unwrap();

    cost
}
