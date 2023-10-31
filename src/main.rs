mod durations;
mod random;
mod thing;

use {
    durations::*,
    pretty_assertions::assert_eq,
    rand::prelude::*,
    std::time::Instant,
    thing::*,
};

fn compute(
    seed: u64,
    verbose: bool,
) -> Durations {
    let mut durations = Durations::default();
    let mut rng = StdRng::seed_from_u64(seed);
    let thing = Thing::new(&mut rng);

    let json = serde_json::to_string_pretty(&thing).unwrap();
    if verbose {
        //println!("JSON:\n{}", &json);
        println!("pretty JSON length: {} bytes", json.len());
    }
    let toml = toml::to_string(&thing).unwrap();
    if verbose {
        println!("TOML length: {} bytes", toml.len());
    }
    let yaml = serde_yaml::to_string(&thing).unwrap();
    if verbose {
        println!("YAML length: {} bytes", yaml.len());
    }

    let start = Instant::now();
    let c: Thing = serde_json::from_str(&json).unwrap();
    durations.add("serde_json", start, json.len());
    assert_eq!(thing, c);

    let start = Instant::now();
    let c: Thing = sonic_rs::from_str(&json).unwrap();
    durations.add("sonic-rs", start, json.len());
    assert_eq!(thing, c);

    let start = Instant::now();
    let c: Thing = deser_hjson::from_str(&json).unwrap();
    durations.add("deser-hjson", start, json.len());
    assert_eq!(thing, c);

    let start = Instant::now();
    let c: Thing = json5::from_str(&json).unwrap();
    durations.add("json5", start, json.len());
    assert_eq!(thing, c);

    let start = Instant::now();
    let c: Thing = toml::from_str(&toml).unwrap();
    durations.add("toml", start, toml.len());
    assert_eq!(thing, c);

    let start = Instant::now();
    let c: Thing = basic_toml::from_str(&toml).unwrap();
    durations.add("basic-toml", start, toml.len());
    assert_eq!(thing, c);

    let start = Instant::now();
    let c: Thing = serde_yaml::from_str(&yaml).unwrap();
    durations.add("serde_yaml", start, yaml.len());
    assert_eq!(thing, c);

    durations
}

fn main() {
    let durations = compute(0, true);
    durations.check();
    println!("warming...");
    for i in 0..10 {
        let durations = compute(i, false);
        durations.check();
    }

    let samples: Vec<Durations> = (1..=10)
        .map(|i| {
            println!("{i}...");
            compute(i, false)
        })
        .collect();
    println!("...done");
    let durations = Durations::sum(&samples);
    durations.print();
}
