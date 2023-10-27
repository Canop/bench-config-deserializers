use {
    crate::random::*,
    rand::Rng,
    serde::{
        Deserialize,
        Serialize,
    },
    std::collections::HashMap,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Thing {
    name: String,
    surname: Option<String>,
    var_things: Vec<VarThing>,
    stuff: HashMap<String, Stuff>,
    bidules: Vec<Bidule>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DetailedThing {
    key: String,
    value: f32,
    coords: (f64, f64),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cost: Option<u8>,
    text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum VarThing {
    Simple(String),
    Detailed(DetailedThing),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Bidule {
    id: String,
    numbers: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Stuff {
    pub name: Option<String>,
    #[serde(default)]
    pub quantity: u16,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub var_thing: Option<VarThing>,
    #[serde(default)]
    pub good: Option<bool>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub condition: Condition,
    pub x: i16,
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Condition {
    #[default]
    Any,
    Directory,
    File,
    TextFile,
    BinaryFile,
}
pub static CONDITIONS: &[Condition] = &[
    Condition::Any,
    Condition::Directory,
    Condition::File,
    Condition::TextFile,
    Condition::BinaryFile,
];

impl Thing {
    pub fn new<R: Rng>(mut r: R) -> Self {
        let var_things = (0..r.gen_range(1400..1500))
            .map(|_| VarThing::new(&mut r))
            .collect();
        let stuff = (0..r.gen_range(1500..1800))
            .map(|_| (rand_string(&mut r), Stuff::new(&mut r)))
            .collect();
        let bidules = (0..r.gen_range(1500..2000))
            .map(|_| Bidule::new(&mut r))
            .collect();
        Self {
            name: rand_string(&mut r),
            surname: Some(rand_string(&mut r)),
            var_things,
            stuff,
            bidules,
        }
    }
}

impl DetailedThing {
    pub fn new<R: Rng>(mut r: R) -> Self {
        let cost = if r.gen() { Some(r.gen()) } else { None };
        let text = rand_text(&mut r);
        let coords = (r.gen(), r.gen());
        Self {
            key: rand_string(&mut r),
            value: r.gen(),
            coords,
            cost,
            text,
        }
    }
}

impl VarThing {
    pub fn new<R: Rng>(mut r: R) -> Self {
        if r.gen() {
            Self::Simple(rand_string(&mut r))
        } else {
            Self::Detailed(DetailedThing::new(&mut r))
        }
    }
}

impl Stuff {
    pub fn new<R: Rng>(mut r: R) -> Self {
        let var_thing = if r.gen() {
            Some(VarThing::new(&mut r))
        } else {
            None
        };
        let good = if r.gen() { Some(r.gen()) } else { None };
        let tags = (0..r.gen_range(5..15))
            .map(|_| rand_string_of_len(&mut r, 4))
            .collect();
        let condition = CONDITIONS[r.gen_range(0..CONDITIONS.len())];
        let x = r.gen();
        Self {
            name: Some(rand_string(&mut r)),
            quantity: r.gen_range(3..500),
            good,
            tags,
            var_thing,
            condition,
            x,
        }
    }
}

impl Bidule {
    pub fn new<R: Rng>(mut r: R) -> Self {
        let id = rand_string_of_len(&mut r, 10);
        let numbers = (0..20).map(|_| r.gen()).collect();
        Self { id, numbers }
    }
}
