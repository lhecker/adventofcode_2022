use adventofcode_2022::*;
use criterion::{criterion_group, criterion_main, Criterion};

macro_rules! gen_bench {
    ($c:expr, $($day:ident,)*) => {
        $(
            $c.bench_function(stringify!($day), |b| b.iter(|| $day()));
        )*
    };
}

fn criterion_benchmark(c: &mut Criterion) {
    foreach_day!(gen_bench, c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
