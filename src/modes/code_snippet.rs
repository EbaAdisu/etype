use super::{Difficulty, Mode};
use crate::content::code::{random_snippet, CodeLang};
use crate::engine::scorer::{accuracy, cpm, wpm};
use crate::engine::timer::KeyTimer;
use crate::modes::ModeResult;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub struct CodeState {
    pub lines: Vec<String>,
    pub current_line: usize,
    pub input: String,
    pub difficulty: Difficulty,
    pub lang: CodeLang,
    pub correct_chars: u32,
    pub total_keystrokes: u32,
    pub error_flash: bool,
    pub line_done: Vec<bool>,
    pub started: bool,
}

pub struct CodeSession {
    pub state: CodeState,
    key_timer: KeyTimer,
    key_stats: HashMap<char, (u32, Vec<f64>)>,
    start: Instant,
}

impl CodeSession {
    pub fn new(diff: Difficulty, lang: CodeLang) -> Self {
        let lines = random_snippet(&lang);
        let line_count = lines.len();
        CodeSession {
            state: CodeState {
                lines,
                current_line: 0,
                input: String::new(),
                difficulty: diff,
                lang,
                correct_chars: 0,
                total_keystrokes: 0,
                error_flash: false,
                line_done: vec![false; line_count],
                started: true,
            },
            key_timer: KeyTimer::new(),
            key_stats: HashMap::new(),
            start: Instant::now(),
        }
    }

    pub fn is_done(&self) -> bool {
        self.state.current_line >= self.state.lines.len()
    }

    pub fn tick(&mut self) {
        if self.state.error_flash {
            self.state.error_flash = false;
        }
    }

    pub fn push_char(&mut self, c: char) {
        let delay = self.key_timer.elapsed_ms();
        self.key_timer.reset();
        self.state.total_keystrokes += 1;
        let entry = self.key_stats.entry(c).or_insert((0, Vec::new()));
        entry.1.push(delay);

        let line = &self.state.lines[self.state.current_line];
        let pos = self.state.input.len();
        if pos < line.len() {
            let expected = line.chars().nth(pos).unwrap_or('\0');
            if c != expected {
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

    pub fn confirm_line(&mut self) {
        let target = &self.state.lines[self.state.current_line];
        if &self.state.input == target {
            self.state.correct_chars += target.len() as u32;
            self.state.line_done[self.state.current_line] = true;
            self.state.current_line += 1;
            self.state.input.clear();
        } else {
            self.state.error_flash = true;
            self.state.total_keystrokes += 1;
            self.state.input.clear();
        }
    }

    pub fn elapsed_secs(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }

    pub fn finish(&self, is_new_best: bool, xp_earned: u32) -> ModeResult {
        let elapsed = self.elapsed_secs();
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
            mode: Mode::Code,
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
