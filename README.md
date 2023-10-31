
## Purpose and methodology

This program compares the time some [serde](https://serde.rs/) deserializers take to deserialize some string into a configuration-like struct deriving `Deserialize`.

The benchmarker also checks the correct round-trip by checking equality of the deserialized config with the source struct (this involves enabling the `float_roundtrip` feature for serde_json).

A configuration file needs comments, and needs to be convenient enough to be written by humans.
For those reasons, JSON isn't suitable, so this benchmark is really dedicated to [Hjson](https://hjson.github.io/), [JSON5](https://json5.org/), [YAML](https://en.wikipedia.org/wiki/YAML), and [TOML](https://toml.io/). For a deeper discussion regarding the choice of a configuration format, read [this blog post about configuration formats](https://dystroy.org/blog/hjson-in-broot/)).

The struct used in this bench is bigger than usual configuration files but otherwise should be quite alike usual configurations.
It is generated 10 times with different random seeds, and stored in memory to avoid disk IO perturbing the measurement.

## JSON

The [serde-json](https://docs.rs/serde_json/), [deser_hjson](https://docs.rs/deser-hjson/), [sonic-rs](https://docs.rs/sonic-rs/), and [json5](https://docs.rs/json5) deserializers are measured with the same JSON file built by serde_json with `to_string_pretty`.

serde_json and sonic_rs are advantaged here, because they don't need to test for meany things you'd normally find in configurations: comments, multi-line texts, alternate ways to write data.
They're still interesting as reference points for other deserializers as long as you remember they're not exactly doing the same work.

In this benchmark, the JSON5 deserializer appears slower than other ones.
It's very probable it doesn't matter for you: deserializing a standard configuration is still done in less than 10 ms.

## TOML

The [toml](https://docs.rs/toml/) and [basic-toml](https://docs.rs/basic-toml/) deserializers are tested with the same struct, but encoded in a TOML string.

## YAML

The [serde_yaml](https://docs.rs/serde_yaml/) deserializer is tested with the same struct, but encoded in a YAML string.

## Results

Here are the results I get on my computer:

    Fastest deserializer: serde_json
    ┌───────────┬─────────────┬─────────────────┬──────────┐
    │   crate   │sum durations│diff with fastest│throughput│
    ├───────────┼─────────────┼─────────────────┼──────────┤
    │serde_json │   40.79965ms│              +0%│  506 Mb/s│
    │ sonic-rs  │  43.144377ms│              +6%│  478 Mb/s│
    │deser-hjson│  91.190384ms│            +124%│  226 Mb/s│
    │serde_yaml │ 341.829977ms│            +738%│   46 Mb/s│
    │basic-toml │ 361.738851ms│            +787%│   39 Mb/s│
    │   toml    │ 466.519594ms│           +1043%│   31 Mb/s│
    │   json5   │ 854.551969ms│           +1995%│   24 Mb/s│
    └───────────┴─────────────┴─────────────────┴──────────┘

A smaller "diff with fastest" is better, it's based on the sum of the durations of 10 random strings, with a size varying between 1 and 2 MB.

The througput is a little less relevant as some formats are more compact. In the specific serialization we do here, the TOML string is smaller than the JSON string (but depending on how you write it yourself, you may get different results).

To test the benchmark yourself with your hardware, use

    cargo run +nightly --release

The `+nightly` is required by sonic_rs.

If you think some common or tricky patterns aren't well tested, that a config deserializer is missing, that I made an error, etc. please create an issue or contact me on [Miaou](https://miaou.dystroy.org/3768).
