
use {
    lazy_static::*,
    rand::{
        distributions::*,
        Rng,
    },
};

pub fn rand_string<R: Rng>(r: &mut R) -> String {
    let len = r.gen_range(3..8) * r.gen_range(1..12);
    rand_string_of_len(r, len)
}

pub fn rand_string_of_len<R: Rng>(r: &mut R, len: usize) -> String {
    r.sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

lazy_static! {
    static ref CHARSET: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 îä\t\nabcdefgh"
        .chars()
        .collect();
}
pub fn rand_text<R: Rng>(r: &mut R) -> String {
    let len = r.gen_range(3..25) * r.gen_range(8..25);
    std::iter::repeat_with(
        || CHARSET[r.gen_range(0..CHARSET.len())]
    ).take(len).collect()
}
