use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

#[rustfmt::skip]
macro_rules! problems {
    ($($problem:ident),*$(,)?) => {
        pub fn ev_benchmark_full(c: &mut Criterion) {
            $(c.bench_function(stringify!($problem), |b| b.iter(|| black_box($problem::solve())));)+
            c.bench_function("all", |b| b.iter(|| ($(black_box($problem::solve())),+)));
        }

        criterion_group! {
            name = benches;
            config = Criterion::default();
            targets = ev_benchmark_full
        }

        criterion_main!{
            benches
        }
    };
}

#[rustfmt::skip]
problems!(
    problem_a
    problem-b,
    problem-c,
    problem-e,
);
