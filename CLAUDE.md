# etype — Claude Project Config

## What This Is
A fully local, terminal-based typing trainer built in Rust.  
No web, no server, no Electron. Pure terminal TUI.  
All design and architecture docs live in `docs/` — read `docs/README.md` for the index.

## Stack
- **Language**: Rust (stable)
- **TUI**: Ratatui 0.29 + Crossterm 0.28
- **DB**: SQLite via `rusqlite` (bundled feature — no system sqlite needed)
- **Other**: `serde`, `rand`, `chrono`

## Commands

```bash
# Build
cargo build

# Run (development)
cargo run

# Run with logging
RUST_LOG=debug cargo run

# Tests
cargo test

# Lint (run before every commit)
cargo clippy -- -D warnings

# Format (run before every commit)
cargo fmt

# Check only (no binary, faster)
cargo check

# Release build
cargo build --release
```

## Project Structure
```
src/
  main.rs          — entry point, init DB, launch app
  app.rs           — App state struct, main event loop
  ui/              — all ratatui rendering, one file per screen
  modes/           — game logic for each of the 4 modes
  engine/          — timer, scorer (WPM/CPM/accuracy), XP formula
  db/              — all SQLite queries, schema migration
  content/         — word list and code snippet loaders
assets/
  words/           — easy/medium/hard/insane word list .txt files
  code/            — python/bash/rust snippet .txt files
docs/
  PLAN.md          — full design doc, phases checklist, DB schema, UI mockups
```

## Architecture Rules

### State
- All game state lives in `App` (src/app.rs). Never scatter state across modules.
- Current screen is an enum variant on `App`. Navigation = changing that enum.
- Each mode has its own state struct that `App` holds as an `Option<ModeState>`.

### Rendering
- Every screen is a pure function: `fn render_X(f: &mut Frame, app: &App)`.
- No side effects in render functions — only read state, never write it.
- Rendering lives in `src/ui/`, game logic lives in `src/modes/`. Never mix them.

### Timing
- Always use `std::time::Instant` for keystroke timing — never `SystemTime`.
- WPM formula: `(correct_chars / 5.0) / elapsed.as_secs_f64() * 60.0`
- CPM formula: `correct_chars as f64 / elapsed.as_secs_f64() * 60.0`

### Database
- All queries go through `src/db/mod.rs`. No raw SQL anywhere else.
- DB file path: `~/.local/share/etype/etype.db`
- Run schema creation with `CREATE TABLE IF NOT EXISTS` on startup — no migration framework needed at this scale.

### Error Handling
- Use `anyhow::Result` for top-level error propagation in main/db.
- Game logic should use concrete error types or `Option` — not `anyhow`.
- Never `unwrap()` on anything that touches the filesystem or DB.

## Code Style
- No comments unless the WHY is non-obvious (a workaround, a hidden constraint).
- No docstrings on obvious functions.
- Prefer `match` over `if let` chains when handling 3+ variants.
- Keep render functions under 60 lines — split into helpers if longer.
- Name mode state structs `WordRushState`, `SentenceState`, etc.

## XP Formula (implement exactly as designed)
```rust
let accuracy_mul = accuracy / 100.0;
let mode_bonus = match mode { WordRush => 1.0, Sentence => 1.1, Code => 1.3, Survival => 1.2 };
let diff_bonus = match diff { Easy => 0.8, Medium => 1.0, Hard => 1.3, Insane => 1.6 };
let streak_bonus = (1.0 + streak_days as f64 * 0.05).min(2.0);
let xp = (wpm * accuracy_mul * mode_bonus * diff_bonus * streak_bonus) as u32;
```

## DB Schema (implement exactly as designed)
See `docs/PLAN.md` section 6 for the full SQL. Tables: `sessions`, `key_stats`, `personal_bests`, `profile`.

## What NOT To Do
- Do not add async/tokio — everything is synchronous terminal I/O, no async needed.
- Do not use `println!` inside the TUI loop — it breaks the terminal. Use a log file or `tracing` if debug output is needed.
- Do not split a simple function into traits/generics just because it handles 2 modes — wait until there are 3+ actual shared patterns.
- Do not add web, networking, or external API calls. This is 100% local.
- Do not create a new file for something that fits cleanly in an existing one.

## Phase Tracking
Check off phases in `docs/dev/phases.md` as they complete.  
Current phase: **Phase 1 — Cargo setup + basic ratatui event loop**
