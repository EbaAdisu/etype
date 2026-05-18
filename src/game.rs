use crate::app::{App, Screen, TimerChoice};
use crate::content::code::CodeLang;
use crate::db;
use crate::engine::xp::{calculate_xp, level_from_xp};
use crate::modes::{
    code_snippet::CodeSession, sentence::SentenceSession, survival::SurvivalSession,
    word_rush::WordRushSession,
};
use crate::modes::{Difficulty, Mode};
use crate::ui;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

pub fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    conn: &rusqlite::Connection,
) -> anyhow::Result<()> {
    let mut word_rush_session: Option<WordRushSession> = None;
    let mut sentence_session: Option<SentenceSession> = None;
    let mut code_session: Option<CodeSession> = None;
    let mut survival_session: Option<SurvivalSession> = None;

    while app.running {
        tick_sessions(
            app,
            conn,
            &mut word_rush_session,
            &mut sentence_session,
            &mut code_session,
            &mut survival_session,
        )?;

        terminal.draw(|f| ui::render(f, app))?;

        if !event::poll(Duration::from_millis(50))? {
            continue;
        }

        if let Event::Key(key) = event::read()? {
            handle_key(
                app,
                conn,
                key.code,
                key.modifiers,
                &mut word_rush_session,
                &mut sentence_session,
                &mut code_session,
                &mut survival_session,
            )?;
        }
    }
    Ok(())
}

fn tick_sessions(
    app: &mut App,
    conn: &rusqlite::Connection,
    wr: &mut Option<WordRushSession>,
    _ss: &mut Option<SentenceSession>,
    cs: &mut Option<CodeSession>,
    sv: &mut Option<SurvivalSession>,
) -> anyhow::Result<()> {
    let wr_done = if let Some(session) = wr.as_mut() {
        session.tick();
        app.screen = Screen::WordRush(session.state.clone());
        session.is_done()
    } else {
        false
    };
    if wr_done {
        let session = wr.take().unwrap();
        let result = finish_session(app, conn, |xp, nb| session.finish(nb, xp))?;
        app.screen = Screen::Results(result);
    }

    if let Some(session) = cs.as_mut() {
        session.tick();
    }

    let sv_done = if let Some(session) = sv.as_mut() {
        if session.should_tick() {
            session.tick();
            app.screen = Screen::Survival(session.state.clone());
        }
        session.is_done()
    } else {
        false
    };
    if sv_done {
        let session = sv.take().unwrap();
        let result = finish_session(app, conn, |xp, nb| session.finish(nb, xp))?;
        app.screen = Screen::Results(result);
    }
    Ok(())
}

fn finish_session<F>(
    app: &mut App,
    conn: &rusqlite::Connection,
    make_result: F,
) -> anyhow::Result<crate::modes::ModeResult>
where
    F: FnOnce(u32, bool) -> crate::modes::ModeResult,
{
    let mut result = make_result(0, false);

    let xp = calculate_xp(
        result.wpm,
        result.accuracy,
        &result.mode,
        &result.difficulty,
        app.profile.streak_days,
    );

    let prev_level = app.current_level();
    app.profile.total_xp += xp as u64;

    let rec = db::SessionRecord {
        mode: result.mode.db_str().to_string(),
        difficulty: result.difficulty.db_str().to_string(),
        wpm: result.wpm,
        cpm: result.cpm,
        accuracy: result.accuracy,
        xp_earned: xp,
        duration_s: result.duration_s,
    };
    let session_id = db::insert_session(conn, &rec)?;
    db::insert_key_stats(conn, session_id, &result.key_stats)?;

    let is_new_best = db::upsert_personal_best(
        conn,
        result.mode.db_str(),
        result.difficulty.db_str(),
        result.wpm,
        result.accuracy,
        session_id,
    )?;

    db::check_and_update_streak(conn, &mut app.profile)?;
    db::update_profile(conn, app.profile.total_xp, app.profile.streak_days)?;

    if level_from_xp(app.profile.total_xp) > prev_level {
        app.level_up_anim = Some(level_from_xp(app.profile.total_xp));
    }

    result.xp_earned = xp;
    result.is_new_best = is_new_best;
    Ok(result)
}

#[allow(clippy::too_many_arguments)]
fn handle_key(
    app: &mut App,
    conn: &rusqlite::Connection,
    key: KeyCode,
    modifiers: KeyModifiers,
    wr: &mut Option<WordRushSession>,
    ss: &mut Option<SentenceSession>,
    cs: &mut Option<CodeSession>,
    sv: &mut Option<SurvivalSession>,
) -> anyhow::Result<()> {
    if key == KeyCode::Char('c') && modifiers.contains(KeyModifiers::CONTROL) {
        app.running = false;
        return Ok(());
    }

    match &app.screen.clone() {
        Screen::Menu => handle_menu(app, conn, key)?,
        Screen::DifficultySelect {
            mode,
            selected_diff,
            selected_timer,
        } => {
            handle_difficulty_select(
                app,
                key,
                mode.clone(),
                *selected_diff,
                *selected_timer,
                wr,
                ss,
                cs,
                sv,
            )?;
        }
        Screen::WordRush(_) => handle_word_rush_key(app, key, modifiers, wr)?,
        Screen::Sentence(_) => handle_sentence_key(app, conn, key, modifiers, ss)?,
        Screen::Code(_) => handle_code_key(app, conn, key, modifiers, cs)?,
        Screen::Survival(_) => handle_survival_key(app, conn, key, modifiers, sv)?,
        Screen::Results(result) => {
            let result = result.clone();
            match key {
                KeyCode::Char('r') | KeyCode::Char('R') => {
                    app.level_up_anim = None;
                    start_game(
                        app,
                        &result.mode,
                        &result.difficulty,
                        result.duration_s,
                        wr,
                        ss,
                        cs,
                        sv,
                    );
                }
                KeyCode::Char('m') | KeyCode::Char('M') | KeyCode::Esc => {
                    app.level_up_anim = None;
                    app.screen = Screen::Menu;
                }
                _ => {}
            }
        }
        Screen::Stats | Screen::Heatmap => {
            if matches!(key, KeyCode::Esc | KeyCode::Char('q')) {
                app.screen = Screen::Menu;
            }
        }
        Screen::Help => {
            if matches!(key, KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('q')) {
                app.screen = Screen::Menu;
            }
        }
    }
    Ok(())
}

fn handle_menu(app: &mut App, conn: &rusqlite::Connection, key: KeyCode) -> anyhow::Result<()> {
    match key {
        KeyCode::Char('q') | KeyCode::Char('Q') => app.running = false,
        KeyCode::Char('1') => {
            app.screen = Screen::DifficultySelect {
                mode: Mode::WordRush,
                selected_diff: 0,
                selected_timer: 1,
            };
        }
        KeyCode::Char('2') => {
            app.screen = Screen::DifficultySelect {
                mode: Mode::Sentence,
                selected_diff: 0,
                selected_timer: 1,
            };
        }
        KeyCode::Char('3') => {
            app.screen = Screen::DifficultySelect {
                mode: Mode::Code,
                selected_diff: 0,
                selected_timer: 1,
            };
        }
        KeyCode::Char('4') => {
            app.screen = Screen::DifficultySelect {
                mode: Mode::Survival,
                selected_diff: 0,
                selected_timer: 1,
            };
        }
        KeyCode::Char('s') | KeyCode::Char('S') => {
            app.refresh_stats(conn)?;
            app.screen = Screen::Stats;
        }
        KeyCode::Char('h') | KeyCode::Char('H') => {
            app.refresh_stats(conn)?;
            app.screen = Screen::Heatmap;
        }
        KeyCode::Char('?') => app.screen = Screen::Help,
        _ => {}
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn handle_difficulty_select(
    app: &mut App,
    key: KeyCode,
    mode: Mode,
    selected_diff: usize,
    selected_timer: usize,
    wr: &mut Option<WordRushSession>,
    ss: &mut Option<SentenceSession>,
    cs: &mut Option<CodeSession>,
    sv: &mut Option<SurvivalSession>,
) -> anyhow::Result<()> {
    match key {
        KeyCode::Esc => app.screen = Screen::Menu,
        KeyCode::Char('1') => set_diff(app, &mode, 0, selected_timer),
        KeyCode::Char('2') => set_diff(app, &mode, 1, selected_timer),
        KeyCode::Char('3') => set_diff(app, &mode, 2, selected_timer),
        KeyCode::Char('4') => set_diff(app, &mode, 3, selected_timer),
        KeyCode::Left => {
            if mode == Mode::WordRush {
                set_diff(app, &mode, selected_diff, selected_timer.saturating_sub(1));
            }
        }
        KeyCode::Right => {
            if mode == Mode::WordRush {
                set_diff(app, &mode, selected_diff, (selected_timer + 1).min(2));
            }
        }
        KeyCode::Enter => {
            let diff = idx_to_difficulty(selected_diff);
            if !app.difficulty_unlocked(&diff) {
                return Ok(());
            }
            let duration = TimerChoice::from_idx(selected_timer).seconds();
            start_game(app, &mode, &diff, duration, wr, ss, cs, sv);
        }
        _ => {}
    }
    Ok(())
}

fn set_diff(app: &mut App, mode: &Mode, diff: usize, timer: usize) {
    app.screen = Screen::DifficultySelect {
        mode: mode.clone(),
        selected_diff: diff,
        selected_timer: timer,
    };
}

fn idx_to_difficulty(idx: usize) -> Difficulty {
    match idx {
        0 => Difficulty::Easy,
        1 => Difficulty::Medium,
        2 => Difficulty::Hard,
        _ => Difficulty::Insane,
    }
}

#[allow(clippy::too_many_arguments)]
fn start_game(
    app: &mut App,
    mode: &Mode,
    diff: &Difficulty,
    duration: u64,
    wr: &mut Option<WordRushSession>,
    ss: &mut Option<SentenceSession>,
    cs: &mut Option<CodeSession>,
    sv: &mut Option<SurvivalSession>,
) {
    match mode {
        Mode::WordRush => {
            let session = WordRushSession::new(diff.clone(), duration);
            app.screen = Screen::WordRush(session.state.clone());
            *wr = Some(session);
        }
        Mode::Sentence => {
            let session = SentenceSession::new(diff.clone());
            app.screen = Screen::Sentence(session.state.clone());
            *ss = Some(session);
        }
        Mode::Code => {
            let session = CodeSession::new(diff.clone(), CodeLang::random());
            app.screen = Screen::Code(session.state.clone());
            *cs = Some(session);
        }
        Mode::Survival => {
            let session = SurvivalSession::new(diff.clone());
            app.screen = Screen::Survival(session.state.clone());
            *sv = Some(session);
        }
    }
}

fn handle_word_rush_key(
    app: &mut App,
    key: KeyCode,
    mods: KeyModifiers,
    wr: &mut Option<WordRushSession>,
) -> anyhow::Result<()> {
    if key == KeyCode::Esc {
        app.screen = Screen::Menu;
        *wr = None;
        return Ok(());
    }
    if let Some(session) = wr.as_mut() {
        match key {
            KeyCode::Char('w') if mods.contains(KeyModifiers::CONTROL) => session.ctrl_w(),
            KeyCode::Backspace => session.backspace(),
            KeyCode::Char(' ') => session.confirm_word(),
            KeyCode::Char(c) => session.push_char(c),
            _ => {}
        }
        app.screen = Screen::WordRush(session.state.clone());
    }
    Ok(())
}

fn handle_sentence_key(
    app: &mut App,
    conn: &rusqlite::Connection,
    key: KeyCode,
    mods: KeyModifiers,
    ss: &mut Option<SentenceSession>,
) -> anyhow::Result<()> {
    if key == KeyCode::Esc {
        app.screen = Screen::Menu;
        *ss = None;
        return Ok(());
    }
    let done = if let Some(session) = ss.as_mut() {
        match key {
            KeyCode::Char('w') if mods.contains(KeyModifiers::CONTROL) => session.ctrl_w(),
            KeyCode::Backspace => session.backspace(),
            KeyCode::Char(' ') => session.confirm_word(),
            KeyCode::Char(c) => session.push_char(c),
            _ => {}
        }
        app.screen = Screen::Sentence(session.state.clone());
        session.is_done()
    } else {
        false
    };
    if done {
        let session = ss.take().unwrap();
        let result = finish_session(app, conn, |xp, nb| session.finish(nb, xp))?;
        app.screen = Screen::Results(result);
    }
    Ok(())
}

fn handle_code_key(
    app: &mut App,
    conn: &rusqlite::Connection,
    key: KeyCode,
    mods: KeyModifiers,
    cs: &mut Option<CodeSession>,
) -> anyhow::Result<()> {
    if key == KeyCode::Esc {
        app.screen = Screen::Menu;
        *cs = None;
        return Ok(());
    }
    let done = if let Some(session) = cs.as_mut() {
        match key {
            KeyCode::Char('w') if mods.contains(KeyModifiers::CONTROL) => session.ctrl_w(),
            KeyCode::Backspace => session.backspace(),
            KeyCode::Enter => session.confirm_line(),
            KeyCode::Char(c) => session.push_char(c),
            _ => {}
        }
        app.screen = Screen::Code(session.state.clone());
        session.is_done()
    } else {
        false
    };
    if done {
        let session = cs.take().unwrap();
        let result = finish_session(app, conn, |xp, nb| session.finish(nb, xp))?;
        app.screen = Screen::Results(result);
    }
    Ok(())
}

fn handle_survival_key(
    app: &mut App,
    conn: &rusqlite::Connection,
    key: KeyCode,
    mods: KeyModifiers,
    sv: &mut Option<SurvivalSession>,
) -> anyhow::Result<()> {
    if key == KeyCode::Esc {
        app.screen = Screen::Menu;
        *sv = None;
        return Ok(());
    }
    let done = if let Some(session) = sv.as_mut() {
        match key {
            KeyCode::Char('w') if mods.contains(KeyModifiers::CONTROL) => session.ctrl_w(),
            KeyCode::Backspace => session.backspace(),
            KeyCode::Char(' ') => session.try_destroy(),
            KeyCode::Char(c) => session.push_char(c),
            _ => {}
        }
        app.screen = Screen::Survival(session.state.clone());
        session.is_done()
    } else {
        false
    };
    if done {
        let session = sv.take().unwrap();
        let result = finish_session(app, conn, |xp, nb| session.finish(nb, xp))?;
        app.screen = Screen::Results(result);
    }
    Ok(())
}
