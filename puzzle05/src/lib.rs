use std::fmt::Display;

const INPUT: &[u8] = include_bytes!("input.txt");
const WIDTH: usize = line_width(INPUT);
const HEIGHT: usize = line_count(INPUT);
const CELLS: usize = WIDTH * HEIGHT;
const MAP: [u8; CELLS] = load_map(INPUT);

#[derive(Clone, Copy)]
struct Seen {
    cells: [u64; CELLS.div_ceil(64)],
    len: u16,
}

impl Default for Seen {
    fn default() -> Self {
        Self {
            cells: [0; CELLS.div_ceil(64)],
            len: 0,
        }
    }
}

impl Seen {
    #[inline]
    fn insert(&mut self, index: usize) -> bool {
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

const fn line_width(input: &[u8]) -> usize {
    let mut width = 0;
    while width < input.len() && input[width] != b'\r' && input[width] != b'\n' {
        width += 1;
    }
    assert!(width != 0, "map must not be empty");
    width
}

const fn line_count(input: &[u8]) -> usize {
    let width = line_width(input);
    let mut rows = 0;
    let mut columns = 0;
    let mut index = 0;
    while index < input.len() {
        match input[index] {
            b'\r' => {}
            b'\n' => {
                assert!(columns == width, "map rows must have equal widths");
                rows += 1;
                columns = 0;
            }
            _ => columns += 1,
        }
        index += 1;
    }
    if columns != 0 {
        assert!(columns == width, "map rows must have equal widths");
        rows += 1;
    }
    rows
}

const fn load_map(input: &[u8]) -> [u8; CELLS] {
    let mut map = [0; CELLS];
    let mut src = 0;
    let mut dst = 0;
    while src < input.len() {
        if input[src] != b'\r' && input[src] != b'\n' {
            map[dst] = input[src];
            dst += 1;
        }
        src += 1;
    }
    assert!(dst == CELLS);
    map
}

#[inline]
fn next(index: usize, direction: u8) -> usize {
    match direction {
        b'>' => index + 1,
        b'<' => index - 1,
        b'^' => index - WIDTH,
        b'v' => index + WIDTH,
        _ => unreachable!(),
    }
}

#[inline]
fn is_edge(index: usize) -> bool {
    index < WIDTH || index >= CELLS - WIDTH || index.is_multiple_of(WIDTH) || (index + 1).is_multiple_of(WIDTH)
}

fn walk_p1(map: &[u8; CELLS], seen: &mut Seen) -> u16 {
    let mut index = 0;
    while seen.insert(index) {
        index = next(index, map[index]);
    }
    seen.len
}

fn walk_p2(map: &[u8; CELLS], index: usize, changed: bool, seen: &mut Seen) -> u16 {
    if !seen.insert(index) {
        seen.len
    } else if changed || is_edge(index) {
        walk_p2(map, next(index, map[index]), changed, seen)
    } else {
        let seen_copy = *seen;
        let direction = map[index];
        let right = if direction == b'>' {
            0
        } else {
            let result = walk_p2(map, index + 1, true, seen);
            seen.clone_from(&seen_copy);
            result
        };
        let left = if direction == b'<' {
            0
        } else {
            let result = walk_p2(map, index - 1, true, seen);
            seen.clone_from(&seen_copy);
            result
        };
        let up = if direction == b'^' {
            0
        } else {
            let result = walk_p2(map, index - WIDTH, true, seen);
            seen.clone_from(&seen_copy);
            result
        };
        let down = if direction == b'v' {
            0
        } else {
            let result = walk_p2(map, index + WIDTH, true, seen);
            seen.clone_from(&seen_copy);
            result
        };

        left.max(right)
            .max(up)
            .max(down)
            .max(walk_p2(map, next(index, direction), changed, seen))
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

fn walk_p3<const CHANGED: bool>(
    map: &[u8; CELLS],
    index: usize,
    changed: Option<(usize, u8)>,
    turns: usize,
    seen: &mut Seen,
) -> u16 {
    if !seen.insert(index) {
        if turns == 3 || is_edge(index) {
            seen.len
        } else {
            let b = if CHANGED {
                let (changed_index, changed_direction) = changed.unwrap();
                if changed_index == index {
                    changed_direction
                } else {
                    map[index]
                }
            } else {
                map[index]
            };
            walk_p3::<CHANGED>(map, next(index, wrong(b)), changed, turns + 1, seen)
        }
    } else if CHANGED || is_edge(index) {
        walk_p3::<CHANGED>(map, next(index, map[index]), changed, turns, seen)
    } else {
        let direction = map[index];
        let right = if direction == b'>' {
            0
        } else {
            let mut branch_seen = *seen;
            walk_p3::<true>(map, index + 1, Some((index, b'>')), turns, &mut branch_seen)
        };
        let left = if direction == b'<' {
            0
        } else {
            let mut branch_seen = *seen;
            walk_p3::<true>(map, index - 1, Some((index, b'<')), turns, &mut branch_seen)
        };
        let up = if direction == b'^' {
            0
        } else {
            let mut branch_seen = *seen;
            walk_p3::<true>(map, index - WIDTH, Some((index, b'^')), turns, &mut branch_seen)
        };
        let down = if direction == b'v' {
            0
        } else {
            let mut branch_seen = *seen;
            walk_p3::<true>(map, index + WIDTH, Some((index, b'v')), turns, &mut branch_seen)
        };

        left.max(right)
            .max(up)
            .max(down)
            .max(walk_p3::<false>(map, next(index, direction), None, turns, seen))
    }
}

#[inline]
pub fn solve_part1() -> impl Display {
    walk_p1(&MAP, &mut Seen::default())
}

#[inline]
pub fn solve_part2() -> impl Display {
    walk_p2(&MAP, 0, false, &mut Seen::default())
}

#[inline]
pub fn solve_part3() -> impl Display {
    walk_p3::<false>(&MAP, 0, None, 0, &mut Seen::default())
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
