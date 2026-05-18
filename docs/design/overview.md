# Project Overview

## What Is etype

etype is a fully local, terminal-based typing trainer for Ubuntu.  
It runs entirely in the terminal — no browser, no Electron app, no server, no internet.

## Goals

- Train typing speed and accuracy through varied, fun game modes
- Collect detailed profiling data: WPM, CPM, accuracy, and per-key heatmap
- Reward consistent daily practice with a streak + XP progression system
- Keep everything on disk — private, offline, yours

## Non-Goals (v1)

- No multiplayer or online leaderboards
- No mobile or web interface
- No cloud sync
- No custom keybinding configuration (v2 candidate)
- No sound effects (terminal bell experiments are v2)

## Who This Is For

One user, one machine. The game stores a single player profile locally.  
Multi-user support (named profiles, shared leaderboard) is planned for v2.

## What "Done" Looks Like

A single `etype` binary that:
1. Runs on any Ubuntu terminal without extra dependencies
2. Lets you play all 4 modes with difficulty selection
3. Saves your session history to SQLite
4. Shows a key heatmap and personal bests
5. Tracks XP, level, and daily streak across sessions
