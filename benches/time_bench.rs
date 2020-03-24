use icedust::*;
use std::time::{Duration, Instant, SystemTime};

#[macro_use]
extern crate criterion;

use criterion::Criterion;

fn bench_full(c: &mut Criterion) {
    let rng = rand::rngs::ThreadRng::default();
    let mut gen: IceDustGenerator<_, 39, 8, 10, true> =
        IceDustGenerator::new(rng, 0x9728, SystemTime::UNIX_EPOCH);

    c.bench_function("time", |b| b.iter(|| gen.generate()));
}

fn bench_simple(c: &mut Criterion) {
    let rng = rand::rngs::ThreadRng::default();
    let mut gen = IceDustGenerator::new_default(rng);

    c.bench_function("time", |b| b.iter(|| gen.generate()));
}

fn bench_external_entropy(c: &mut Criterion) {
    let rng = rand::rngs::ThreadRng::default();
    let mut gen = IceDustGenerator::new_default(rng);

    c.bench_function("time", |b| b.iter(|| gen.generate_with_random(0x12345678)));
}

criterion_group!(benches, bench_full, bench_simple, bench_external_entropy);
criterion_main!(benches);
