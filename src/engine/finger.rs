#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Hand {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Finger {
    Pinky,
    Ring,
    Middle,
    Index,
    Thumb,
}

pub struct FingerHint {
    pub hand: Hand,
    pub finger: Finger,
}

/// Returns which hand and finger should type the given character.
pub fn key_finger(c: char) -> Option<FingerHint> {
    let (hand, finger) = match c.to_ascii_lowercase() {
        'q' | 'a' | 'z' => (Hand::Left, Finger::Pinky),
        'w' | 's' | 'x' => (Hand::Left, Finger::Ring),
        'e' | 'd' | 'c' => (Hand::Left, Finger::Middle),
        'r' | 'f' | 'v' | 't' | 'g' | 'b' => (Hand::Left, Finger::Index),
        ' ' => (Hand::Right, Finger::Thumb),
        'y' | 'h' | 'n' | 'u' | 'j' | 'm' => (Hand::Right, Finger::Index),
        'i' | 'k' | ',' => (Hand::Right, Finger::Middle),
        'o' | 'l' | '.' => (Hand::Right, Finger::Ring),
        'p' | ';' | '\'' | '/' | '[' | ']' => (Hand::Right, Finger::Pinky),
        _ => return None,
    };
    Some(FingerHint { hand, finger })
}
