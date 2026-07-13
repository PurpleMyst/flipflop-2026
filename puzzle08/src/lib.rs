use std::collections::HashMap;
use std::fmt::Display;
use std::mem::swap;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let mut kv = HashMap::new();

    include_str!("input.txt").lines().for_each(|line| {
        let (from, tos) = line.split_once(' ').unwrap();
        let tos = tos.split(' ').collect::<Vec<_>>();
        kv.entry(from).or_insert(tos);
    });

    let mut stoats = HashMap::<&str, usize>::new();
    stoats.insert("A", 1);
    stoats.insert("B", 1);

    let mut new_stoats = HashMap::<&str, usize>::new();

    for _ in 0..7 {
        for (k, v) in stoats.drain() {
            for s in &kv[k] {
                *new_stoats.entry(s).or_default() += v;
            }
        }
        swap(&mut stoats, &mut new_stoats)
    }

    stoats.values().sum::<usize>()
}

#[inline]
pub fn solve_part2() -> impl Display {
    do_solve23(7)
}

#[inline]
pub fn solve_part3() -> impl Display {
    do_solve23(21)
}

#[inline]
fn do_solve23(gens: usize) -> usize {
    let rules = include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut it = line.split(' ');
            let pair = (it.next().unwrap(), it.next().unwrap());
            (pair, it.collect::<Vec<_>>())
        })
        .collect::<HashMap<_, _>>();

    let mut stoat_pairs = HashMap::<(&str, &str), usize>::new();
    stoat_pairs.insert(("A", "B"), 1);

    let mut next_stoat_pairs = HashMap::<(&str, &str), usize>::new();

    for _ in 0..gens {
        for (pair, count) in stoat_pairs.drain() {
            let mut prev = pair.0;
            if let Some(children) = rules.get(&pair).or(rules.get(&(pair.1, pair.0))) {
                for &child in children {
                    *next_stoat_pairs.entry((prev, child)).or_default() += count;
                    prev = child;
                }
            }
            *next_stoat_pairs.entry((prev, pair.1)).or_default() += count;
        }
        swap(&mut stoat_pairs, &mut next_stoat_pairs)
    }

    stoat_pairs.values().sum::<usize>() + 1
}

