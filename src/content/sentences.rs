use rand::seq::SliceRandom;
use std::sync::OnceLock;

static QUOTES_RAW: &str = include_str!("../../assets/sentences/quotes.txt");

fn quotes() -> &'static [&'static str] {
    static CACHE: OnceLock<Vec<&'static str>> = OnceLock::new();
    CACHE.get_or_init(|| {
        QUOTES_RAW
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .collect()
    })
}

pub fn random_quote() -> &'static str {
    let list = quotes();
    let mut rng = rand::thread_rng();
    list.choose(&mut rng)
        .copied()
        .unwrap_or("The quick brown fox jumps over the lazy dog.")
}
