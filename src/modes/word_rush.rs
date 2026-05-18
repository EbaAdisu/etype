use super::{Difficulty, Mode};
use crate::content::words::shuffled_words;
use crate::engine::scorer::{accuracy, cpm, wpm};
use crate::engine::timer::{KeyTimer, SessionTimer};
use crate::modes::ModeResult;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct WordRushState {
    pub words: Vec<String>,
    pub current_idx: usize,
    pub input: String,
    pub difficulty: Difficulty,
    pub timer: Option<SessionTimerSnapshot>,
    pub duration_secs: u64,
    pub correct_chars: u32,
    pub total_keystrokes: u32,
    pub error_flash: bool,
    pub key_stats: HashMap<char, KeyStat>,
    pub started: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SessionTimerSnapshot {
    pub remaining: f64,
    pub fraction: f64,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct KeyStat {
    pub errors: u32,
    pub delays_ms: Vec<f64>,
}

pub struct WordRushSession {
    pub state: WordRushState,
    timer: SessionTimer,
    key_timer: KeyTimer,
}

impl WordRushSession {
    pub fn new(diff: Difficulty, duration_secs: u64) -> Self {
        let words = shuffled_words(&diff);
        let timer = SessionTimer::start(duration_secs);
        WordRushSession {
            state: WordRushState {
                words,
                current_idx: 0,
                input: String::new(),
                difficulty: diff,
                timer: None,
                duration_secs,
                correct_chars: 0,
                total_keystrokes: 0,
                error_flash: false,
                key_stats: HashMap::new(),
                started: true,
            },
            timer,
            key_timer: KeyTimer::new(),
        }
    }

    pub fn tick(&mut self) {
        let remaining = self.timer.remaining_secs();
        let fraction = self.timer.fraction_remaining();
        self.state.timer = Some(SessionTimerSnapshot {
            remaining,
            fraction,
        });
        if self.state.error_flash {
            self.state.error_flash = false;
        }
    }

    pub fn is_done(&self) -> bool {
        self.timer.is_expired()
    }

    pub fn push_char(&mut self, c: char) {
        let delay = self.key_timer.elapsed_ms();
        self.key_timer.reset();
        self.state.total_keystrokes += 1;
        let stat = self.state.key_stats.entry(c).or_default();
        stat.delays_ms.push(delay);
        self.state.input.push(c);
    }

    pub fn backspace(&mut self) {
        self.state.input.pop();
    }

    pub fn ctrl_w(&mut self) {
        self.state.input.clear();
    }

    pub fn confirm_word(&mut self) {
        let target = self
            .state
            .words
            .get(self.state.current_idx)
            .cloned()
            .unwrap_or_default();
        if self.state.input.trim() == target {
            self.state.correct_chars += target.len() as u32 + 1; // +1 for space
            self.state.current_idx += 1;
            // Wrap words if we run out
            if self.state.current_idx >= self.state.words.len() {
                self.state
                    .words
                    .extend(shuffled_words(&self.state.difficulty));
            }
            self.state.input.clear();
        } else {
            // Wrong — record errors for each char
            for c in self.state.input.chars() {
                let stat = self.state.key_stats.entry(c).or_default();
                stat.errors += 1;
            }
            self.state.total_keystrokes += 1;
            self.state.error_flash = true;
            self.state.input.clear();
        }
    }

    pub fn finish(&self, is_new_best: bool, xp_earned: u32) -> ModeResult {
        let elapsed = self.timer.elapsed_secs();
        let w = wpm(self.state.correct_chars, elapsed);
        let c = cpm(self.state.correct_chars, elapsed);
        let a = accuracy(
            self.state.correct_chars,
            self.state.correct_chars
                + (self
                    .state
                    .total_keystrokes
                    .saturating_sub(self.state.correct_chars)),
        );
        let key_stats = self
            .state
            .key_stats
            .iter()
            .map(|(ch, stat)| {
                let avg = if stat.delays_ms.is_empty() {
                    0.0
                } else {
                    stat.delays_ms.iter().sum::<f64>() / stat.delays_ms.len() as f64
                };
                (*ch, (stat.errors, avg))
            })
            .collect();
        ModeResult {
            mode: Mode::WordRush,
            difficulty: self.state.difficulty.clone(),
            wpm: w,
            cpm: c,
            accuracy: a,
            duration_s: self.state.duration_secs,
            is_new_best,
            xp_earned,
            key_stats,
        }
    }
}
