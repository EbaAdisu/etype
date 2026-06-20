# Packaging — Execution Tracker (resume across sessions)

> **Read me first if resuming.** This file is the living state of the packaging work.
> Full design/config detail lives in [`packaging.md`](./packaging.md). This file tracks
> *what is done, what is next, and which branch we are on*.
>
> Last updated: 2026-06-20

---

## 🎯 Goal
Ship etype as installable packages so users run `etype` after a one-line install instead
of `git clone && cargo run`. Two channels chosen: **`.deb`** then **Snap**.

## 🌿 Branch workflow (agreed)
1. **`feat/packaging-deb`** — do the `.deb` work here. ← *(current branch)*
2. Build + **test the `.deb`**, then **merge to `main`**.
3. **`feat/packaging-snap`** — branch off updated `main`, do the Snap work.
4. Build + **test the snap**, then **merge to `main`**.

Reasoning: keep the two packagers isolated so each can be reviewed/tested on its own.

---

## ▶️ RESUME HERE (current status)

- **Phase 1 (.deb): DONE** — tested OK by user, merged to `main`.
- **Phase 2 (Snap): IN PROGRESS** on branch `feat/packaging-snap`.
- **Next action:** see the first unchecked box in Phase 2 below.

---

## Phase 1 — `.deb` (branch `feat/packaging-deb`)

- [x] Add `[package]` metadata to `Cargo.toml`: `authors`, `repository`, `homepage`,
      `rust-version`. (`readme`/`keywords`/`categories` already present.)
- [x] Add `[package.metadata.deb]` section (see `packaging.md` §2.2).
- [x] `cargo install cargo-deb` (installed v3.7.0).
- [x] `cargo deb` → produced `target/debian/etype_0.1.0-1_amd64.deb`.
- [x] Inspect package: contents OK (binary at `usr/bin/etype`), deps = `libc6` only.
- [x] Commit on `feat/packaging-deb`.
- [x] **USER TEST:** install + run `etype` — passed.
- [x] Merge `feat/packaging-deb` → `main`, push.

**Acceptance:** `sudo apt install ./etype_*.deb && etype` launches the TUI; `which etype`
→ `/usr/bin/etype`; `sudo apt remove etype` cleans up.

## Phase 2 — Snap (branch `feat/packaging-snap`, off updated `main`)

- [ ] `git checkout main && git pull && git checkout -b feat/packaging-snap`.
- [ ] Add `snap/snapcraft.yaml` (strict confinement, `core24`, rust plugin — see
      `packaging.md` §3.3). No code change needed (DB path uses `$HOME`).
- [ ] Ensure run command stays `etype` (snap name = app name, or add store alias).
- [ ] `snapcraft` build (needs `snapcraft` + LXD/Multipass — likely on USER machine).
- [ ] `sudo snap install ./etype_*.snap --dangerous` local test.
- [ ] Verify DB writes to `~/snap/etype/current/.local/share/etype/etype.db`.
- [ ] `snapcraft register etype` (confirm name free; fallback `etype-trainer`/`etype-cli`).
- [ ] Commit on `feat/packaging-snap`.
- [ ] **USER TEST:** install snap, run `etype`, confirm auto-update channel works.
- [ ] Merge `feat/packaging-snap` → `main`, push; `snapcraft upload --release=stable`.

**Acceptance:** `sudo snap install etype && etype` launches; data persists under
`~/snap/etype/...`; `sudo snap remove etype` cleans up.

---

## 🔑 Open decisions (defaults chosen unless you say otherwise)
| Decision | Default |
|---|---|
| Architectures at launch | **amd64 only** (arm64 later) |
| `.deb` hosting | **GitHub Releases** (no APT repo yet) |
| CI automation | **Manual first** (GitHub Actions later) |
| Snap name | `etype`; fallback `etype-trainer` / `etype-cli` |
| Run command | must stay **`etype`** on both |
| `--version` flag | not adding unless requested |

## 🧰 Quick command reference
```bash
# DEB
cargo install cargo-deb
cargo deb                                  # build .deb
dpkg -c target/debian/etype_*.deb          # list contents
sudo apt install ./target/debian/etype_*.deb
etype ; sudo apt remove etype

# SNAP (mostly on user machine)
sudo snap install snapcraft --classic
snapcraft                                  # build .snap
sudo snap install ./etype_*.snap --dangerous
etype ; sudo snap remove etype
```

## 🗒️ Session log (append one line per working session)
- 2026-06-20: Created tracker + `feat/packaging-deb` branch; started Phase 1.
