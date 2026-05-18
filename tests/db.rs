use etype::db::{
    self, check_and_update_streak, insert_key_stats, insert_session, load_personal_bests,
    load_profile, load_recent_sessions, open_in_memory, update_profile, upsert_personal_best,
    SessionRecord,
};
use std::collections::HashMap;

fn conn() -> rusqlite::Connection {
    open_in_memory().expect("failed to open in-memory DB")
}

// --- profile ---

#[test]
fn profile_is_created_on_first_load() {
    let c = conn();
    let p = load_profile(&c).unwrap();
    assert_eq!(p.total_xp, 0);
    assert_eq!(p.streak_days, 0);
    assert!(p.last_played.is_none());
}

#[test]
fn profile_update_persists() {
    let c = conn();
    load_profile(&c).unwrap(); // ensure row exists
    update_profile(&c, 1234, 7).unwrap();
    let p = load_profile(&c).unwrap();
    assert_eq!(p.total_xp, 1234);
    assert_eq!(p.streak_days, 7);
    assert!(p.last_played.is_some());
}

// --- streak ---

#[test]
fn streak_increments_on_consecutive_days() {
    let c = conn();
    let mut p = load_profile(&c).unwrap();

    // Simulate last_played = yesterday
    let yesterday = (chrono::Local::now() - chrono::Duration::days(1))
        .format("%Y-%m-%d")
        .to_string();
    p.last_played = Some(yesterday);
    p.streak_days = 3;

    check_and_update_streak(&c, &mut p).unwrap();
    assert_eq!(p.streak_days, 4);
}

#[test]
fn streak_resets_when_day_missed() {
    let c = conn();
    let mut p = load_profile(&c).unwrap();

    // Simulate last_played = two days ago
    let two_days_ago = (chrono::Local::now() - chrono::Duration::days(2))
        .format("%Y-%m-%d")
        .to_string();
    p.last_played = Some(two_days_ago);
    p.streak_days = 10;

    check_and_update_streak(&c, &mut p).unwrap();
    assert_eq!(p.streak_days, 0);
}

// --- sessions ---

fn sample_session() -> SessionRecord {
    SessionRecord {
        mode: "word_rush".to_string(),
        difficulty: "medium".to_string(),
        wpm: 72.5,
        cpm: 362.5,
        accuracy: 96.3,
        xp_earned: 85,
        duration_s: 60,
    }
}

#[test]
fn insert_and_load_session() {
    let c = conn();
    insert_session(&c, &sample_session()).unwrap();
    let sessions = load_recent_sessions(&c).unwrap();
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0].mode, "word_rush");
    assert!((sessions[0].wpm - 72.5).abs() < 0.01);
}

#[test]
fn recent_sessions_capped_at_10() {
    let c = conn();
    for _ in 0..15 {
        insert_session(&c, &sample_session()).unwrap();
    }
    let sessions = load_recent_sessions(&c).unwrap();
    assert_eq!(sessions.len(), 10);
}

// --- key stats ---

#[test]
fn key_stats_round_trip() {
    let c = conn();
    let session_id = insert_session(&c, &sample_session()).unwrap();

    let mut stats: HashMap<char, (u32, f64)> = HashMap::new();
    stats.insert('f', (3, 210.5));
    stats.insert('p', (1, 180.0));
    insert_key_stats(&c, session_id, &stats).unwrap();

    let heatmap = db::load_key_heatmap_data(&c).unwrap();
    let f_row = heatmap.iter().find(|r| r.key_char == "f").unwrap();
    assert_eq!(f_row.total_errors, 3);
    assert!((f_row.avg_delay_ms - 210.5).abs() < 0.01);
}

// --- personal bests ---

#[test]
fn first_session_is_always_a_new_best() {
    let c = conn();
    let session_id = insert_session(&c, &sample_session()).unwrap();
    let is_new = upsert_personal_best(&c, "word_rush", "medium", 72.5, 96.3, session_id).unwrap();
    assert!(is_new);
}

#[test]
fn higher_wpm_replaces_personal_best() {
    let c = conn();
    let id1 = insert_session(&c, &sample_session()).unwrap();
    upsert_personal_best(&c, "word_rush", "medium", 72.5, 96.3, id1).unwrap();

    let id2 = insert_session(&c, &sample_session()).unwrap();
    let is_new = upsert_personal_best(&c, "word_rush", "medium", 90.0, 98.0, id2).unwrap();
    assert!(is_new);

    let pbs = load_personal_bests(&c).unwrap();
    let pb = pbs
        .iter()
        .find(|r| r.mode == "word_rush" && r.difficulty == "medium")
        .unwrap();
    assert!((pb.wpm - 90.0).abs() < 0.01);
}

#[test]
fn lower_wpm_does_not_replace_personal_best() {
    let c = conn();
    let id1 = insert_session(&c, &sample_session()).unwrap();
    upsert_personal_best(&c, "word_rush", "medium", 90.0, 98.0, id1).unwrap();

    let id2 = insert_session(&c, &sample_session()).unwrap();
    let is_new = upsert_personal_best(&c, "word_rush", "medium", 60.0, 92.0, id2).unwrap();
    assert!(!is_new);

    let pbs = load_personal_bests(&c).unwrap();
    let pb = pbs
        .iter()
        .find(|r| r.mode == "word_rush" && r.difficulty == "medium")
        .unwrap();
    assert!(
        (pb.wpm - 90.0).abs() < 0.01,
        "PB should still be 90, got {}",
        pb.wpm
    );
}

#[test]
fn personal_bests_are_independent_per_slot() {
    let c = conn();
    let id = insert_session(&c, &sample_session()).unwrap();
    upsert_personal_best(&c, "word_rush", "easy", 80.0, 97.0, id).unwrap();
    upsert_personal_best(&c, "word_rush", "hard", 50.0, 94.0, id).unwrap();
    upsert_personal_best(&c, "sentence", "medium", 65.0, 99.0, id).unwrap();

    let pbs = load_personal_bests(&c).unwrap();
    assert_eq!(pbs.len(), 3);
}
