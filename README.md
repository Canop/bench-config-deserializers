
This program compares the time some [serde](https://serde.rs/) deserializers take to deserialize some string into a configuration-like struct deriving `Deserialize`.

The benchmarker also checks the correct round-trip by checking equality of the deserialized config with the source struct (this involves enabling the `float_roundtrip` feature for serde_json).

A configuration file needs comments, and needs to be convenient enough to be written by humans.
For those reasons, JSON isn't suitable, so this benchmark is really dedicated to [Hjson](https://hjson.github.io/), [JSON5](https://json5.org/), [YAML](https://en.wikipedia.org/wiki/YAML), and [TOML](https://toml.io/). For a deeper discussion regarding the choice of a configuration format, read [this blog post about configuration formats](https://dystroy.org/blog/hjson-in-broot/)).

The [serde-json](https://docs.rs/serde_json/), [deser_hjson](https://docs.rs/deser-hjson/), [sonic-rs](https://docs.rs/sonic-rs/), and [json5](https://docs.rs/json5) deserializers are measured with the same JSON file built by serde_json with `to_string_pretty`.

serde_json and sonic_rs are advantaged here, because they don't need to test for meany things you'd normally find in configurations: comments, multi-line texts, alternate ways to write data.
They're still interesting as reference points for other deserializers as long as you remember they're not exactly doing the same work.

In this benchmark, the JSON5 deserializer appears slower than other ones.
It's very probable it doesn't matter for you: deserializing a standard configuration is still done in less than 10 ms.

The [toml](https://docs.rs/toml/) deserializer is tested with the same struct, but encoded in a TOML string.
The [serde_yaml](https://docs.rs/serde_yaml/) deserializer is tested with the same struct, but encoded in a YAML string.

The struct used in this bench is bigger than usual configuration files but otherwise should be quite alike usual configurations.
It is generated 10 times with different random seeds.

Here are the results I get on my computer:

    Fastest deserializer: serde_json
    ┌────────────┬─────────────┬─────────────────┬──────────┐
    │deserializer│sum durations│diff with fastest│throughput│
    ├────────────┼─────────────┼─────────────────┼──────────┤
    │ serde_json │  38.061465ms│              +0%│  521 Mb/s│
    │  sonic-rs  │  45.537732ms│             +20%│  436 Mb/s│
    │deser-hjson │   87.78362ms│            +131%│  226 Mb/s│
    │   json5    │ 817.185345ms│           +2047%│   24 Mb/s│
    │    toml    │ 436.907208ms│           +1048%│   31 Mb/s│
    │ serde_yaml │ 321.301263ms│            +744%│   47 Mb/s│
    └────────────┴─────────────┴─────────────────┴──────────┘
    (a smaller "diff with fastest" is better)

To test the benchmark yourself with your hardware, use

    cargo run +nightly --release

The `+nightly` is required by sonic_rs.


If you think some common or tricky patterns aren't well tested, that a config deserializer is missing, that I made an error, etc. please create an issue or contact me on Miaou.
