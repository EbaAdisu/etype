use crate::db;
use crate::engine::xp::level_from_xp;
use crate::modes::code_snippet::CodeState;
use crate::modes::sentence::SentenceState;
use crate::modes::survival::SurvivalState;
use crate::modes::word_rush::WordRushState;
use crate::modes::{Difficulty, Mode, ModeResult};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TimerChoice {
    Thirty,
    Sixty,
    OneTwenty,
}

impl TimerChoice {
    pub fn seconds(&self) -> u64 {
        match self {
            TimerChoice::Thirty => 30,
            TimerChoice::Sixty => 60,
            TimerChoice::OneTwenty => 120,
        }
    }

    pub fn from_idx(i: usize) -> Self {
        match i {
            0 => TimerChoice::Thirty,
            1 => TimerChoice::Sixty,
            _ => TimerChoice::OneTwenty,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Menu,
    DifficultySelect {
        mode: Mode,
        selected_diff: usize,
        selected_timer: usize,
    },
    WordRush(WordRushState),
    Sentence(SentenceState),
    Code(CodeState),
    Survival(SurvivalState),
    Results(ModeResult),
    Stats,
    Heatmap,
    Help,
}

pub struct Profile {
    pub total_xp: u64,
    pub streak_days: u32,
    pub last_played: Option<String>,
}

pub struct App {
    pub screen: Screen,
    pub profile: Profile,
    pub running: bool,
    pub level_up_anim: Option<u32>,
    // cached DB data for stats/heatmap screens
    pub cached_pbs: HashMap<(String, String), f64>,
    pub cached_sessions: Vec<db::SessionRow>,
    pub cached_heatmap: HashMap<char, (i64, f64)>,
}

impl App {
    pub fn new(conn: &rusqlite::Connection) -> anyhow::Result<Self> {
        let mut profile = db::load_profile(conn)?;
        db::check_and_update_streak(conn, &mut profile)?;
        Ok(App {
            screen: Screen::Menu,
            profile,
            running: true,
            level_up_anim: None,
            cached_pbs: HashMap::new(),
            cached_sessions: Vec::new(),
            cached_heatmap: HashMap::new(),
        })
    }

    pub fn current_level(&self) -> u32 {
        level_from_xp(self.profile.total_xp)
    }

    pub fn difficulty_unlocked(&self, diff: &Difficulty) -> bool {
        self.current_level() >= diff.unlock_level()
    }

    pub fn refresh_stats(&mut self, conn: &rusqlite::Connection) -> anyhow::Result<()> {
        let pbs = db::load_personal_bests(conn)?;
        self.cached_pbs = pbs
            .into_iter()
            .map(|r| ((r.mode, r.difficulty), r.wpm))
            .collect();
        self.cached_sessions = db::load_recent_sessions(conn)?;
        let heatmap_rows = db::load_key_heatmap_data(conn)?;
        self.cached_heatmap = heatmap_rows
            .into_iter()
            .filter_map(|r| {
                let ch = r.key_char.chars().next()?;
                Some((ch, (r.total_errors, r.avg_delay_ms)))
            })
            .collect();
        Ok(())
    }
}
