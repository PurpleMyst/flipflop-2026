use atoi::FromRadix10;
use std::fmt::Display;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    include_str!("input.txt")
        .lines()
        .map(|n| {
            let (v, used) = u16::from_radix_10(n.as_bytes());
            debug_assert_eq!(used, n.len());
            v
        })
        .filter_map(|n| 60_u16.checked_sub(n))
        .sum::<u16>()
}

#[inline]
pub fn solve_part2() -> impl Display {
    include_str!("input.txt")
        .lines()
        .map(|n| {
            let (v, used) = u16::from_radix_10(n.as_bytes());
            debug_assert_eq!(used, n.len());
            v
        })
        .map(|n| if n > 60 { (n - 60) * 5 } else { 60 - n })
        .sum::<u16>()
}

#[inline]
pub fn solve_part3() -> impl Display {
    let lines = include_str!("input.txt")
        .lines()
        .map(|n| {
            let (v, used) = u16::from_radix_10(n.as_bytes());
            debug_assert_eq!(used, n.len());
            v
        })
        .collect::<Vec<_>>();
    let (temps, targets) = lines.split_at(lines.len() / 2);
    temps
        .into_iter()
        .copied()
        .zip(targets.into_iter().copied())
        .map(|(n, t)| if n > t { (n - t) * 5 } else { t - n })
        .sum::<u16>()
}
