# Database

## Location

```
~/.local/share/etype/etype.db
```

Created automatically on first launch. No manual setup needed.

## Schema

### `sessions` — one row per completed game session

```sql
CREATE TABLE IF NOT EXISTS sessions (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    mode        TEXT    NOT NULL,       -- word_rush | sentence | code | survival
    difficulty  TEXT    NOT NULL,       -- easy | medium | hard | insane
    wpm         REAL    NOT NULL,
    cpm         REAL    NOT NULL,
    accuracy    REAL    NOT NULL,       -- 0.0 to 100.0
    xp_earned   INTEGER NOT NULL,
    duration_s  INTEGER NOT NULL,       -- session length in seconds
    played_at   TEXT    NOT NULL        -- ISO 8601: "2026-05-31T14:22:00"
);
```

### `key_stats` — per-key data aggregated per session

```sql
CREATE TABLE IF NOT EXISTS key_stats (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id   INTEGER NOT NULL REFERENCES sessions(id),
    key_char     TEXT    NOT NULL,      -- single character e.g. "f", "p", " "
    error_count  INTEGER NOT NULL DEFAULT 0,
    avg_delay_ms REAL    NOT NULL DEFAULT 0.0
);
```

### `personal_bests` — best WPM per mode + difficulty combination

```sql
CREATE TABLE IF NOT EXISTS personal_bests (
    mode        TEXT    NOT NULL,
    difficulty  TEXT    NOT NULL,
    wpm         REAL    NOT NULL,
    accuracy    REAL    NOT NULL,
    session_id  INTEGER REFERENCES sessions(id),
    set_at      TEXT    NOT NULL,       -- ISO 8601
    PRIMARY KEY (mode, difficulty)      -- 16 possible slots
);
```

### `profile` — single-row player profile

```sql
CREATE TABLE IF NOT EXISTS profile (
    id          INTEGER PRIMARY KEY CHECK (id = 1),
    total_xp    INTEGER NOT NULL DEFAULT 0,
    streak_days INTEGER NOT NULL DEFAULT 0,
    last_played TEXT                        -- ISO 8601 date: "2026-05-31"
);
```

> Level is never stored. Always derive it from `total_xp` using `level_from_xp()` in `engine/xp.rs`.

## Key Queries

| Operation                | Table           | Notes                                         |
|--------------------------|-----------------|-----------------------------------------------|
| Save session             | sessions        | INSERT, return last_insert_rowid()            |
| Save key stats           | key_stats       | INSERT one row per unique key in session      |
| Update personal best     | personal_bests  | INSERT OR REPLACE when wpm > existing         |
| Load last 10 sessions    | sessions        | ORDER BY played_at DESC LIMIT 10              |
| Load all personal bests  | personal_bests  | SELECT all 16 rows                            |
| Update profile XP        | profile         | UPDATE total_xp, streak_days, last_played     |
| Load profile on startup  | profile         | SELECT id=1, INSERT if missing                |

## Migration Strategy

No migration framework. All tables use `CREATE TABLE IF NOT EXISTS`.  
If a breaking schema change is needed in the future, bump a `schema_version` table and handle it manually in `db/mod.rs`.
