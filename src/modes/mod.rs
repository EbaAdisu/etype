pub mod code_snippet;
pub mod sentence;
pub mod survival;
pub mod word_rush;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    WordRush,
    Sentence,
    Code,
    Survival,
}

impl Mode {
    pub fn label(&self) -> &'static str {
        match self {
            Mode::WordRush => "Word Rush",
            Mode::Sentence => "Sentence",
            Mode::Code => "Code Snippets",
            Mode::Survival => "Survival",
        }
    }

    pub fn db_str(&self) -> &'static str {
        match self {
            Mode::WordRush => "word_rush",
            Mode::Sentence => "sentence",
            Mode::Code => "code",
            Mode::Survival => "survival",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Insane,
}

impl Difficulty {
    pub fn unlock_level(&self) -> u32 {
        match self {
            Difficulty::Easy => 1,
            Difficulty::Medium => 2,
            Difficulty::Hard => 3,
            Difficulty::Insane => 4,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
            Difficulty::Insane => "Insane",
        }
    }

    pub fn db_str(&self) -> &'static str {
        match self {
            Difficulty::Easy => "easy",
            Difficulty::Medium => "medium",
            Difficulty::Hard => "hard",
            Difficulty::Insane => "insane",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModeResult {
    pub mode: Mode,
    pub difficulty: Difficulty,
    pub wpm: f64,
    pub cpm: f64,
    pub accuracy: f64,
    pub duration_s: u64,
    pub is_new_best: bool,
    pub xp_earned: u32,
    pub key_stats: HashMap<char, (u32, f64)>,
}
