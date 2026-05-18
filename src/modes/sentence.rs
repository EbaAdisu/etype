use super::{Difficulty, Mode};
use crate::content::sentences::random_quote;
use crate::engine::scorer::{accuracy, cpm, wpm};
use crate::engine::timer::KeyTimer;
use crate::modes::ModeResult;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub struct SentenceState {
    pub words: Vec<String>,
    pub current_word: usize,
    pub input: String,
    pub difficulty: Difficulty,
    pub correct_chars: u32,
    pub total_keystrokes: u32,
    pub started: bool,
    pub start_time: Option<Instant>,
}

pub struct SentenceSession {
    pub state: SentenceState,
    key_timer: KeyTimer,
    key_stats: HashMap<char, (u32, Vec<f64>)>,
    start: Instant,
}

impl SentenceSession {
    pub fn new(diff: Difficulty) -> Self {
        let quote = random_quote();
        let words: Vec<String> = quote.split_whitespace().map(String::from).collect();
        SentenceSession {
            state: SentenceState {
                words,
                current_word: 0,
                input: String::new(),
                difficulty: diff,
                correct_chars: 0,
                total_keystrokes: 0,
                started: true,
                start_time: Some(Instant::now()),
            },
            key_timer: KeyTimer::new(),
            key_stats: HashMap::new(),
            start: Instant::now(),
        }
    }

    pub fn is_done(&self) -> bool {
        self.state.current_word >= self.state.words.len()
    }

    pub fn push_char(&mut self, c: char) {
        let delay = self.key_timer.elapsed_ms();
        self.key_timer.reset();
        self.state.total_keystrokes += 1;
        let entry = self.key_stats.entry(c).or_insert((0, Vec::new()));
        entry.1.push(delay);

        let target = &self.state.words[self.state.current_word];
        let pos = self.state.input.len();
        if pos < target.len() {
            let expected = target.chars().nth(pos).unwrap_or('\0');
            if c == expected {
                self.state.correct_chars += 1;
            } else {
                let e = self.key_stats.entry(c).or_insert((0, Vec::new()));
                e.0 += 1;
            }
        }
        self.state.input.push(c);
    }

    pub fn backspace(&mut self) {
        self.state.input.pop();
    }

    pub fn ctrl_w(&mut self) {
        self.state.input.clear();
    }

    pub fn confirm_word(&mut self) {
        let target = self.state.words[self.state.current_word].clone();
        if self.state.input == target {
            self.state.correct_chars += 1;
            self.state.current_word += 1;
            self.state.input.clear();
        } else {
            self.state.input.clear();
        }
    }

    pub fn finish(&self, is_new_best: bool, xp_earned: u32) -> ModeResult {
        let elapsed = self.start.elapsed().as_secs_f64();
        let w = wpm(self.state.correct_chars, elapsed);
        let c = cpm(self.state.correct_chars, elapsed);
        let a = accuracy(self.state.correct_chars, self.state.total_keystrokes);
        let key_stats = self
            .key_stats
            .iter()
            .map(|(ch, (errors, delays))| {
                let avg = if delays.is_empty() {
                    0.0
                } else {
                    delays.iter().sum::<f64>() / delays.len() as f64
                };
                (*ch, (*errors, avg))
            })
            .collect();
        ModeResult {
            mode: Mode::Sentence,
            difficulty: self.state.difficulty.clone(),
            wpm: w,
            cpm: c,
            accuracy: a,
            duration_s: elapsed as u64,
            is_new_best,
            xp_earned,
            key_stats,
        }
    }
}
