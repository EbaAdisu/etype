# Contributing to etype

Thanks for your interest in contributing. This document covers everything you need to get started.

## Setup

```bash
git clone <repo-url>
cd etype
cargo build
```

No system dependencies required — SQLite is bundled via `rusqlite`'s `bundled` feature.

## Before submitting a PR

Run these and make sure they all pass:

```bash
cargo fmt          # format code
cargo clippy -- -D warnings   # lint — no warnings allowed
cargo test         # all unit tests must pass
```

## Project layout

```
src/
  main.rs           event loop and key dispatch
  app.rs            App state, Screen enum, Mode, Difficulty
  engine/           pure logic — timer, scorer, XP formula
  modes/            one file per game mode, session structs
  ui/               one file per screen, pure render functions
  content/          word lists and code snippet loaders
  db/               all SQLite access
assets/
  words/            easy / medium / hard / insane word lists
  code/             python / bash / rust code snippets
docs/               architecture and design docs
```

Full architecture details are in [`docs/dev/DEVDOC.md`](docs/dev/DEVDOC.md).

## Architecture rules

- **State lives in `App`** — never scatter state across modules.
- **Render functions are pure** — `fn render_X(f: &mut Frame, app: &App)` reads state, never writes it.
- **All DB access through `db/mod.rs`** — no raw SQL elsewhere.
- **No async** — everything is synchronous terminal I/O.
- **No `println!` inside the TUI loop** — it breaks the terminal. Use `tracing` if you need debug output.

## Adding content

**New words:** add them to the appropriate `assets/words/*.txt` file (one word per line).

**New code snippets:** append to `assets/code/{python,bash,rust}.txt`, separated by a line containing only `---`.

**New quotes:** add to the `QUOTES` slice in `src/content/sentences.rs`.

## Adding a new game mode

See the "Adding a New Mode" section in [`docs/dev/DEVDOC.md`](docs/dev/DEVDOC.md).

## Code style

- No comments unless the *why* is non-obvious.
- Prefer `match` over `if let` chains for 3+ variants.
- Keep render functions under 60 lines — split into helpers if longer.
- New mode state structs follow the `WordRushState`, `SentenceState` naming convention.

## Opening issues

- Bug reports: include your OS, terminal emulator, and steps to reproduce.
- Feature requests: explain the use case, not just the feature.
- Check existing issues before opening a new one.
