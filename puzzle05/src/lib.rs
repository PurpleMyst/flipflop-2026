use std::fmt::Display;

const SIDE: usize = 50;

#[derive(Clone, Copy)]
struct Seen {
    cells: [u64; 40],
    len: u16,
}

impl Default for Seen {
    fn default() -> Self {
        Self { cells: [0; 40], len: 0 }
    }
}

impl Seen {
    #[inline]
    fn insert(&mut self, y: usize, x: usize) -> bool {
        let index = y * SIDE + x;
        let word = &mut self.cells[index / 64];
        let bit = 1u64 << (index % 64);
        if *word & bit != 0 {
            false
        } else {
            *word |= bit;
            self.len += 1;
            true
        }
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

fn load_map() -> Vec<u8> {
    include_str!("input.txt")
        .lines()
        .flat_map(|s| s.trim().bytes())
        .collect()
}

fn walk_p1(map: &[u8], seen: &mut Seen) -> u16 {
    let mut y = 0;
    let mut x = 0;
    while seen.insert(y, x) {
        match map[y * SIDE + x] {
            b'>' => x += 1,
            b'<' => x -= 1,
            b'^' => y -= 1,
            b'v' => y += 1,
            _ => unreachable!(),
        }
    }
    seen.len
}

fn walk_p2(map: &[u8], y: usize, x: usize, changed: bool, seen: &mut Seen) -> u16 {
    if !seen.insert(y, x) {
        seen.len
    } else if changed || x == 0 || y == 0 || x == SIDE - 1 || y == SIDE - 1 {
        match map[y * SIDE + x] {
            b'>' => walk_p2(map, y, x + 1, changed, seen),
            b'<' => walk_p2(map, y, x - 1, changed, seen),
            b'^' => walk_p2(map, y - 1, x, changed, seen),
            b'v' => walk_p2(map, y + 1, x, changed, seen),
            _ => unreachable!(),
        }
    } else {
        let seen_copy = *seen;
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

fn walk_p3(map: &[u8], y: usize, x: usize, changed: Option<(usize, usize, u8)>, turns: usize, seen: &mut Seen) -> u16 {
    if !seen.insert(y, x) {
        if turns == 3 || x == 0 || y == 0 || x == SIDE - 1 || y == SIDE - 1 {
            seen.len
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
        let seen_copy = *seen;
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
pub fn solve_part1() -> impl Display {
    walk_p1(&load_map(), &mut Seen::default())
}

#[inline]
pub fn solve_part2() -> impl Display {
    let map = load_map();
    walk_p2(&map, 0, 0, false, &mut Seen::default())
}

#[inline]
pub fn solve_part3() -> impl Display {
    let map = load_map();
    walk_p3(&map, 0, 0, None, 0, &mut Seen::default())
}

#[cfg(test)]
mod tests {
    #[test]
    fn solutions_match_expected_answers() {
        assert_eq!(format!("{}", super::solve_part1()), "210");
        assert_eq!(format!("{}", super::solve_part2()), "261");
        assert_eq!(format!("{}", super::solve_part3()), "301");
    }
}
