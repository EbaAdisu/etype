pub fn wpm(correct_chars: u32, elapsed_secs: f64) -> f64 {
    if elapsed_secs < 0.1 {
        return 0.0;
    }
    (correct_chars as f64 / 5.0) / elapsed_secs * 60.0
}

pub fn cpm(correct_chars: u32, elapsed_secs: f64) -> f64 {
    if elapsed_secs < 0.1 {
        return 0.0;
    }
    correct_chars as f64 / elapsed_secs * 60.0
}

pub fn accuracy(correct: u32, total: u32) -> f64 {
    if total == 0 {
        return 100.0;
    }
    (correct as f64 / total as f64) * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wpm() {
        // 300 chars in 60s = 60 WPM (300/5=60 words, 60/60*60=60)
        let result = wpm(300, 60.0);
        assert!(
            (result - 60.0).abs() < 0.01,
            "expected 60 WPM, got {result}"
        );
    }

    #[test]
    fn test_cpm() {
        // 300 chars in 60s = 300 CPM
        let result = cpm(300, 60.0);
        assert!(
            (result - 300.0).abs() < 0.01,
            "expected 300 CPM, got {result}"
        );
    }

    #[test]
    fn test_accuracy() {
        assert!((accuracy(95, 100) - 95.0).abs() < 0.01);
        assert!((accuracy(0, 0) - 100.0).abs() < 0.01);
        assert!((accuracy(50, 100) - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_wpm_zero_elapsed() {
        assert_eq!(wpm(100, 0.0), 0.0);
    }
}
