use crate::modes::Difficulty;
use rand::seq::SliceRandom;

static EASY: &str = include_str!("../../assets/words/easy.txt");
static MEDIUM: &str = include_str!("../../assets/words/medium.txt");
static HARD: &str = include_str!("../../assets/words/hard.txt");
static INSANE: &str = include_str!("../../assets/words/insane.txt");

pub fn load_words(diff: &Difficulty) -> Vec<String> {
    let raw = match diff {
        Difficulty::Easy => EASY,
        Difficulty::Medium => MEDIUM,
        Difficulty::Hard => HARD,
        Difficulty::Insane => INSANE,
    };
    raw.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(String::from)
        .collect()
}

pub fn shuffled_words(diff: &Difficulty) -> Vec<String> {
    let mut words = load_words(diff);
    let mut rng = rand::thread_rng();
    words.shuffle(&mut rng);
    words
}
