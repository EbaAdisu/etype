use crate::engine::finger::{key_finger, Finger, Hand};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

// ── Colour palette ────────────────────────────────────────────────────────────

fn finger_color(hand: Hand, finger: Finger) -> Color {
    match finger {
        Finger::Pinky => Color::Magenta,
        Finger::Ring => Color::Blue,
        Finger::Middle => Color::Cyan,
        Finger::Index => match hand {
            Hand::Left => Color::Green,
            Hand::Right => Color::Yellow,
        },
        Finger::Thumb => Color::White,
    }
}

fn key_style(hand: Hand, finger: Finger, active: bool) -> Style {
    let color = finger_color(hand, finger);
    if active {
        Style::default()
            .fg(Color::Black)
            .bg(color)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(color)
    }
}

// ── Key → finger lookup table ─────────────────────────────────────────────────

const ROWS: &[&[char]] = &[
    &['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
    &['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';'],
    &['z', 'x', 'c', 'v', 'b', 'n', 'm'],
];

// ── ASCII hand art ────────────────────────────────────────────────────────────
//
// Each hand is 5 rows tall:
//
//   Row 0  ┌─┐ ┌─┐ ┌─┐ ┌─┐
//   Row 1  │ │ │ │ │ │ │ │
//   Row 2  │ │ │ │ │ │ │ │  ┌─┐
//   Row 3  └─┘ └─┘ └─┘ └─┘  │ │
//   Row 4                    └─┘
//
// Left hand fingers (L→R): Pinky Ring Middle Index  (thumb appears rows 2-4)
// Right hand (L→R):        Thumb  Index Middle Ring Pinky

fn finger_box_top(hand: Hand, finger: Finger, active: bool) -> Vec<Span<'static>> {
    let s = key_style(hand, finger, active);
    vec![Span::styled("┌─┐", s)]
}

fn finger_box_mid(hand: Hand, finger: Finger, active: bool) -> Vec<Span<'static>> {
    let s = key_style(hand, finger, active);
    if active {
        vec![Span::styled("│█│", s)]
    } else {
        vec![Span::styled("│ │", s)]
    }
}

fn finger_box_bot(hand: Hand, finger: Finger, active: bool) -> Vec<Span<'static>> {
    let s = key_style(hand, finger, active);
    vec![Span::styled("└─┘", s)]
}

fn sp(n: usize) -> Span<'static> {
    Span::raw(" ".repeat(n))
}

/// Build the 5 rows of the hands display.
/// `hint` is the (hand, finger) that should shine right now.
fn build_hand_rows(active: Option<(Hand, Finger)>) -> Vec<Line<'static>> {
    let is_active =
        |h: Hand, f: Finger| -> bool { active.is_some_and(|(ah, af)| ah == h && af == f) };

    // Aliases for brevity
    let (lp, lr, lm, li, lt) = (
        is_active(Hand::Left, Finger::Pinky),
        is_active(Hand::Left, Finger::Ring),
        is_active(Hand::Left, Finger::Middle),
        is_active(Hand::Left, Finger::Index),
        is_active(Hand::Left, Finger::Thumb),
    );
    let (rt, ri, rm, rr, rp) = (
        // Right thumb shows even for space (mapped to Right Thumb)
        active.is_some_and(|(_, f)| f == Finger::Thumb),
        is_active(Hand::Right, Finger::Index),
        is_active(Hand::Right, Finger::Middle),
        is_active(Hand::Right, Finger::Ring),
        is_active(Hand::Right, Finger::Pinky),
    );

    // Row 0: four main fingers of each hand (no thumbs yet)
    let row0 = Line::from(
        [
            finger_box_top(Hand::Left, Finger::Pinky, lp),
            vec![sp(1)],
            finger_box_top(Hand::Left, Finger::Ring, lr),
            vec![sp(1)],
            finger_box_top(Hand::Left, Finger::Middle, lm),
            vec![sp(1)],
            finger_box_top(Hand::Left, Finger::Index, li),
            vec![sp(8)], // gap between hands
            finger_box_top(Hand::Right, Finger::Index, ri),
            vec![sp(1)],
            finger_box_top(Hand::Right, Finger::Middle, rm),
            vec![sp(1)],
            finger_box_top(Hand::Right, Finger::Ring, rr),
            vec![sp(1)],
            finger_box_top(Hand::Right, Finger::Pinky, rp),
        ]
        .concat(),
    );

    // Row 1: mid of four fingers
    let row1 = Line::from(
        [
            finger_box_mid(Hand::Left, Finger::Pinky, lp),
            vec![sp(1)],
            finger_box_mid(Hand::Left, Finger::Ring, lr),
            vec![sp(1)],
            finger_box_mid(Hand::Left, Finger::Middle, lm),
            vec![sp(1)],
            finger_box_mid(Hand::Left, Finger::Index, li),
            vec![sp(8)],
            finger_box_mid(Hand::Right, Finger::Index, ri),
            vec![sp(1)],
            finger_box_mid(Hand::Right, Finger::Middle, rm),
            vec![sp(1)],
            finger_box_mid(Hand::Right, Finger::Ring, rr),
            vec![sp(1)],
            finger_box_mid(Hand::Right, Finger::Pinky, rp),
        ]
        .concat(),
    );

    // Row 2: mid of four fingers + thumb tops appear
    let lt_s = key_style(Hand::Left, Finger::Thumb, lt);
    let rt_s = key_style(Hand::Right, Finger::Thumb, rt);
    let row2 = Line::from(
        [
            finger_box_mid(Hand::Left, Finger::Pinky, lp),
            vec![sp(1)],
            finger_box_mid(Hand::Left, Finger::Ring, lr),
            vec![sp(1)],
            finger_box_mid(Hand::Left, Finger::Middle, lm),
            vec![sp(1)],
            finger_box_mid(Hand::Left, Finger::Index, li),
            vec![sp(1), Span::styled("┌─┐", lt_s), sp(1)], // left thumb top
            vec![Span::styled("┌─┐", rt_s), sp(1)],        // right thumb top
            finger_box_mid(Hand::Right, Finger::Index, ri),
            vec![sp(1)],
            finger_box_mid(Hand::Right, Finger::Middle, rm),
            vec![sp(1)],
            finger_box_mid(Hand::Right, Finger::Ring, rr),
            vec![sp(1)],
            finger_box_mid(Hand::Right, Finger::Pinky, rp),
        ]
        .concat(),
    );

    // Row 3: bottom of four fingers + thumb mids
    let lt_mid = if lt {
        Span::styled("│█│", lt_s)
    } else {
        Span::styled("│ │", lt_s)
    };
    let rt_mid = if rt {
        Span::styled("│█│", rt_s)
    } else {
        Span::styled("│ │", rt_s)
    };
    let row3 = Line::from(
        [
            finger_box_bot(Hand::Left, Finger::Pinky, lp),
            vec![sp(1)],
            finger_box_bot(Hand::Left, Finger::Ring, lr),
            vec![sp(1)],
            finger_box_bot(Hand::Left, Finger::Middle, lm),
            vec![sp(1)],
            finger_box_bot(Hand::Left, Finger::Index, li),
            vec![sp(1), lt_mid, sp(1)],
            vec![rt_mid, sp(1)],
            finger_box_bot(Hand::Right, Finger::Index, ri),
            vec![sp(1)],
            finger_box_bot(Hand::Right, Finger::Middle, rm),
            vec![sp(1)],
            finger_box_bot(Hand::Right, Finger::Ring, rr),
            vec![sp(1)],
            finger_box_bot(Hand::Right, Finger::Pinky, rp),
        ]
        .concat(),
    );

    // Row 4: thumb bottoms only
    let row4 = Line::from(vec![
        sp(21),
        Span::styled("└─┘", lt_s),
        sp(1),
        Span::styled("└─┘", rt_s),
    ]);

    vec![row0, row1, row2, row3, row4]
}

// ── Keyboard rows ─────────────────────────────────────────────────────────────

fn build_keyboard_rows(active_char: Option<char>) -> Vec<Line<'static>> {
    let active =
        active_char.and_then(|c| key_finger(c).map(|h| (c.to_ascii_lowercase(), h.hand, h.finger)));

    let offsets = ["", " ", "  "]; // stagger per row

    let mut lines: Vec<Line<'static>> = ROWS
        .iter()
        .enumerate()
        .map(|(row_i, keys)| {
            let mut spans = vec![Span::raw(offsets[row_i])];
            for &k in *keys {
                let is_active = active.is_some_and(|(ac, _, _)| ac == k);
                let (hand, finger) = key_finger(k)
                    .map(|h| (h.hand, h.finger))
                    .unwrap_or((Hand::Left, Finger::Pinky));
                let label = format!("[{}]", k);
                spans.push(Span::styled(label, key_style(hand, finger, is_active)));
            }
            Line::from(spans)
        })
        .collect();

    // Space bar
    let space_active = active_char == Some(' ');
    let space_style = key_style(Hand::Right, Finger::Thumb, space_active);
    lines.push(Line::from(vec![
        sp(7),
        Span::styled("[        space        ]", space_style),
    ]));

    lines
}

// ── Public render entry point ─────────────────────────────────────────────────

/// Renders the finger guide into `area`.
/// `next_char` is the character the player needs to type next.
pub fn render_finger_guide(f: &mut Frame, area: ratatui::layout::Rect, next_char: Option<char>) {
    let hint = next_char.and_then(|c| key_finger(c).map(|h| (h.hand, h.finger)));

    let mut lines: Vec<Line> = Vec::new();

    // Hands (5 rows)
    lines.extend(build_hand_rows(hint));

    // Separator
    lines.push(Line::from(Span::raw(
        "─".repeat(area.width.saturating_sub(2) as usize),
    )));

    // Keyboard (4 rows: 3 letter rows + space)
    lines.extend(build_keyboard_rows(next_char));

    let block = Block::default()
        .title(" Finger Guide  [Tab] toggle ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let p = Paragraph::new(lines).block(block);
    f.render_widget(p, area);
}

// ── Height constant (used by callers to reserve layout space) ─────────────────

/// Total height the finger guide occupies (border + hands + sep + keyboard).
pub const GUIDE_HEIGHT: u16 = 13; // 2 border + 5 hands + 1 sep + 4 keys + 1 space row
