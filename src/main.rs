use icedust::*;
use std::time::{Duration, Instant, SystemTime};

pub fn main() {
    let rng = rand::rngs::ThreadRng::default();
    let mut gen: IceDustGenerator<_, 39, 100, 12, true> =
        IceDustGenerator::new_full(rng, 0x9728, SystemTime::UNIX_EPOCH);
}
