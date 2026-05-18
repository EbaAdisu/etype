# Tech Stack

## Language

**Rust (stable)**  
Chosen for: precise keystroke timing, single compiled binary, zero runtime dependencies.

## Dependencies

```toml
[dependencies]
ratatui    = "0.29"
crossterm  = "0.28"
rusqlite   = { version = "0.32", features = ["bundled"] }
serde      = { version = "1", features = ["derive"] }
serde_json = "1"
rand       = "0.8"
chrono     = "0.4"
anyhow     = "1"
```

| Crate       | Purpose                                                  |
|-------------|----------------------------------------------------------|
| ratatui     | TUI framework — layouts, widgets, rendering              |
| crossterm   | Cross-platform terminal I/O backend for ratatui          |
| rusqlite    | SQLite bindings. `bundled` = no system sqlite required   |
| serde       | Serialization — used for config and JSON export          |
| serde_json  | JSON support for serde                                   |
| rand        | Random word/snippet selection                            |
| chrono      | Timestamps and date comparison for streak logic          |
| anyhow      | Error propagation in main and db layer                   |

## Why No Async

Everything in this app is synchronous terminal I/O.  
Adding tokio would add complexity and a large compile-time dependency for zero benefit.  
The terminal event loop is a tight synchronous loop — keep it that way.

## Build Output

`cargo build --release` produces a single binary at `target/release/etype`.  
That binary can be copied to `/usr/local/bin/etype` for system-wide use.

## Minimum Rust Version

Rust 1.75+ (stable). No nightly features used.
