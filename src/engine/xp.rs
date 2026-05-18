use crate::modes::{Difficulty, Mode};

pub fn calculate_xp(
    wpm: f64,
    accuracy: f64,
    mode: &Mode,
    diff: &Difficulty,
    streak_days: u32,
) -> u32 {
    let accuracy_mul = accuracy / 100.0;
    let mode_bonus = match mode {
        Mode::WordRush => 1.0,
        Mode::Sentence => 1.1,
        Mode::Code => 1.3,
        Mode::Survival => 1.2,
    };
    let diff_bonus = match diff {
        Difficulty::Easy => 0.8,
        Difficulty::Medium => 1.0,
        Difficulty::Hard => 1.3,
        Difficulty::Insane => 1.6,
    };
    let sb = streak_bonus(streak_days);
    (wpm * accuracy_mul * mode_bonus * diff_bonus * sb) as u32
}

pub fn streak_bonus(streak_days: u32) -> f64 {
    (1.0 + streak_days as f64 * 0.05).min(2.0)
}

pub fn level_from_xp(total_xp: u64) -> u32 {
    let thresholds: &[u64] = &[0, 500, 1_500, 3_500, 7_000];
    let mut level = 1u32;
    for (i, &threshold) in thresholds.iter().enumerate() {
        if total_xp >= threshold {
            level = (i + 1) as u32;
        } else {
            break;
        }
    }
    // Levels 6+ require 4000 XP each after 7000
    if total_xp >= 7_000 {
        let extra = total_xp - 7_000;
        level = 5 + (extra / 4_000) as u32 + 1;
    }
    level
}

pub fn xp_for_next_level(total_xp: u64) -> Option<u64> {
    let thresholds: &[u64] = &[500, 1_500, 3_500, 7_000];
    for &threshold in thresholds {
        if total_xp < threshold {
            return Some(threshold - total_xp);
        }
    }
    // Level 5+ uses 4000 increments
    let extra = total_xp - 7_000;
    let next_boundary = ((extra / 4_000) + 1) * 4_000 + 7_000;
    Some(next_boundary - total_xp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_from_xp() {
        assert_eq!(level_from_xp(0), 1);
        assert_eq!(level_from_xp(499), 1);
        assert_eq!(level_from_xp(500), 2);
        assert_eq!(level_from_xp(1_499), 2);
        assert_eq!(level_from_xp(1_500), 3);
        assert_eq!(level_from_xp(3_500), 4);
        assert_eq!(level_from_xp(7_000), 6);
    }

    #[test]
    fn test_streak_bonus() {
        assert!((streak_bonus(0) - 1.0).abs() < 0.001);
        assert!((streak_bonus(10) - 1.5).abs() < 0.001);
        // Caps at 2.0
        assert!((streak_bonus(100) - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_calculate_xp() {
        // 70 WPM, 95% accuracy, Hard Word Rush, 10-day streak
        // = 70 * 0.95 * 1.0 * 1.3 * 1.5 = ~130
        let xp = calculate_xp(70.0, 95.0, &Mode::WordRush, &Difficulty::Hard, 10);
        assert!(xp >= 125 && xp <= 135, "expected ~130, got {xp}");
    }
}
