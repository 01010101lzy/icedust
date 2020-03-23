use icedust::*;
use std::time::{Duration, Instant, SystemTime};

#[macro_use]
extern crate criterion;

use criterion::Criterion;

fn bench_full(c: &mut Criterion) {
    let rng = rand::rngs::ThreadRng::default();
    let mut gen: IceDustGenerator<_, 39, 10, 12, true> =
        IceDustGenerator::new(rng, 0x9728, SystemTime::UNIX_EPOCH);

    c.bench_function("time", |b| b.iter(|| gen.generate()));
}

fn bench_simple(c: &mut Criterion) {
    let rng = rand::rngs::ThreadRng::default();
    let mut gen = IceDustGenerator::new_default(rng);

    c.bench_function("time", |b| b.iter(|| gen.generate()));
}

criterion_group!(benches, bench_full, bench_simple);
criterion_main!(benches);
