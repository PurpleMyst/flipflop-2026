use std::fmt::Display;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    load_input().skip(400).filter(|&n| n != 0).count()
}

#[inline]
pub fn solve_part2() -> impl Display {
    load_input()
        .filter(|&n| n != 0)
        .collect::<Vec<_>>()
        .array_windows()
        .filter(|[prev, next]| prev != next)
        .count()
}

#[inline]
pub fn solve_part3() -> impl Display {
    let mut flower = load_input().filter(|&n| n != 0).collect::<Vec<_>>();
    let mut answer = 0;
    while !flower.is_empty() {
        for i in 0..flower.len() - 1 {
            if flower[i] == flower[i + 1] {
                continue;
            }
            flower[i] = 0;
        }
        flower.pop();
        flower.retain(|&n| n != 0);
        answer += 1;
    }
    answer
}

fn load_input() -> impl Iterator<Item = i8> {
    include_str!("input.txt").lines().rev().filter_map(|line| {
        Some(match line.trim() {
            "o-|" => -1,
            "|-o" => 1,
            "|" => 0,
            _ => return None,
        })
    })
}
