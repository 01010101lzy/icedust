# IceDust

_This crate uses **Constant Generics** thus must be used in **nightly rust**._

IceDust is an ID generator inspired by Ulid and Twitter Snowflake.

Every IceDust ID is a 64-bit unsigned integer, and is composed of three parts: `timestamp`, `machine_id` and `random`. The three parts can be configured to have different lengths. 

## Format

There are 5 configurable parameters in IceDust: `timestamp_bits`, `timestamp_resolution`, `machine_id_bits`, `monotonic` and `epoch`. They are used in the following manner:

- `timestamp_bits` controlls how many bits will be used to represent `timestamp`. Defaults to 39.
- `timestamp_resolution` controlls how long 1 `timestamp` unit is in milliseconds. Defaults to 10.
- `machine_id_bits` controlls how many bits will be used to represent `machine_id`. Defaults to 0, recommended to be around 10 for multiple machines.
- `monotonic` controlls whether to simply increase the random value when generating when timestamp is the same (aka counter). Defaults to be `true`. The value wraps around when reaching max value.
- `epoch` is the starting point of counting `timestamp`. Defaults to be Unix Epoch.

The parts are simply combined together into a 64-bit unsigned integer. THe following is a sample bit representation of an IceDust ID with 39 bits of timestamp and 10 bits of machine ID.

```
  0        8        16       24       32       40       48       56       64
  tttttttt tttttttt tttttttt tttttmmm mmmmmmmr rrrrrrrr rrrrrrrr rrrrrrrr
  |--         timestamp        --|| machine ||--       random         --|
```

## Usage

### Generator

Construct a generator for the default format using the default configuration constructor:

```rust
// this constructor assumes 39 bits of time at 10ms resolution, no machine ID and 
// epoch at Unix epoch. You still need to provide a random number generator that 
// implements `rand::Rng` here.
gen = IceDustGenerator::new_default(rng);
```

Alternatively, you can create a custom-format generator with the following code:

```rust
let mut gen: IceDustGenerator<_, 39, 10, 10, true>;
//                            |  |   |   |    ^ is monotonic generator switched on?
//                            |  |   |   ^ Machine ID length
//                            |  |   ^ Timestamp resolution (ms)
//                            |  ^ Timestamp bits
//                            ^ RNG type (to be inferred)

let rng = rand::rngs::ThreadRng::default();

// Construct with full control
gen = IceDustGenerator::new(rng, machine_id, SystemTime::UNIX_EPOCH);

// Construct with no machine ID and Unix epoch
gen = IceDustGenerator::new_simple(rng);
```

### ID Generation

Generate an ID with `generator.generate()`. Alternatively, generate with supplied random value with `generator.generate_with_random(random)`.

Both generation methods return a `Option<u64>`, returning `None` if the generator refuses to generate an ID.

Situations when the generator will refuse to generate:

- Timestamp goes backward (last generation timestamp < this generation timestamp)
- Timestamp cannot be represented in specified bits

## Performance

Fast. About 45ns/iter in default config and `rand::rngs::ThreadRng` random source on an AMD Ryzen 3700X.

## License

MIT.
