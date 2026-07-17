use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

#[rustfmt::skip]
macro_rules! problems {
    ($($problem:ident),*$(,)?) => {
        pub fn benchmark_full(c: &mut Criterion) {
            $(c.bench_function(stringify!($problem), |b| b.iter(|| black_box($problem::solve())));)*
            c.bench_function("all", |b| b.iter(|| black_box(($(black_box($problem::solve())),*))));
        }

        pub fn benchmark_parts(c: &mut Criterion) {
            let _ = c;
            $(c.bench_function(concat!(stringify!($problem), "/part1"), |b| b.iter(|| black_box($problem::solve_part1())));)*
            $(c.bench_function(concat!(stringify!($problem), "/part2"), |b| b.iter(|| black_box($problem::solve_part2())));)*
            $(c.bench_function(concat!(stringify!($problem), "/part3"), |b| b.iter(|| black_box($problem::solve_part3())));)*
        }

        criterion_group! {
            name = benches;
            config = Criterion::default();
            targets = benchmark_full, benchmark_parts
        }

        criterion_main!{
            benches
        }
    };
}

#[rustfmt::skip]
problems!(
    puzzle01,
    puzzle02,
    puzzle03,
    puzzle04,
    puzzle05,
    puzzle06,
    puzzle07,
    puzzle08,
    puzzle09,
    puzzle10,
    puzzle11,
    puzzle12,
);
