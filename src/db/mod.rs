use anyhow::Result;
use chrono::Local;
use rusqlite::{params, Connection};
use std::collections::HashMap;

use crate::app::Profile;

pub fn open_db() -> Result<Connection> {
    let data_dir = dirs_path();
    std::fs::create_dir_all(&data_dir)?;
    let path = data_dir.join("etype.db");
    let conn = Connection::open(path)?;
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
    create_schema(&conn)?;
    Ok(conn)
}

/// Opens an in-memory database with the full schema applied. Intended for tests.
pub fn open_in_memory() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    conn.execute_batch("PRAGMA foreign_keys=ON;")?;
    create_schema(&conn)?;
    Ok(conn)
}

fn dirs_path() -> std::path::PathBuf {
    let base = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    std::path::PathBuf::from(base)
        .join(".local")
        .join("share")
        .join("etype")
}

fn create_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS sessions (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            mode        TEXT    NOT NULL,
            difficulty  TEXT    NOT NULL,
            wpm         REAL    NOT NULL,
            cpm         REAL    NOT NULL,
            accuracy    REAL    NOT NULL,
            xp_earned   INTEGER NOT NULL,
            duration_s  INTEGER NOT NULL,
            played_at   TEXT    NOT NULL
        );

        CREATE TABLE IF NOT EXISTS key_stats (
            id           INTEGER PRIMARY KEY AUTOINCREMENT,
            session_id   INTEGER NOT NULL REFERENCES sessions(id),
            key_char     TEXT    NOT NULL,
            error_count  INTEGER NOT NULL DEFAULT 0,
            avg_delay_ms REAL    NOT NULL DEFAULT 0.0
        );

        CREATE TABLE IF NOT EXISTS personal_bests (
            mode        TEXT    NOT NULL,
            difficulty  TEXT    NOT NULL,
            wpm         REAL    NOT NULL,
            accuracy    REAL    NOT NULL,
            session_id  INTEGER REFERENCES sessions(id),
            set_at      TEXT    NOT NULL,
            PRIMARY KEY (mode, difficulty)
        );

        CREATE TABLE IF NOT EXISTS profile (
            id          INTEGER PRIMARY KEY CHECK (id = 1),
            total_xp    INTEGER NOT NULL DEFAULT 0,
            streak_days INTEGER NOT NULL DEFAULT 0,
            last_played TEXT
        );
        ",
    )?;
    Ok(())
}

pub fn load_profile(conn: &Connection) -> Result<Profile> {
    let result = conn.query_row(
        "SELECT total_xp, streak_days, last_played FROM profile WHERE id = 1",
        [],
        |row| {
            Ok(Profile {
                total_xp: row.get::<_, u64>(0)?,
                streak_days: row.get::<_, u32>(1)?,
                last_played: row.get::<_, Option<String>>(2)?,
            })
        },
    );
    match result {
        Ok(p) => Ok(p),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            conn.execute(
                "INSERT INTO profile (id, total_xp, streak_days) VALUES (1, 0, 0)",
                [],
            )?;
            Ok(Profile {
                total_xp: 0,
                streak_days: 0,
                last_played: None,
            })
        }
        Err(e) => Err(e.into()),
    }
}

pub fn update_profile(conn: &Connection, total_xp: u64, streak_days: u32) -> Result<()> {
    let today = Local::now().format("%Y-%m-%d").to_string();
    conn.execute(
        "UPDATE profile SET total_xp = ?1, streak_days = ?2, last_played = ?3 WHERE id = 1",
        params![total_xp, streak_days, today],
    )?;
    Ok(())
}

pub fn check_and_update_streak(conn: &Connection, profile: &mut Profile) -> Result<()> {
    let today = Local::now().format("%Y-%m-%d").to_string();
    match &profile.last_played {
        None => {
            profile.streak_days = 0;
        }
        Some(last) => {
            if *last == today {
                // already played today, streak unchanged
                return Ok(());
            }
            // Check if last_played was yesterday
            let yesterday = (Local::now() - chrono::Duration::days(1))
                .format("%Y-%m-%d")
                .to_string();
            if *last == yesterday {
                profile.streak_days += 1;
            } else {
                profile.streak_days = 0;
            }
        }
    }
    update_profile(conn, profile.total_xp, profile.streak_days)?;
    profile.last_played = Some(today);
    Ok(())
}

pub struct SessionRecord {
    pub mode: String,
    pub difficulty: String,
    pub wpm: f64,
    pub cpm: f64,
    pub accuracy: f64,
    pub xp_earned: u32,
    pub duration_s: u64,
}

pub fn insert_session(conn: &Connection, rec: &SessionRecord) -> Result<i64> {
    let played_at = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO sessions (mode, difficulty, wpm, cpm, accuracy, xp_earned, duration_s, played_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            rec.mode,
            rec.difficulty,
            rec.wpm,
            rec.cpm,
            rec.accuracy,
            rec.xp_earned,
            rec.duration_s,
            played_at
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn insert_key_stats(
    conn: &Connection,
    session_id: i64,
    stats: &HashMap<char, (u32, f64)>,
) -> Result<()> {
    for (ch, (errors, avg_delay)) in stats {
        conn.execute(
            "INSERT INTO key_stats (session_id, key_char, error_count, avg_delay_ms)
             VALUES (?1, ?2, ?3, ?4)",
            params![session_id, ch.to_string(), errors, avg_delay],
        )?;
    }
    Ok(())
}

pub fn upsert_personal_best(
    conn: &Connection,
    mode: &str,
    difficulty: &str,
    wpm: f64,
    accuracy: f64,
    session_id: i64,
) -> Result<bool> {
    let existing: Option<f64> = conn
        .query_row(
            "SELECT wpm FROM personal_bests WHERE mode = ?1 AND difficulty = ?2",
            params![mode, difficulty],
            |row| row.get(0),
        )
        .ok();

    if existing.is_none_or(|old_wpm| wpm > old_wpm) {
        let set_at = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        conn.execute(
            "INSERT OR REPLACE INTO personal_bests (mode, difficulty, wpm, accuracy, session_id, set_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![mode, difficulty, wpm, accuracy, session_id, set_at],
        )?;
        return Ok(true);
    }
    Ok(false)
}

pub struct SessionRow {
    pub mode: String,
    pub difficulty: String,
    pub wpm: f64,
    pub accuracy: f64,
    pub played_at: String,
}

pub fn load_recent_sessions(conn: &Connection) -> Result<Vec<SessionRow>> {
    let mut stmt = conn.prepare(
        "SELECT mode, difficulty, wpm, accuracy, played_at FROM sessions
         ORDER BY played_at DESC LIMIT 10",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(SessionRow {
            mode: row.get(0)?,
            difficulty: row.get(1)?,
            wpm: row.get(2)?,
            accuracy: row.get(3)?,
            played_at: row.get(4)?,
        })
    })?;
    Ok(rows.flatten().collect())
}

pub struct PersonalBestRow {
    pub mode: String,
    pub difficulty: String,
    pub wpm: f64,
}

pub fn load_personal_bests(conn: &Connection) -> Result<Vec<PersonalBestRow>> {
    let mut stmt = conn.prepare("SELECT mode, difficulty, wpm FROM personal_bests")?;
    let rows = stmt.query_map([], |row| {
        Ok(PersonalBestRow {
            mode: row.get(0)?,
            difficulty: row.get(1)?,
            wpm: row.get(2)?,
        })
    })?;
    Ok(rows.flatten().collect())
}

pub struct KeyHeatRow {
    pub key_char: String,
    pub total_errors: i64,
    pub avg_delay_ms: f64,
}

pub fn load_key_heatmap_data(conn: &Connection) -> Result<Vec<KeyHeatRow>> {
    let mut stmt = conn.prepare(
        "SELECT key_char, SUM(error_count), AVG(avg_delay_ms)
         FROM key_stats GROUP BY key_char ORDER BY SUM(error_count) DESC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(KeyHeatRow {
            key_char: row.get(0)?,
            total_errors: row.get(1)?,
            avg_delay_ms: row.get(2)?,
        })
    })?;
    Ok(rows.flatten().collect())
}
