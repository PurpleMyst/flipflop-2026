use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet, hash_map::Entry},
    fmt::Display,
    mem::swap,
};

use itertools::Itertools;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

type Rules = HashMap<&'static str, (Option<&'static str>, Option<&'static str>, Option<&'static str>)>;
type Sprouts = HashMap<(isize, isize), &'static str>;
type Stems = HashSet<(isize, isize)>;

fn parse_input() -> impl Iterator<Item = Rules> {
    fn p(s: &str) -> Option<&str> {
        (s != "XX").then_some(s)
    }
    include_str!("input.txt")
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.split_ascii_whitespace())
        .tuples()
        .map(|(above, row)| {
            row.tuples()
                .zip(above)
                .map(|((left, cur, right), above)| (cur, (p(left), p(above), p(right))))
                .collect::<Rules>()
        })
}

#[inline]
pub fn solve_part1() -> impl Display {
    let mut total = 0;
    for rules in parse_input() {
        let mut sprouts = Sprouts::new();
        sprouts.insert((0, 0), "00");
        let mut stems = Stems::new();

        for age in 1..=100 {
            let mut next_sprouts = Sprouts::new();
            for ((y, x), id) in sprouts.drain() {
                let (left, above, right) = rules[id];
                grow(&mut next_sprouts, &stems, y, x - 1, left);
                grow(&mut next_sprouts, &stems, y, x + 1, right);
                grow(&mut next_sprouts, &stems, y + 1, x, above);
                stems.insert((y, x));
            }
            swap(&mut sprouts, &mut next_sprouts);
            if age >= 5 && (harvested_energy(&stems, &stems, age) < requested_energy(&sprouts, &stems)) {
                break;
            }
        }

        total += sprouts.len() + stems.len();
    }
    total
}

fn requested_energy(sprouts: &Sprouts, stems: &Stems) -> usize {
    3 * (sprouts.len() + stems.len())
}

fn harvested_energy(stems: &Stems, blockers: &Stems, age: isize) -> usize {
    stems
        .iter()
        .map(|&(y, x)| {
            (y as usize + 1).min(10)
                * 3usize.saturating_sub((y + 1..=age).filter(|&y| blockers.contains(&(y, x))).take(3).count())
        })
        .sum::<usize>()
}

fn grow(next_sprouts: &mut Sprouts, stems: &HashSet<(isize, isize)>, y: isize, x: isize, child: Option<&'static str>) {
    let Some(child) = child else {
        return;
    };
    if stems.contains(&(y, x)) {
        return;
    }
    match next_sprouts.entry((y, x)) {
        Entry::Occupied(mut occupied_entry) => {
            if *occupied_entry.get() < child {
                occupied_entry.insert(child);
            }
        }
        Entry::Vacant(vacant_entry) => {
            vacant_entry.insert(child);
        }
    }
}

#[inline]
pub fn solve_part2() -> impl Display {
    let all_rules = parse_input().collect::<Vec<_>>();

    let mut all_sprouts = vec![Sprouts::new(); all_rules.len()];
    for (i, sprouts) in all_sprouts.iter_mut().enumerate() {
        sprouts.insert((0, 10 * i as isize), "00");
    }
    let all_stems = evolve_p2(&all_rules, &mut all_sprouts);
    all_sprouts.iter().flat_map(|s| s.keys()).count() + all_stems.iter().flatten().count()
}

fn evolve_p2(all_rules: &[Rules], all_sprouts: &mut [Sprouts]) -> Vec<Stems> {
    let mut all_stems = vec![Stems::new(); all_rules.len()];
    let mut all_dead = vec![false; all_rules.len()];

    for age in 1..=100 {
        let mut occupied: Stems = all_stems
            .iter()
            .flatten()
            .copied()
            .chain(all_sprouts.iter().flat_map(|s| s.keys()).copied())
            .collect();

        for (dead, (rules, (sprouts, stems))) in all_dead
            .iter()
            .zip(all_rules.iter().zip(all_sprouts.iter_mut().zip(all_stems.iter_mut())))
        {
            if *dead {
                continue;
            }

            let mut next_sprouts = Sprouts::new();
            for ((y, x), id) in sprouts.drain() {
                let (left, above, right) = rules[id];
                grow(&mut next_sprouts, &occupied, y, x - 1, left);
                grow(&mut next_sprouts, &occupied, y, x + 1, right);
                grow(&mut next_sprouts, &occupied, y + 1, x, above);
                stems.insert((y, x));
            }
            swap(sprouts, &mut next_sprouts);
            occupied.extend(sprouts.keys().copied());
        }

        if age >= 5 {
            let all_stems_flat: Stems = all_stems.iter().flatten().copied().collect();
            for (i, (sprouts, stems)) in all_sprouts.iter_mut().zip(all_stems.iter()).enumerate() {
                if harvested_energy(stems, &all_stems_flat, age) < requested_energy(sprouts, stems) {
                    all_dead[i] = true;
                }
            }
        }
    }
    all_stems
}

#[inline]
pub fn solve_part3() -> impl Display {
    let mut all_rules = parse_input().collect::<Vec<_>>();

    let mut all_sprouts = vec![Sprouts::new(); all_rules.len()];
    for (i, sprouts) in all_sprouts.iter_mut().enumerate() {
        sprouts.insert((0, 10 * i as isize), "00");
    }

    let mut all_stems = evolve_p2(&all_rules, &mut all_sprouts);
    for _ in 0..2 {
        (all_sprouts, all_rules) = all_sprouts
            .iter()
            .enumerate()
            .flat_map(|(i, sprouts)| sprouts.keys().map(move |&p| (i, p)))
            .sorted_by_key(|&(_, (y, x))| (x, Reverse(y)))
            .unique_by(|&(_, (_, x))| x)
            .map(|(i, (_, x))| {
                let mut sprouts = Sprouts::new();
                sprouts.insert((0, x), "00");
                (sprouts, all_rules[i].clone())
            })
            .unzip();
        all_stems = evolve_p2(&all_rules, &mut all_sprouts);
    }

    all_sprouts.iter().flat_map(|s| s.keys()).count() + all_stems.iter().flatten().count()
}
