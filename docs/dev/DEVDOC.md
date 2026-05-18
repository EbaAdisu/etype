# etype — Developer Reference

## Project Structure

```
src/
  main.rs          — entry point, DB init, terminal setup, event loop, key dispatch
  app.rs           — App struct, Screen enum, Mode, Difficulty, Profile
  engine/
    mod.rs
    timer.rs       — SessionTimer (countdown), KeyTimer (keystroke delay)
    scorer.rs      — wpm(), cpm(), accuracy()
    xp.rs          — calculate_xp(), level_from_xp(), streak_bonus(), xp_for_next_level()
  modes/
    mod.rs         — ModeResult struct (shared return type for all modes)
    word_rush.rs   — WordRushSession + WordRushState
    sentence.rs    — SentenceSession + SentenceState
    code_snippet.rs — CodeSession + CodeState
    survival.rs    — SurvivalSession + SurvivalState
  ui/
    mod.rs         — render() dispatcher
    menu.rs        — render_menu(), render_difficulty_select()
    game.rs        — render_word_rush(), render_sentence(), render_code(), render_survival()
    results.rs     — render_results()
    stats.rs       — render_stats()
    heatmap.rs     — render_heatmap()
    help.rs        — render_help()
  content/
    mod.rs
    words.rs       — load_words(), shuffled_words() — backed by assets/words/*.txt
    sentences.rs   — random_quote() — hardcoded QUOTES slice
    code.rs        — load_snippets(), random_snippet() — backed by assets/code/*.txt
  db/
    mod.rs         — open_db(), schema, all query functions
assets/
  words/easy.txt medium.txt hard.txt insane.txt
  code/python.txt bash.txt rust.txt
```

---

## Architecture

### State flow

```
App.screen: Screen  ←──── all game state lives here as enum variant
     │
     ├── Screen::Menu
     ├── Screen::DifficultySelect { mode, selected_diff, selected_timer }
     ├── Screen::WordRush(WordRushState)      ← snapshot, cloned each tick
     ├── Screen::Sentence(SentenceState)
     ├── Screen::Code(CodeState)
     ├── Screen::Survival(SurvivalState)
     ├── Screen::Results(ModeResult)
     ├── Screen::Stats
     ├── Screen::Heatmap
     └── Screen::Help
```

The **live session objects** (`WordRushSession`, etc.) are held in `run_app` as `Option<T>` locals — they own the timers and key stat accumulators. Each tick, the session's public `state` field is cloned into `app.screen` for rendering.

### Session lifecycle

```
start_game()  →  Session::new()  →  held in run_app Option<T>
     │
     ├── tick_sessions() called every loop iteration
     │     • updates timer snapshot in state
     │     • checks is_done(), calls finish_session() if true
     │
     ├── handle_*_key() called on key events
     │     • mutates session (push_char, confirm_word/line, etc.)
     │     • clones state into app.screen
     │     • checks is_done() for untimed modes (sentence, code)
     │
     └── finish_session()
           • calls session.finish(0, false) to get ModeResult with stats
           • computes XP
           • writes to DB (session, key_stats, personal_bests, profile)
           • patches result.xp_earned and result.is_new_best
           • transitions to Screen::Results
```

**Why call `finish(0, false)` first?** The session owns the computed stats (WPM, CPM, accuracy, key_stats). `finish()` takes `&self` so it doesn't consume the session. We call it once with dummy values to extract stats, compute XP, persist to DB, then patch the returned struct before showing the results screen.

---

## Key Modules

### `engine/timer.rs`

- `SessionTimer` — wraps `Instant`, records duration. `remaining_secs()`, `fraction_remaining()`, `is_expired()`.
- `KeyTimer` — tracks time between keystrokes. Call `elapsed_ms()` before each keystroke, then `reset()`.

### `engine/scorer.rs`

```rust
wpm(correct_chars: u32, elapsed_secs: f64) -> f64
    // (chars / 5) / elapsed * 60

cpm(correct_chars: u32, elapsed_secs: f64) -> f64
    // chars / elapsed * 60

accuracy(correct: u32, total: u32) -> f64
    // (correct / total) * 100, returns 100.0 if total == 0
```

### `engine/xp.rs`

```rust
calculate_xp(wpm, accuracy, mode, diff, streak_days) -> u32
    // wpm * (accuracy/100) * mode_bonus * diff_bonus * streak_bonus

level_from_xp(total_xp: u64) -> u32
    // Thresholds: 0→L1, 500→L2, 1500→L3, 3500→L4, 7000→L5, +4000/level after

streak_bonus(streak_days: u32) -> f64
    // min(1.0 + days * 0.05, 2.0)

xp_for_next_level(total_xp: u64) -> Option<u64>
    // XP remaining until next level threshold
```

### `db/mod.rs`

All DB access is through this module. Key functions:

| Function | Description |
|---|---|
| `open_db()` | Opens `~/.local/share/etype/etype.db`, runs `CREATE TABLE IF NOT EXISTS` |
| `load_profile()` | Reads or inserts the single profile row |
| `check_and_update_streak()` | Compares `last_played` to today/yesterday, updates streak |
| `insert_session()` | Inserts session record, returns `last_insert_rowid()` |
| `insert_key_stats()` | Inserts one row per key from `HashMap<char, (errors, avg_delay)>` |
| `upsert_personal_best()` | INSERT OR REPLACE only when new WPM beats existing; returns `bool` |
| `load_recent_sessions()` | Last 10 sessions ORDER BY played_at DESC |
| `load_personal_bests()` | All rows from personal_bests table |
| `load_key_heatmap_data()` | GROUP BY key_char with SUM(errors) and AVG(delay) |
| `update_profile()` | Updates total_xp, streak_days, last_played=today |

### `content/`

Word lists and code snippets are embedded at compile time via `include_str!`. Changing asset files requires a recompile.

- `words.rs` — `shuffled_words(&diff)` returns a randomized `Vec<String>` for the given difficulty. Word Rush refills from a new shuffle when it exhausts the list.
- `sentences.rs` — `random_quote()` picks from 22 hardcoded quotes.
- `code.rs` — snippets are separated by `\n---\n` in each file. `random_snippet(&lang)` picks one.

---

## Mode Internals

### Word Rush
- Timer-based: `SessionTimer` starts at game launch, `is_expired()` ends the session.
- Space confirms the current word. Correct → advance, wrong → `error_flash=true`, clear input.
- Words are pre-shuffled; when `current_idx >= words.len()`, a new shuffled batch is appended.
- `total_keystrokes` counts every key press including retries. Accuracy is derived from `correct_chars / total`.

### Sentence
- No timer — ends when `current_word >= words.len()`.
- Space advances words. Wrong input clears without advancing.
- Per-character green/red highlighting is rendered in `build_sentence_display()` in `ui/game.rs`.

### Code Snippets
- Enter confirms each line. Lines must match exactly (whitespace included).
- Wrong lines flash red (`error_flash`) and must be retyped.
- `line_done: Vec<bool>` tracks which lines show ✓.

### Survival
- Tick-based: `SurvivalSession::should_tick()` checks if enough time has passed. Ticks happen in `tick_sessions()` independently of key events.
- Words fall one row per tick. Words past `MAX_ROWS` cost a life and are removed.
- Speed and spawn rate increase every 30 elapsed seconds.
- Input box matches any visible word on Space. Partial matches highlight prefix in yellow.
- Game ends when `lives == 0`.

---

## Rendering

All render functions are pure: `fn render_X(f: &mut Frame, ...)`. They read state, never write it.

- `ui/mod.rs::render()` dispatches to the right screen based on `app.screen`.
- `ui/game.rs` handles all four live game screens.
- `ui/stats.rs` and `ui/heatmap.rs` read from `app.cached_pbs`, `app.cached_sessions`, `app.cached_heatmap` — populated by `app.refresh_stats()` when navigating to those screens.

---

## Adding a New Mode

1. Add a variant to `Mode` in `app.rs` with `label()` and `db_str()`.
2. Add a `Screen::NewMode(NewModeState)` variant.
3. Create `modes/new_mode.rs` with `NewModeState`, `NewModeSession`, and a `finish()` method returning `ModeResult`.
4. Create `ui/new_mode.rs` with a `render_new_mode(f, state)` function.
5. Wire up rendering in `ui/mod.rs`.
6. Wire up key handling in `main.rs`: add session Option, tick it, handle keys, call `finish_session`.

---

## Database Schema

See `docs/architecture/database.md` for the full schema. Tables:

| Table | Purpose |
|---|---|
| `sessions` | One row per completed session |
| `key_stats` | Per-key error count and avg delay, foreign key to sessions |
| `personal_bests` | Best WPM per (mode, difficulty) — 16 slots max |
| `profile` | Single row: total_xp, streak_days, last_played |

Level is always derived at runtime via `level_from_xp(total_xp)` — never stored.

---

## Build Commands

```bash
cargo build          # debug build
cargo run            # run (dev)
cargo test           # 7 unit tests in engine/scorer.rs and engine/xp.rs
cargo clippy -- -D warnings   # lint (run before commit)
cargo fmt            # format (run before commit)
cargo build --release         # release binary → target/release/etype
```

DB is created automatically at `~/.local/share/etype/etype.db` on first run.
