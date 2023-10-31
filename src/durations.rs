use {
    std::time::{
        Duration,
        Instant,
    },
    termimad::{
        minimad::*,
        *,
    },
};

static TEMPLATE: &str = r#"
Fastest deserializer: **${fastest}**
|:-:|:-:|:-:|
|**crate**|**sum durations**|**diff with fastest**|**throughput**|
|:-:|-:|-:|-:|
${entry
|${name}|${duration}|**${diff}**|${throughput}|
}
|:-:|-:|-:|-:|
"#;

#[derive(Debug, Clone)]
pub(crate) struct Durations {
    entries: Vec<Entry>,
    samples_count: usize,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct Entry {
    pub name: &'static str,
    pub duration: Duration,
    pub src_len: usize,
}

impl Default for Durations {
    fn default() -> Self {
        Self {
            entries: Vec::default(),
            samples_count: 1,
        }
    }
}

impl Entry {
    fn diff(
        &self,
        other: &Entry,
    ) -> f32 {
        let s = self.duration.as_nanos() as f32;
        let o = other.duration.as_nanos() as f32;
        (s - o) / o // this will break if one duration is 0
    }
    /// return the number of Gb read per second
    #[allow(dead_code)]
    fn throughput_gbps(&self) -> f32 {
        (self.src_len as f32) / (self.duration.as_nanos() as f32)
    }
    /// return the number of Mb read per second
    fn throughput_mbps(&self) -> f32 {
        (self.src_len as f32) / (self.duration.as_micros() as f32)
    }
}

impl Durations {
    pub fn add(
        &mut self,
        name: &'static str,
        start: Instant,
        src_len: usize,
    ) {
        self.entries.push(Entry {
            name,
            duration: start.elapsed(),
            src_len,
        });
    }
    /// find the matching entry, panic if it's not found
    #[allow(dead_code)]
    pub fn get(
        &self,
        name: &'static str,
    ) -> &Entry {
        self.entries.iter().find(|e| e.name == name).unwrap()
    }
    /// sum all durations, panic if they don't have the same
    /// entries, and give unconsistent results if they're not
    /// in the same order
    pub fn sum(samples: &[Durations]) -> Self {
        if samples.is_empty() {
            return Self::default();
        }
        let mut sum = samples[0].clone();
        for (idx, entry) in sum.entries.iter_mut().enumerate() {
            for sample in samples.iter().skip(1) {
                entry.duration += sample.entries[idx].duration;
                entry.src_len += sample.entries[idx].src_len;
            }
        }
        sum.samples_count = samples.len();
        sum
    }
    // use all computed values to ensure the optimizer doesn't
    // remove some computations
    pub fn check(&self) {
        let s = self
            .entries
            .iter()
            .fold(0, |s, e| s + e.duration.as_nanos());
        assert!(s > 0);
    }
    pub fn fastest(&self) -> &Entry {
        self.entries.iter().min_by_key(|e| e.duration).unwrap()
    }
    pub fn print(mut self) {
        self.entries.sort_by_key(|e| e.duration);
        let skin = MadSkin::default();
        let mut expander = OwningTemplateExpander::new();
        let fastest = self.fastest();
        expander.set("fastest", fastest.name);
        for entry in &self.entries {
            expander
                .sub("entry")
                .set("name", entry.name)
                .set("duration", format!("{:?}", entry.duration))
                .set("diff", format!("+{:.0}%", entry.diff(fastest) * 100.0))
                .set("throughput", format!("{:.0} Mb/s", entry.throughput_mbps()));
        }
        let template = TextTemplate::from(TEMPLATE);
        let text = expander.expand(&template);
        let (width, _) = terminal_size();
        let fmt_text = FmtText::from_text(&skin, text, Some(width as usize));
        print!("{}", fmt_text);
    }
}
