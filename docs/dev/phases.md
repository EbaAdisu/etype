# Build Phases

> Update this file as phases complete. This is the source of truth for progress.

## Status Key
- `[ ]` Not started
- `[~]` In progress
- `[x]` Complete

---

## Phase 1 — Project Foundation
`[x]` Initialize `cargo new etype`  
`[x]` Add all dependencies to `Cargo.toml`  
`[x]` Basic ratatui event loop (render → input → update)  
`[x]` `App` struct with `Screen` enum  
`[x]` Terminal setup and cleanup (raw mode, alternate screen)  
`[x]` Quit with `q` or `Ctrl+C`  

## Phase 2 — Main Menu
`[x]` Render main menu screen (`ui/menu.rs`)  
`[x]` Keyboard navigation between menu items  
`[x]` Difficulty selector screen  
`[x]` Difficulty lock check (always unlocked at start — lock logic comes in Phase 12)  

## Phase 3 — Core Engine
`[x]` Keystroke timer in `engine/timer.rs` using `std::time::Instant`  
`[x]` `scorer.rs`: `wpm()`, `cpm()`, `accuracy()` functions  
`[x]` `xp.rs`: `calculate_xp()`, `level_from_xp()`, `streak_bonus()`  
`[x]` Unit tests for all engine functions  

## Phase 4 — Word Lists & Content Loader
`[x]` Create `assets/words/easy.txt` (~200 words)  
`[x]` Create `assets/words/medium.txt` (~200 words)  
`[x]` Create `assets/words/hard.txt` (~200 words)  
`[x]` Create `assets/words/insane.txt` (~100 words)  
`[x]` `content/words.rs`: load from assets, return `Vec<String>`  

## Phase 5 — Word Rush Mode
`[x]` `modes/word_rush.rs`: `WordRushState` struct  
`[x]` Input handling, word validation, advance-on-correct  
`[x]` Countdown timer integration  
`[x]` `ui/game.rs`: render word rush HUD (timer bar, WPM, input box, word line)  
`[x]` Returns `ModeResult` on session end  

## Phase 6 — Sentence Mode
`[x]` Hardcoded quotes in `content/sentences.rs` (20+ quotes)  
`[x]` `modes/sentence.rs`: `SentenceState`, word-by-word validation  
`[x]` Inline error highlighting (green/red chars)  
`[x]` Render sentence mode UI  
`[x]` Returns `ModeResult` on completion  

## Phase 7 — Code Snippets Mode
`[x]` Create `assets/code/python.txt`, `bash.txt`, `rust.txt` (5+ snippets each)  
`[x]` `content/code.rs`: load and split snippets on `---`  
`[x]` `modes/code_snippet.rs`: line-by-line validation including whitespace  
`[x]` Render code mode UI with syntax colors and line checkmarks  
`[x]` Returns `ModeResult` on completion  

## Phase 8 — Survival Mode
`[x]` `modes/survival.rs`: `SurvivalState` with falling word positions  
`[x]` Tick function: move words down, spawn new ones, check bottom collision  
`[x]` Speed increase every 30 seconds  
`[x]` Life loss on word reaching bottom  
`[x]` Render survival UI: scattered word positions + lives + score  
`[x]` Returns `ModeResult` on game over  

## Phase 9 — SQLite Database Layer
`[x]` `db/mod.rs`: `open_db()`, schema creation  
`[x]` `insert_session()` and `insert_key_stats()`  
`[x]` `upsert_personal_best()` with comparison logic  
`[x]` `load_profile()` and `update_profile()`  
`[x]` `load_recent_sessions()` (last 10)  
`[x]` `load_personal_bests()` (all 16 slots)  
`[x]` `load_key_heatmap_data()` (aggregated across all sessions)  

## Phase 10 — Results Screen
`[x]` `ui/results.rs`: session summary layout  
`[x]` Display WPM, CPM, accuracy, XP earned, streak  
`[x]` "NEW BEST!" highlight when PB is beaten  
`[x]` XP progress bar toward next level  
`[x]` [r] play again / [m] menu navigation  

## Phase 11 — Stats Screen
`[x]` `ui/stats.rs`: personal bests grid (4 modes × 4 difficulties)  
`[x]` Recent sessions list (last 10 rows)  

## Phase 12 — Key Heatmap Screen
`[x]` `ui/heatmap.rs`: render QWERTY keyboard layout in terminal  
`[x]` Color each key by error rate + avg delay (green → yellow → red)  
`[x]` "Worst keys" list below the keyboard  

## Phase 13 — XP / Level / Streak System
`[x]` Level-up detection in results screen  
`[x]` Level-up animation  
`[x]` Difficulty lock enforcement based on level  
`[x]` Streak check on app launch: reset if a day was missed  
`[x]` Streak display on main menu  

## Phase 14 — Polish & Help
`[x]` `ui/help.rs`: keybinds reference screen  
`[x]` Color theme: consistent color palette across all screens  
`[x]` Timer bar animation (smooth depletion)  
`[x]` Input cursor blinking or highlight  
`[x]` Final QA: test all modes end-to-end  

---

## Backlog (v2)

- [ ] Local multi-user leaderboard with named profiles
- [ ] JSON export of session history
- [ ] Custom word list import from `.txt`
- [ ] Config file at `~/.config/etype/config.toml`
- [ ] Color theme selection
- [ ] Sound effects (terminal bell patterns)
