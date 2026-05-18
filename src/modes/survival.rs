use super::{Difficulty, Mode};
use crate::content::words::shuffled_words;
use crate::engine::scorer::{accuracy, cpm, wpm};
use crate::engine::timer::KeyTimer;
use crate::modes::ModeResult;
use rand::Rng;
use std::collections::HashMap;
use std::time::Instant;

const MAX_COLS: u16 = 60;
const MAX_ROWS: u16 = 20;

#[derive(Debug, Clone, PartialEq)]
pub struct FallingWord {
    pub text: String,
    pub row: u16,
    pub col: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SurvivalState {
    pub words: Vec<FallingWord>,
    pub input: String,
    pub lives: u8,
    pub score: u32,
    pub difficulty: Difficulty,
    pub speed_level: u32,
    pub correct_chars: u32,
    pub total_keystrokes: u32,
    pub started: bool,
}

pub struct SurvivalSession {
    pub state: SurvivalState,
    word_pool: Vec<String>,
    pool_idx: usize,
    key_timer: KeyTimer,
    key_stats: HashMap<char, (u32, Vec<f64>)>,
    start: Instant,
    last_tick: Instant,
    tick_interval_ms: u64,
    next_spawn_tick: u32,
    tick_count: u32,
    spawn_interval: u32,
}

impl SurvivalSession {
    pub fn new(diff: Difficulty) -> Self {
        let word_pool = shuffled_words(&diff);
        let mut session = SurvivalSession {
            state: SurvivalState {
                words: Vec::new(),
                input: String::new(),
                lives: 3,
                score: 0,
                difficulty: diff,
                speed_level: 1,
                correct_chars: 0,
                total_keystrokes: 0,
                started: true,
            },
            word_pool,
            pool_idx: 0,
            key_timer: KeyTimer::new(),
            key_stats: HashMap::new(),
            start: Instant::now(),
            last_tick: Instant::now(),
            tick_interval_ms: 600,
            next_spawn_tick: 0,
            tick_count: 0,
            spawn_interval: 4,
        };
        // Spawn 3 initial words
        for _ in 0..3 {
            session.spawn_word();
        }
        session
    }

    fn next_word(&mut self) -> String {
        if self.pool_idx >= self.word_pool.len() {
            let mut words = shuffled_words(&self.state.difficulty);
            self.word_pool.append(&mut words);
        }
        let w = self.word_pool[self.pool_idx].clone();
        self.pool_idx += 1;
        w
    }

    fn spawn_word(&mut self) {
        let text = self.next_word();
        let max_col = MAX_COLS.saturating_sub(text.len() as u16 + 1);
        let col = rand::thread_rng().gen_range(0..=max_col);
        self.state.words.push(FallingWord { text, row: 0, col });
    }

    pub fn should_tick(&self) -> bool {
        self.last_tick.elapsed().as_millis() as u64 >= self.tick_interval_ms
    }

    pub fn tick(&mut self) {
        self.last_tick = Instant::now();
        self.tick_count += 1;

        // Move words down
        let mut lost_words = 0u8;
        self.state.words.retain_mut(|w| {
            w.row += 1;
            if w.row > MAX_ROWS {
                lost_words += 1;
                false
            } else {
                true
            }
        });
        if lost_words > 0 {
            self.state.lives = self.state.lives.saturating_sub(lost_words);
        }

        // Spawn
        if self.tick_count >= self.next_spawn_tick {
            self.spawn_word();
            self.next_spawn_tick = self.tick_count + self.spawn_interval;
        }

        // Speed increase every 30 seconds
        let elapsed = self.start.elapsed().as_secs();
        let new_speed = 1 + (elapsed / 30) as u32;
        if new_speed > self.state.speed_level {
            self.state.speed_level = new_speed;
            // Decrease tick interval (faster fall)
            self.tick_interval_ms = (600u64)
                .saturating_sub((new_speed - 1) as u64 * 80)
                .max(150);
            // Increase spawn rate
            self.spawn_interval = (4u32).saturating_sub(new_speed.saturating_sub(1)).max(1);
        }
    }

    pub fn is_done(&self) -> bool {
        self.state.lives == 0
    }

    pub fn push_char(&mut self, c: char) {
        let delay = self.key_timer.elapsed_ms();
        self.key_timer.reset();
        self.state.total_keystrokes += 1;
        let entry = self.key_stats.entry(c).or_insert((0, Vec::new()));
        entry.1.push(delay);
        self.state.input.push(c);
    }

    pub fn backspace(&mut self) {
        self.state.input.pop();
    }

    pub fn ctrl_w(&mut self) {
        self.state.input.clear();
    }

    pub fn try_destroy(&mut self) {
        let typed = self.state.input.trim().to_string();
        if typed.is_empty() {
            return;
        }
        if let Some(idx) = self.state.words.iter().position(|w| w.text == typed) {
            let word = self.state.words.remove(idx);
            self.state.correct_chars += word.text.len() as u32;
            self.state.score += 1;
            self.state.input.clear();
        } else {
            // Record errors for mismatched chars
            for c in typed.chars() {
                let e = self.key_stats.entry(c).or_insert((0, Vec::new()));
                e.0 += 1;
            }
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
            mode: Mode::Survival,
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
