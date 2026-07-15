const INPUT: &[u8] = include_bytes!("input.txt");
const WIDTH: usize = line_width(INPUT);
const HEIGHT: usize = line_count(INPUT);
const CELLS: usize = WIDTH * HEIGHT;

const fn line_width(input: &[u8]) -> usize {
    let mut width = 0;
    while width < input.len() && input[width] != b'\r' && input[width] != b'\n' {
        width += 1;
    }
    assert!(width != 0, "maze must not be empty");
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
                assert!(columns == width, "maze rows must have equal widths");
                rows += 1;
                columns = 0;
            }
            _ => columns += 1,
        }
        index += 1;
    }
    if columns != 0 {
        assert!(columns == width, "maze rows must have equal widths");
        rows += 1;
    }
    rows
}

const fn parse_maze(input: &[u8]) -> [u8; CELLS] {
    let mut maze = [0; CELLS];
    let mut src = 0;
    let mut dst = 0;
    while src < input.len() {
        let byte = input[src];
        src += 1;
        if byte != b'\r' && byte != b'\n' {
            let y = dst / WIDTH;
            let x = dst % WIDTH;
            assert!(
                (x != 0 && x != WIDTH - 1 && y != 0 && y != HEIGHT - 1) || byte == b'#',
                "maze must have a solid wall border",
            );
            maze[dst] = byte;
            dst += 1;
        }
    }
    assert!(dst == CELLS);
    maze
}

const fn find_marker(maze: &[u8; CELLS], marker: u8) -> usize {
    let mut found = usize::MAX;
    let mut index = 0;
    while index < CELLS {
        if maze[index] == marker {
            assert!(found == usize::MAX, "maze marker must be unique");
            found = index;
        }
        index += 1;
    }
    assert!(found != usize::MAX, "maze marker is missing");
    found
}

static MAZE: [u8; CELLS] = parse_maze(INPUT);
const START: usize = find_marker(&MAZE, b'S');
const END: usize = find_marker(&MAZE, b'E');

use std::{collections::VecDeque, fmt::Display};

#[derive(Clone, Copy)]
struct Seen {
    bits: [u64; CELLS.div_ceil(64)],
}

impl Default for Seen {
    fn default() -> Self {
        Self {
            bits: [0; CELLS.div_ceil(64)],
        }
    }
}

impl Seen {
    #[inline]
    fn insert(&mut self, index: usize) -> bool {
        let word = &mut self.bits[index / 64];
        let bit = 1u64 << (index % 64);
        if *word & bit != 0 {
            false
        } else {
            *word |= bit;
            true
        }
    }
}

const fn portal_endpoints() -> [[usize; 4]; CELLS] {
    let mut endpoints = [[0; 4]; CELLS];
    let mut y = 0;
    while y < HEIGHT {
        let mut left = 0;
        let mut x = 0;
        while x < WIDTH {
            let i = y * WIDTH + x;
            if MAZE[i] == b'#' {
                left = i + 1;
            } else {
                endpoints[i][1] = left;
            }
            x += 1;
        }

        let mut right = 0;
        x = WIDTH;
        while x > 0 {
            x -= 1;
            let i = y * WIDTH + x;
            if MAZE[i] == b'#' {
                right = i.saturating_sub(1);
            } else {
                endpoints[i][0] = right;
            }
        }
        y += 1;
    }

    let mut x = 0;
    while x < WIDTH {
        let mut up = 0;
        y = 0;
        while y < HEIGHT {
            let i = y * WIDTH + x;
            if MAZE[i] == b'#' {
                up = i + WIDTH;
            } else {
                endpoints[i][3] = up;
            }
            y += 1;
        }

        let mut down = 0;
        y = HEIGHT;
        while y > 0 {
            y -= 1;
            let i = y * WIDTH + x;
            if MAZE[i] == b'#' {
                down = i.saturating_sub(WIDTH);
            } else {
                endpoints[i][2] = down;
            }
        }
        x += 1;
    }

    endpoints
}

const PORTAL_ENDPOINTS: [[usize; 4]; CELLS] = portal_endpoints();

fn adjacent_to_wall() -> [bool; CELLS] {
    let mut adjacent = [false; CELLS];
    for i in WIDTH..CELLS - WIDTH {
        if MAZE[i] != b'#' {
            adjacent[i] = [i - WIDTH, i + WIDTH, i - 1, i + 1]
                .into_iter()
                .any(|next| MAZE[next] == b'#');
        }
    }
    adjacent
}

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let maze = &MAZE;

    let mut q = VecDeque::new();
    q.push_back(START);

    let mut visited = [false; CELLS];
    visited[START] = true;
    let mut d = 0;

    while !q.is_empty() {
        for _ in 0..q.len() {
            let i = q.pop_front().unwrap();

            if i == END {
                return d;
            }

            for next in [i - WIDTH, i + WIDTH, i - 1, i + 1] {
                if maze[next] != b'#' && !visited[next] {
                    visited[next] = true;
                    q.push_back(next);
                }
            }
        }
        d += 1;
    }

    0
}

#[inline]
pub fn solve_part2() -> impl Display {
    let maze = &MAZE;
    let endpoints = &PORTAL_ENDPOINTS;

    let mut current = vec![START];
    let mut next = Vec::new();

    let mut visited = Seen::default();
    visited.insert(START);
    let mut d = 0;

    while !current.is_empty() {
        for &i in &current {
            if i == END {
                return d;
            }

            for next_i in [
                i - WIDTH,
                i + WIDTH,
                i - 1,
                i + 1,
                endpoints[i][0],
                endpoints[i][1],
                endpoints[i][2],
                endpoints[i][3],
            ] {
                if maze[next_i] != b'#' && visited.insert(next_i) {
                    next.push(next_i);
                }
            }
        }
        current.clear();
        std::mem::swap(&mut current, &mut next);
        d += 1;
    }

    0
}

#[inline]
pub fn solve_part3() -> impl Display {
    let maze = &MAZE;
    let endpoints = &PORTAL_ENDPOINTS;
    let adjacent_to_wall = adjacent_to_wall();
    let start_state = START * 2;
    let mut distances = [usize::MAX; CELLS * 2];
    distances[start_state] = 0;

    let mut buckets = [[0; WIDTH]; 4];
    buckets[0][0] = start_state;
    let mut bucket_lengths = [1, 0, 0, 0];
    let mut queued = 1;
    let mut cost = 0;

    while queued != 0 {
        let bucket = cost % 4;
        while bucket_lengths[bucket] != 0 {
            bucket_lengths[bucket] -= 1;
            let state = buckets[bucket][bucket_lengths[bucket]];
            queued -= 1;
            if cost != distances[state] {
                continue;
            }

            let i = state / 2;
            let has_portal = state % 2 != 0;
            if i == END {
                return cost;
            }

            for next in [i - WIDTH, i + WIDTH, i - 1, i + 1] {
                if maze[next] == b'#' {
                    continue;
                }

                let next_state = next * 2;
                let next_cost = cost + 1;
                if next_cost < distances[next_state] {
                    distances[next_state] = next_cost;
                    let bucket = next_cost % 4;
                    buckets[bucket][bucket_lengths[bucket]] = next_state;
                    bucket_lengths[bucket] += 1;
                    queued += 1;
                }
            }

            if adjacent_to_wall[i] {
                let portal_cost = cost + if has_portal { 2 } else { 3 };
                for next in endpoints[i] {
                    let next_state = next * 2 + 1;
                    if portal_cost < distances[next_state] {
                        distances[next_state] = portal_cost;
                        let bucket = portal_cost % 4;
                        buckets[bucket][bucket_lengths[bucket]] = next_state;
                        bucket_lengths[bucket] += 1;
                        queued += 1;
                    }
                }
            }
        }
        cost += 1;
    }

    unreachable!("maze has no path to the end")
}
