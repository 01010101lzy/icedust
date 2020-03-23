# IceDust

_This crate uses **Constant Generics** thus must be used in **nightly rust**._

IceDust is an ID generator inspired by Ulid and Twitter Snowflake.

Every IceDust ID is a 64-bit unsigned integer, and is composed of three parts: `Timestamp`, `Machine ID` and `Random`. The three parts can be configured to have different lengths.

## Usage

### Construction

Choose your desired data format. Starting with something similar to Sony's Sonyflake: 39 bits of timestamp at 10ms resolution, 10 bits of machine ID, and monotonic generator switched ON. You can choose any format that fits your need.

The code should look like this:

```rust
let mut gen: IceDustGenerator<_, 39, 10, 10, true>;
//                            |  |   |   |    ^ is monotonic generator switched on?
//                            |  |   |   ^ Machine ID length
//                            |  |   ^ Timestamp resolution (ms)
//                            |  ^ Timestamp bits
//                            ^ RNG type (to be inferred)
```

Choose your desired random generator. The throughput of this algorithm largely depends on the generator, so choose one that satisfies your need. Anyone that implements `rand::Rng` should be fine.

Also specify your epoch timestamp (usually that's Unix epoch), i.e. at which time the timestamp should be zero.

Create the generator with `IceDustGenerator::new_full(rng, machine_id, epoch)` and you're good to go. The code should look like this:

```rust
let rng = rand::rngs::ThreadRng::default();

gen = IceDustGenerator::new(rng, machine_id, SystemTime::UNIX_EPOCH);
```

There's also simpler constructors:

```rust
// this constructor assumes no machine ID and epoch at Unix epoch. You still need to fill in the rest of the parameters.
gen = IceDustGenerator::new_simple(rng);

// this constructor assumes 39 bits of time at 10ms resolution, no machine ID and epoch at Unix epoch. No parameters needed.
gen = IceDustGenerator::new_default(rng);
```

### ID Generation

Generate an ID with `generator.generate()`. Then just use it like any other `u64` ids.

Generate with supplied random value with `generator.generate_with_random(random)`。

## License

MIT.
