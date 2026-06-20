# Packaging & Distribution Plan — `.deb` + Snap

> Status: **PLAN — awaiting review.** No config has been written yet.
> Goal: let people install etype with one command instead of `git clone && cargo run`.
> Target channels (chosen): **Debian/Ubuntu `.deb`** and **Snap**.

---

## 0. Why this is easy for etype

- **Single self-contained binary.** All assets are embedded with `include_str!`, and
  SQLite is statically linked via `rusqlite`'s `bundled` feature. There are **no external
  runtime data files and no system library to install** — the shipped artifact is just
  `etype` + libc.
- **DB path is derived from `$HOME` at runtime** (`$HOME/.local/share/etype/etype.db`,
  see `src/db/mod.rs:26`). This matters for Snap (Section 2) because Snap remaps `$HOME`
  into its sandbox, so the app "just works" under strict confinement with no code change.

These two facts remove the usual hard parts of Linux packaging.

---

## 1. Shared prerequisite — `Cargo.toml` metadata

Both packagers read metadata from `Cargo.toml`. Today we have `name`, `version`,
`description`, `license = "MIT"`. Before packaging we add the fields below.

**Planned changes to `[package]`:**
```toml
authors     = ["EbaAdisu <ebaadisu2@gmail.com>"]
repository  = "https://github.com/EbaAdisu/etype"
homepage    = "https://github.com/EbaAdisu/etype"
readme      = "README.md"
keywords    = ["typing", "tui", "terminal", "trainer", "game"]
categories  = ["command-line-utilities", "games"]
rust-version = "1.74"   # pin the MSRV we actually build against
```

No source-code changes are required for either packager. (The only optional code touch is
adding a `--version`/`--help` flag if we want `dpkg`/reviewers to introspect it; ratatui
apps usually skip this. Flagged as optional in Section 5.)

---

## 2. Part A — `.deb` package (Debian / Ubuntu)

**Tool:** [`cargo-deb`](https://github.com/kornelski/cargo-deb) — builds a `.deb` straight
from `Cargo.toml`, auto-detecting library dependencies with `dpkg-shlibdeps`.

### 2.1 Setup (one time)
```bash
cargo install cargo-deb
```

### 2.2 Metadata — new `[package.metadata.deb]` section in `Cargo.toml`
```toml
[package.metadata.deb]
maintainer = "EbaAdisu <ebaadisu2@gmail.com>"
copyright  = "2026, EbaAdisu <ebaadisu2@gmail.com>"
license-file = ["LICENSE", "0"]
extended-description = """\
etype is a fully local, terminal-based typing trainer with four game modes \
(Word Rush, Sentence, Code Snippets, Survival), an XP/level/streak system, \
and a per-key heatmap. No network, no accounts — all data stays in SQLite \
under your home directory."""
section  = "games"
priority = "optional"
assets = [
  ["target/release/etype", "usr/bin/", "755"],
  ["README.md", "usr/share/doc/etype/README.md", "644"],
]
# depends is auto-filled by cargo-deb (essentially just libc6) — leave default.
```
Because SQLite is statically bundled, the dependency list collapses to libc — no
`libsqlite3` runtime dep. Good.

### 2.3 Build
```bash
cargo deb            # builds release + emits target/debian/etype_0.1.0-1_amd64.deb
```

### 2.4 Install / uninstall (end users)
```bash
sudo apt install ./etype_0.1.0-1_amd64.deb   # resolves deps, installs to /usr/bin/etype
etype                                          # run
sudo apt remove etype                          # uninstall
```

### 2.5 Multiple architectures
- `amd64` builds natively.
- For `arm64` (Raspberry Pi, Apple-Silicon Linux VMs): add the Rust target and
  cross-build, e.g. `cargo deb --target aarch64-unknown-linux-gnu` (needs a cross
  toolchain or build on an arm64 runner). Decision point in Section 6.

### 2.6 Distribution of the `.deb`
- **Phase 1 (simple):** attach the `.deb` files to **GitHub Releases**. Users download
  and `apt install ./file.deb`. Zero hosting infrastructure.
- **Phase 2 (optional, nicer):** host a proper APT repo so users can
  `apt update && apt install etype` after adding the repo. Options: an `aptly`/`reprepro`
  repo served via GitHub Pages, or a hosted service. More maintenance — deferred.

---

## 3. Part B — Snap package (Ubuntu / cross-distro, auto-updating)

**Tool:** [`snapcraft`](https://snapcraft.io/docs). Builds in an isolated LXD/Multipass VM
and publishes to the Snap Store (free).

### 3.1 Confinement decision — **strict** ✅
Snap strict confinement remaps `$HOME` to `$SNAP_USER_DATA`
(`~/snap/etype/current/`). Since etype computes its DB path from `$HOME` at runtime, under
a strict snap it will write to
`~/snap/etype/current/.local/share/etype/etype.db` automatically.

- **No code change needed.**
- **No `personal-files`/`home`-interface grant needed** (those require manual store
  review and are only needed to reach the *real* `~/.local/share`).
- **Trade-off to note:** snap-installed stats live in the snap's private dir, separate
  from a `.deb`/`cargo` install's `~/.local/share/etype`. Acceptable; documented for users.

This makes strict confinement clearly preferable to `classic` (which would require a manual
Snap Store reviewer approval). We go **strict**.

### 3.2 Setup (one time)
```bash
sudo snap install snapcraft --classic
sudo snap install lxd && sudo lxd init --auto   # build backend
snapcraft register etype                        # claim the name on the Snap Store
```
> ⚠️ Name availability: `etype` must be free/registerable on the Snap Store. Verify during
> `snapcraft register`. Fallback names if taken: `etype-trainer`, `etype-cli`.

### 3.3 New file — `snap/snapcraft.yaml`
```yaml
name: etype
base: core24
version: '0.1.0'        # or: version-script / adopt-info from Cargo.toml
summary: Local terminal typing trainer
description: |
  etype is a fully local, terminal-based typing trainer with four game modes
  (Word Rush, Sentence, Code Snippets, Survival), an XP/level/streak system,
  and a per-key heatmap. No network, no accounts.
grade: stable
confinement: strict

parts:
  etype:
    plugin: rust
    source: .
    build-packages: [gcc, libc6-dev]   # for the bundled-sqlite C compile

apps:
  etype:
    command: bin/etype
    # No plugs required: the app only writes to its own $SNAP_USER_DATA.
```

### 3.4 Build & test locally
```bash
snapcraft                       # produces etype_0.1.0_amd64.snap
sudo snap install ./etype_0.1.0_amd64.snap --dangerous   # local test install
etype
```

### 3.5 Publish & channels
```bash
snapcraft login
snapcraft upload --release=stable etype_0.1.0_amd64.snap
```
Users then:
```bash
sudo snap install etype          # auto-updates handled by snapd
```
Channel strategy: push pre-releases to `edge`/`beta`, promote to `stable` when verified.

### 3.6 Architectures
The Snap Store builds per-arch. `core24` + the `rust` plugin can target `amd64` and
`arm64` via Snapcraft remote-build or store-side build infra. Start with `amd64`.

---

## 4. Optional — CI automation (GitHub Actions)

Not required to ship, but removes manual steps. Proposed workflow `release.yml`, triggered
on `git tag v*`:
1. Build `--release` binary.
2. Run `cargo deb`, upload the `.deb` to the GitHub Release.
3. Run `snapcraft` (or Snapcraft's remote-build) and `snapcraft upload --release=stable`
   using a `SNAPCRAFT_STORE_CREDENTIALS` secret.

This means: **tag a version → both packages publish themselves.** Recommended as a Phase 2
follow-up once the manual path works end to end.

---

## 5. Versioning & release checklist (per release)

1. Bump `version` in `Cargo.toml` (and `snapcraft.yaml` if not auto-adopted).
2. Update `CHANGELOG.md`.
3. `cargo fmt && cargo clippy -- -D warnings && cargo test`.
4. `git tag vX.Y.Z && git push --tags`.
5. `cargo deb` → upload `.deb` to the GitHub Release.
6. `snapcraft && snapcraft upload --release=stable …`.
7. (Optional) `--version` flag — *only* if we decide reviewers/users need CLI version
   introspection. Small addition to `main.rs`; flagged, not assumed.

---

## 6. Open decisions for you

| Decision | Options | Default I'd pick |
|---|---|---|
| Architectures at launch | amd64 only / amd64 + arm64 | **amd64 only** first, add arm64 later |
| `.deb` hosting | GitHub Releases only / full APT repo | **GitHub Releases** (Phase 1) |
| CI automation | Manual now / GitHub Actions now | **Manual first**, automate in Phase 2 |
| Snap name if `etype` taken | `etype-trainer` / `etype-cli` / other | confirm at `snapcraft register` |
| Add `--version` flag | yes / no | **no** unless you want it |

---

## 7. Proposed implementation order (after this plan is approved)

1. Add `[package]` metadata fields (Section 1).
2. Add `[package.metadata.deb]`; produce + test a `.deb` locally.
3. Add `snap/snapcraft.yaml`; build + locally install the snap; verify DB writes to
   `~/snap/etype/...`.
4. Register the Snap name; first `stable` upload.
5. Cut `v0.1.0`, attach `.deb` to the GitHub Release.
6. (Phase 2) GitHub Actions release automation; arm64; optional APT repo.

All of the above lands on a `feat/packaging` branch for review before any publish.
