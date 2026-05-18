# etype

A fully local, terminal-based typing trainer written in Rust.

No web, no server, no Electron. Just your terminal.

```
╔══════════════════════════════════════╗
║            e t y p e                ║
║   Level 3  •  streak: 7 days        ║
║   Total XP: 2,341                   ╠══════════════════════════════════════╣
║   [1]  Word Rush                    ║
║   [2]  Sentence Mode                ║
║   [3]  Code Snippets                ║
║   [4]  Survival                     ║
╠══════════════════════════════════════╣
║   [s]  Stats & History              ║
║   [h]  Key Heatmap                  ║
║   [?]  Help     [q]  Quit           ║
╚══════════════════════════════════════╝
```

## Features

**Four game modes**

| Mode | How it works |
|---|---|
| **Word Rush** | Type words as fast as possible before the timer runs out (30 / 60 / 120s) |
| **Sentence** | Type a full quote with high accuracy, no time pressure |
| **Code Snippets** | Type real Python, Bash, or Rust code line-by-line, including indentation |
| **Survival** | Words fall down the screen — type them before they reach the bottom |

**Progression system**
- XP earned each session based on WPM, accuracy, mode, difficulty, and streak bonus
- 4 difficulty tiers (Easy / Medium / Hard / Insane) that unlock as you level up
- Daily streak tracking with up to ×2.0 XP bonus
- Personal bests tracked per mode × difficulty (16 slots)

**Stats screens**
- Personal bests grid
- Last 10 sessions history
- QWERTY key heatmap colored by error rate and keystroke delay

**100% local** — all data stored in SQLite at `~/.local/share/etype/etype.db`. No telemetry, no accounts.

## Install

### From source

Requires Rust stable (`rustup` is the easiest way to install it).

```bash
git clone <repo-url>
cd etype
cargo build --release
```

The binary is at `target/release/etype`. Copy it anywhere on your `$PATH`:

```bash
cp target/release/etype ~/.local/bin/
```

### Requirements

- A terminal that supports 256 colors
- Linux or macOS (Windows terminal support is untested)

No system SQLite needed — it's bundled into the binary.

## Usage

```bash
etype
```

### Controls

| Key | Action |
|---|---|
| `1`–`4` | Select mode or difficulty |
| `Space` | Confirm word (Word Rush, Sentence, Survival) |
| `Enter` | Confirm line (Code Snippets) |
| `Backspace` | Delete last character |
| `Ctrl+W` | Clear entire input |
| `s` | Stats screen |
| `h` | Key heatmap |
| `?` | Help |
| `Esc` | Back / cancel |
| `q` / `Ctrl+C` | Quit |

## Difficulty tiers

| Tier | Unlocks at | XP multiplier |
|---|---|---|
| Easy | Level 1 | ×0.8 |
| Medium | Level 2 | ×1.0 |
| Hard | Level 3 | ×1.3 |
| Insane | Level 4 | ×1.6 |

## XP formula

```
xp = (wpm × accuracy% × mode_bonus × diff_bonus × streak_bonus) as u32

mode_bonus:   Word Rush 1.0 | Sentence 1.1 | Code 1.3 | Survival 1.2
diff_bonus:   Easy 0.8 | Medium 1.0 | Hard 1.3 | Insane 1.6
streak_bonus: min(1.0 + days × 0.05, 2.0)
```

## Stack

- **Language**: Rust (stable)
- **TUI**: [Ratatui](https://ratatui.rs) 0.29 + Crossterm 0.28
- **Database**: SQLite via `rusqlite` (bundled, no system install needed)
- **Other**: `serde`, `rand`, `chrono`, `anyhow`

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT — see [LICENSE](LICENSE).
