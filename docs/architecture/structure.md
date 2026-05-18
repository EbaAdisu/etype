# Project Structure

## Top-Level Layout

```
etype/
├── Cargo.toml
├── Cargo.lock
├── CLAUDE.md                  — Claude project instructions
├── docs/                      — all documentation (you are here)
├── assets/                    — word lists and code snippets
└── src/                       — all Rust source code
```

## Assets

```
assets/
├── words/
│   ├── easy.txt               — common 3-4 letter words, one per line
│   ├── medium.txt             — 5-7 letter words
│   ├── hard.txt               — 8-12 letter words
│   └── insane.txt             — rare, technical, long words
└── code/
    ├── python.txt             — Python snippets separated by ---
    ├── bash.txt               — Bash snippets separated by ---
    └── rust.txt               — Rust snippets separated by ---
```

## Source Tree

```
src/
├── main.rs                    — init DB, set up terminal, start app loop
├── app.rs                     — App struct (global state), Screen enum, event loop
│
├── ui/                        — ratatui rendering only, no game logic here
│   ├── mod.rs
│   ├── menu.rs                — main menu screen
│   ├── game.rs                — shared game HUD (timer bar, WPM display, input box)
│   ├── results.rs             — end-of-session summary + XP gain animation
│   ├── stats.rs               — session history list + personal bests
│   ├── heatmap.rs             — keyboard layout rendered with per-key coloring
│   └── help.rs                — keybinds and how-to-play
│
├── modes/                     — game logic, one file per mode
│   ├── mod.rs                 — shared types: Difficulty enum, ModeResult struct
│   ├── word_rush.rs           — WordRushState, tick(), handle_input()
│   ├── sentence.rs            — SentenceState, tick(), handle_input()
│   ├── code_snippet.rs        — CodeState, tick(), handle_input()
│   └── survival.rs            — SurvivalState, tick(), handle_input()
│
├── engine/                    — pure calculation, no state, no I/O
│   ├── mod.rs
│   ├── timer.rs               — keystroke timing helpers using Instant
│   ├── scorer.rs              — wpm(), cpm(), accuracy() functions
│   └── xp.rs                  — calculate_xp(), level_from_xp(), streak_bonus()
│
├── db/                        — all SQLite interaction
│   └── mod.rs                 — open_db(), schema init, all query functions
│
└── content/                   — loads and serves game content
    ├── words.rs               — loads assets/words/*.txt, returns Vec<String>
    ├── sentences.rs           — hardcoded quotes, returns Vec<String>
    └── code.rs                — loads assets/code/*.txt, splits on ---, returns Vec<String>
```

## Key Design Rules

**App state is centralized.**  
`App` in `src/app.rs` is the single source of truth. Every screen and mode reads from it.  
`App` holds an `Option<ActiveMode>` enum that contains the current mode's state.

**Rendering is pure.**  
All `ui/` functions have the signature `fn render_X(f: &mut Frame, app: &App)`.  
They only read `app` — they never write to it.

**Mode logic is isolated.**  
Each mode file exposes `handle_input(state, key) -> Option<ModeResult>` and `tick(state)`.  
`ModeResult` is returned when a session ends and contains WPM, CPM, accuracy, key stats.

**The engine is stateless.**  
`engine/` functions are pure: given inputs, return a value. No global state, no DB access.

**DB access is centralized.**  
Only `db/mod.rs` talks to SQLite. All other modules call functions from there.
