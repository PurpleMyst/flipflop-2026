use std::{collections::HashSet, fmt::Display};

const SIDE: usize = 50;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let map = include_str!("input.txt")
        .lines()
        .flat_map(|s| s.trim().bytes())
        .collect::<Vec<_>>();

    let mut y = 0;
    let mut x = 0;
    let mut seen = HashSet::new();
    while seen.insert((y, x)) {
        match map[y * SIDE + x] {
            b'>' => x += 1,
            b'<' => x -= 1,
            b'^' => y -= 1,
            b'v' => y += 1,
            _ => unreachable!(),
        }
    }

    seen.len()
}

fn walk_p2(map: &[u8], y: usize, x: usize, changed: bool, seen: &mut HashSet<(usize, usize)>) -> usize {
    if !seen.insert((y, x)) {
        seen.len()
    } else if changed || x == 0 || y == 0 || x == SIDE - 1 || y == SIDE - 1 {
        match map[y * SIDE + x] {
            b'>' => walk_p2(map, y, x + 1, changed, seen),
            b'<' => walk_p2(map, y, x - 1, changed, seen),
            b'^' => walk_p2(map, y - 1, x, changed, seen),
            b'v' => walk_p2(map, y + 1, x, changed, seen),
            _ => unreachable!(),
        }
    } else {
        let seen_copy = seen.clone();

        let right = walk_p2(map, y, x + 1, true, seen);
        seen.clone_from(&seen_copy);

        let left = walk_p2(map, y, x - 1, true, seen);
        seen.clone_from(&seen_copy);

        let up = walk_p2(map, y - 1, x, true, seen);
        seen.clone_from(&seen_copy);

        let down = walk_p2(map, y + 1, x, true, seen);
        seen.clone_from(&seen_copy);

        left.max(right).max(up).max(down).max(match map[y * SIDE + x] {
            b'>' => walk_p2(map, y, x + 1, changed, seen),
            b'<' => walk_p2(map, y, x - 1, changed, seen),
            b'^' => walk_p2(map, y - 1, x, changed, seen),
            b'v' => walk_p2(map, y + 1, x, changed, seen),
            _ => unreachable!(),
        })
    }
}

fn wrong(b: u8) -> u8 {
    match b {
        b'^' => b'>',
        b'>' => b'v',
        b'v' => b'<',
        b'<' => b'^',
        _ => unreachable!(),
    }
}

fn walk_p3(
    map: &[u8],
    y: usize,
    x: usize,
    changed: Option<(usize, usize, u8)>,
    turns: usize,
    seen: &mut HashSet<(usize, usize)>,
) -> usize {
    if !seen.insert((y, x)) {
        if turns == 3 || x == 0 || y == 0 || x == SIDE - 1 || y == SIDE - 1 {
            seen.len()
        } else {
            let b = if let Some((cy, cx, cb)) = changed
                && (cy, cx) == (y, x)
            {
                cb
            } else {
                map[y * SIDE + x]
            };

            match wrong(b) {
                b'>' => walk_p3(map, y, x + 1, changed, turns + 1, seen),
                b'<' => walk_p3(map, y, x - 1, changed, turns + 1, seen),
                b'^' => walk_p3(map, y - 1, x, changed, turns + 1, seen),
                b'v' => walk_p3(map, y + 1, x, changed, turns + 1, seen),
                _ => unreachable!(),
            }
        }
    } else if changed.is_some() || x == 0 || y == 0 || x == SIDE - 1 || y == SIDE - 1 {
        match map[y * SIDE + x] {
            b'>' => walk_p3(map, y, x + 1, changed, turns, seen),
            b'<' => walk_p3(map, y, x - 1, changed, turns, seen),
            b'^' => walk_p3(map, y - 1, x, changed, turns, seen),
            b'v' => walk_p3(map, y + 1, x, changed, turns, seen),
            _ => unreachable!(),
        }
    } else {
        let seen_copy = seen.clone();

        let right = walk_p3(map, y, x + 1, Some((y, x, b'>')), turns, seen);
        seen.clone_from(&seen_copy);

        let left = walk_p3(map, y, x - 1, Some((y, x, b'<')), turns, seen);
        seen.clone_from(&seen_copy);

        let up = walk_p3(map, y - 1, x, Some((y, x, b'^')), turns, seen);
        seen.clone_from(&seen_copy);

        let down = walk_p3(map, y + 1, x, Some((y, x, b'v')), turns, seen);
        seen.clone_from(&seen_copy);

        left.max(right).max(up).max(down).max(match map[y * SIDE + x] {
            b'>' => walk_p3(map, y, x + 1, None, turns, seen),
            b'<' => walk_p3(map, y, x - 1, None, turns, seen),
            b'^' => walk_p3(map, y - 1, x, None, turns, seen),
            b'v' => walk_p3(map, y + 1, x, None, turns, seen),
            _ => unreachable!(),
        })
    }
}

#[inline]
pub fn solve_part2() -> impl Display {
    let map = include_str!("input.txt")
        .lines()
        .flat_map(|s| s.trim().bytes())
        .collect::<Vec<_>>();
    walk_p2(&map, 0, 0, false, &mut Default::default())
}

#[inline]
pub fn solve_part3() -> impl Display {
    let map = include_str!("input.txt")
        .lines()
        .flat_map(|s| s.trim().bytes())
        .collect::<Vec<_>>();
    walk_p3(&map, 0, 0, None, 0, &mut Default::default())
}
