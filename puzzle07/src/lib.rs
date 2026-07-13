use atoi::FromRadix10;
use std::fmt::Display;

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
            (x, y)
        })
        .peekable();

    let mut segments = vec![(0, 0)];

    for b in instrs.bytes() {
        let &(mut x, mut y) = segments.last().unwrap();
        match b {
            b'>' => x += 1,
            b'<' => x -= 1,
            b'^' => y += 1,
            b'v' => y -= 1,
            _ => unreachable!(),
        }

        if (x, y) == *sushi.peek().unwrap() {
            sushi.next();
        } else {
            segments.remove(0);
        }
        if segments.contains(&(x, y)) {
            break;
        }
        segments.push((x, y));
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
            (x, y)
        })
        .peekable();

    let mut segments = vec![(0, 0)];

    let mut noms = 0;
    for b in instrs.bytes() {
        let &(mut x, mut y) = segments.last().unwrap();
        match b {
            b'>' => x += 1,
            b'<' => x -= 1,
            b'^' => y += 1,
            b'v' => y -= 1,
            _ => unreachable!(),
        }

        segments.push((x, y));
        if (x, y) == *sushi.peek().unwrap() {
            sushi.next();
            continue;
        } else if let Some(idx) = segments.iter().position(|p| p == &(x, y))
            && idx != 0
            && idx != segments.len() - 1
        {
            segments.drain(..=idx);
            noms += 1;
        }
        segments.remove(0);
    }

    segments.len() * noms
}
