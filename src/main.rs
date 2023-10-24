mod thing;
mod random;

use {
    pretty_assertions::assert_eq,
    rand::{
        prelude::*,
    },
    std::time::{Instant, Duration},
    thing::*,
};

#[derive(Debug, Default)]
struct Durations {
    pub serde_json: Duration,
    pub deser_hjson: Duration,
    pub sonic_rs: Duration,
    pub json5: Duration,
    pub toml: Duration,
}

impl Durations {
    pub fn compute(thing: &Thing, verbose: bool) -> Self {
        let json = serde_json::to_string_pretty(&thing).unwrap();
        if verbose {
            //println!("JSON:\n{}", &json);
            println!("pretty JSON length: {} bytes", json.len());
        }

        let start = Instant::now();
        let c: Thing = serde_json::from_str(&json).unwrap();
        let serde_json = start.elapsed();
        assert_eq!(thing, &c);

        let start = Instant::now();
        let c: Thing = deser_hjson::from_str(&json).unwrap();
        let deser_hjson = start.elapsed();
        assert_eq!(thing, &c);

        let start = Instant::now();
        let c: Thing = sonic_rs::from_str(&json).unwrap();
        let sonic_rs = start.elapsed();
        assert_eq!(thing, &c);

        let start = Instant::now();
        let c: Thing = json5::from_str(&json).unwrap();
        let json5 = start.elapsed();
        assert_eq!(thing, &c);

        let toml = toml::to_string(&thing).unwrap();
        if verbose {
            println!("TOML length: {} bytes", toml.len());
        }

        let start = Instant::now();
        let c: Thing = toml::from_str(&toml).unwrap();
        let toml = start.elapsed();
        assert_eq!(thing, &c);

        Self { deser_hjson, serde_json, sonic_rs, json5, toml }
    }
    /// use all values, thus ensuring their computation
    /// isn't optimized out
    pub fn check(&self) {
        assert!(self.serde_json.as_nanos() > 0);
        assert!(self.deser_hjson.as_nanos() > 0);
        assert!(self.sonic_rs.as_nanos() > 0);
        assert!(self.json5.as_nanos() > 0);
        assert!(self.toml.as_nanos() > 0);
    }
    pub fn deser_hjson_ratio(&self) -> f32 {
        (self.deser_hjson.as_nanos() as f32) / (self.serde_json.as_nanos() as f32)
    }
    pub fn sum(samples: &[Self]) -> Self {
        let mut sum = Durations::default();
        for sample in samples {
            sum.serde_json += sample.serde_json;
            sum.deser_hjson += sample.deser_hjson;
            sum.sonic_rs += sample.sonic_rs;
            sum.json5 += sample.json5;
            sum.toml += sample.toml;
        }
        sum
    }
}

fn main() {
    let mut rng = StdRng::seed_from_u64(0);
    let thing = Thing::new(&mut rng);

    println!("warming...");
    let durations = Durations::compute(&thing, true);
    durations.check();

    let samples: Vec<Durations> = (1..=10)
        .map(|i| {
            println!("{i}...");
            Durations::compute(&thing, false)
        })
        .collect();
    println!("\ndone");
    let durations = Durations::sum(&samples);
    dbg!(&durations);
    durations.check();
    println!("deser_hjson / serde_json : {:.1}%", (durations.deser_hjson_ratio() * 100.0));
}
