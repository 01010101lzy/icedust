use icedust::*;
use std::time::{Duration, Instant, SystemTime};

#[macro_use]
extern crate criterion;

use criterion::Criterion;

pub fn time_bench() {}

fn criterion_benchmark(c: &mut Criterion) {
    let rng = rand::rngs::ThreadRng::default();
    let mut gen: IceDustGenerator<_, 39, 10, 12, true> =
        IceDustGenerator::new_full(rng, 0x9728, SystemTime::UNIX_EPOCH);

    c.bench_function("time", |b| b.iter(|| gen.generate()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
