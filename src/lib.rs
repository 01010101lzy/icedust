#![feature(const_generics)]
#![feature(atomic_min_max)]

mod test;

use rand::Rng;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::time::SystemTime;

/// An IceDust ID generator. This generator is sharable between threads.
pub struct IceDustGenerator<
    G: Rng,
    const TIMESTAMP_BITS: u8,
    const TIMESTAMP_RESOLUTION: u64,
    const MACHINE_ID_BITS: u8,
    const MONOTONIC: bool,
> {
    generator: G,
    machine_id: u64,
    epoch: SystemTime,
    // Only used when MONOTONIC=true
    last_timestamp: u64,
    // Only used when MONOTONIC=true
    last_random: u64,
}

impl<G: Rng, const TIMESTAMP_BITS: u8, const TIMESTAMP_RESOLUTION: u64, const MONOTONIC: bool>
    IceDustGenerator<G, TIMESTAMP_BITS, TIMESTAMP_RESOLUTION, 0, MONOTONIC>
{
    /// this constructor uses no machine ID and epoch at Unix epoch. You still need to fill in the rest of the parameters.
    pub fn new_simple(
        generator: G,
    ) -> IceDustGenerator<G, TIMESTAMP_BITS, TIMESTAMP_RESOLUTION, 0, MONOTONIC> {
        assert!((TIMESTAMP_BITS) < 64);

        IceDustGenerator {
            generator,
            machine_id: 0,
            epoch: SystemTime::UNIX_EPOCH,
            last_timestamp: 0,
            last_random: 0,
        }
    }
}

impl<G: Rng> IceDustGenerator<G, 39, 10, 0, true> {
    /// this constructor uses 39 bits of time at 10ms resolution, no machine ID and epoch at Unix epoch. No parameters needed.
    pub fn new_default(generator: G) -> IceDustGenerator<G, 39, 10, 0, true> {
        IceDustGenerator {
            generator,
            machine_id: 0,
            epoch: SystemTime::UNIX_EPOCH,
            last_timestamp: 0,
            last_random: 0,
        }
    }
}

impl<
        G: Rng,
        const TIMESTAMP_BITS: u8,
        const TIMESTAMP_RESOLUTION: u64,
        const MACHINE_ID_BITS: u8,
        const MONOTONIC: bool,
    > IceDustGenerator<G, TIMESTAMP_BITS, TIMESTAMP_RESOLUTION, MACHINE_ID_BITS, MONOTONIC>
{
    const RANDOM_BITS: u8 = 64 - TIMESTAMP_BITS - MACHINE_ID_BITS;

    pub fn new(
        generator: G,
        machine_id: u64,
        epoch: SystemTime,
    ) -> IceDustGenerator<G, TIMESTAMP_BITS, TIMESTAMP_RESOLUTION, MACHINE_ID_BITS, MONOTONIC> {
        assert!((TIMESTAMP_BITS + MACHINE_ID_BITS) < 64);
        let machine_id = machine_id & ((1 << MACHINE_ID_BITS) - 1);

        IceDustGenerator {
            generator,
            machine_id,
            epoch,
            last_timestamp: 0,
            last_random: 0,
        }
    }

    #[inline]
    fn get_random(&mut self, simple_inc: bool) -> u64 {
        let random = if simple_inc {
            // Monotonic update
            self.last_random += 1;
            self.last_random
        } else {
            let rnd = self.generator.next_u64();
            self.last_random = rnd;
            rnd
        };
        let random = random & ((1 << Self::RANDOM_BITS) - 1);
        random
    }

    #[inline]
    fn get_timestamp(&mut self) -> Option<(u64, bool)> {
        let timestamp = SystemTime::now()
            .duration_since(self.epoch)
            .unwrap()
            .as_millis();

        if timestamp >= ((1 << TIMESTAMP_BITS) * TIMESTAMP_RESOLUTION) as u128 {
            // Check timestamp range. Should be cold path.
            return None;
        };
        let timestamp = timestamp as u64;
        let timestamp = timestamp / TIMESTAMP_RESOLUTION;

        let last = self.last_timestamp;
        self.last_timestamp = timestamp;
        Some((timestamp, timestamp == last))
    }

    pub fn generate(&mut self) -> Option<u64> {
        let (timestamp, inc) = self.get_timestamp()?;
        let simple_inc = inc && MONOTONIC;
        let random = self.get_random(simple_inc);
        let res = (timestamp << (MACHINE_ID_BITS + Self::RANDOM_BITS))
            | (self.machine_id << (Self::RANDOM_BITS))
            | random;

        Some(res)
    }

    pub fn generate_with_random(&mut self, random: u64) -> Option<u64> {
        let (timestamp, _) = self.get_timestamp()?;
        let res = (timestamp << (MACHINE_ID_BITS + Self::RANDOM_BITS))
            | (self.machine_id << (Self::RANDOM_BITS))
            | random;
        Some(res)
    }
}
