use crate::engine::finger::{key_finger, Finger, Hand};
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

// ── Colour palette ────────────────────────────────────────────────────────────

const BORDER: Color = Color::DarkGray;

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

fn finger_name(f: Finger) -> &'static str {
    match f {
        Finger::Pinky => "PINKY",
        Finger::Ring => "RING",
        Finger::Middle => "MIDDLE",
        Finger::Index => "INDEX",
        Finger::Thumb => "THUMB",
    }
}

fn hand_name(h: Hand) -> &'static str {
    match h {
        Hand::Left => "LEFT",
        Hand::Right => "RIGHT",
    }
}

// ── Keyboard layout ───────────────────────────────────────────────────────────

const ROW_TOP: &[char] = &['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'];
const ROW_HOME: &[char] = &['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';'];
const ROW_BOT: &[char] = &['z', 'x', 'c', 'v', 'b', 'n', 'm'];

/// A horizontal grid border line, e.g. `╭───┬───┬───╮`.
fn border_line(indent: usize, n: usize, left: char, mid: char, right: char) -> Line<'static> {
    let mut s = String::new();
    s.push(left);
    for i in 0..n {
        s.push_str("───");
        s.push(if i + 1 < n { mid } else { right });
    }
    Line::from(vec![
        Span::raw(" ".repeat(indent)),
        Span::styled(s, Style::default().fg(BORDER)),
    ])
}

fn sep() -> Span<'static> {
    Span::styled("│", Style::default().fg(BORDER))
}

/// One row of keycaps, e.g. `│ Q │ W │ E │`, with the active key lit in its
/// finger colour and the F / J home keys underlined.
fn cells_line(indent: usize, keys: &[char], active: Option<char>) -> Line<'static> {
    let mut spans = vec![Span::raw(" ".repeat(indent)), sep()];
    for &k in keys {
        let (hand, finger) = key_finger(k)
            .map(|h| (h.hand, h.finger))
            .unwrap_or((Hand::Left, Finger::Pinky));
        let color = finger_color(hand, finger);
        let upper = k.to_ascii_uppercase();

        if active == Some(k) {
            let st = Style::default()
                .fg(Color::Black)
                .bg(color)
                .add_modifier(Modifier::BOLD);
            spans.push(Span::styled(format!(" {} ", upper), st));
        } else if k == 'f' || k == 'j' {
            // Home-row markers: underline the bump keys.
            let st = Style::default()
                .fg(color)
                .add_modifier(Modifier::UNDERLINED | Modifier::BOLD);
            spans.push(Span::raw(" "));
            spans.push(Span::styled(upper.to_string(), st));
            spans.push(Span::raw(" "));
        } else {
            spans.push(Span::styled(
                format!(" {} ", upper),
                Style::default().fg(color),
            ));
        }
        spans.push(sep());
    }
    Line::from(spans)
}

/// A connected keycap grid for one or more equal-width rows.
fn grid(indent: usize, rows: &[&[char]], active: Option<char>) -> Vec<Line<'static>> {
    let n = rows[0].len();
    let mut out = vec![border_line(indent, n, '╭', '┬', '╮')];
    for (i, row) in rows.iter().enumerate() {
        out.push(cells_line(indent, row, active));
        if i + 1 < rows.len() {
            out.push(border_line(indent, n, '├', '┼', '┤'));
        }
    }
    out.push(border_line(indent, n, '╰', '┴', '╯'));
    out
}

// ── Cue + legend ──────────────────────────────────────────────────────────────

/// The headline telling the player which key and finger comes next.
fn cue_line(next: Option<char>) -> Line<'static> {
    let mut spans = vec![Span::styled("  next ▶  ", Style::default().fg(Color::Gray))];
    match next {
        Some(' ') => {
            let white = Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD);
            spans.push(Span::styled("␣", white));
            spans.push(Span::raw("     "));
            spans.push(Span::styled("RIGHT THUMB", white));
            spans.push(Span::styled("  (space)", Style::default().fg(BORDER)));
        }
        Some(c) => match key_finger(c) {
            Some(h) => {
                let color = finger_color(h.hand, h.finger);
                let st = Style::default().fg(color).add_modifier(Modifier::BOLD);
                spans.push(Span::styled(c.to_ascii_uppercase().to_string(), st));
                spans.push(Span::raw("     "));
                spans.push(Span::styled(
                    format!("{} {}", hand_name(h.hand), finger_name(h.finger)),
                    st,
                ));
            }
            None => spans.push(Span::styled(
                c.to_string(),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )),
        },
        None => spans.push(Span::styled("ready", Style::default().fg(BORDER))),
    }
    Line::from(spans)
}

fn space_line(active: bool) -> Line<'static> {
    let style = if active {
        Style::default()
            .fg(Color::Black)
            .bg(Color::White)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(BORDER)
    };
    Line::from(vec![
        Span::raw("     "),
        Span::styled("╰─────────── space ───────────╯", style),
    ])
}

fn legend_line() -> Line<'static> {
    let chip = |label: &str, color: Color| {
        vec![
            Span::styled("▮", Style::default().fg(color)),
            Span::styled(format!(" {}  ", label), Style::default().fg(Color::Gray)),
        ]
    };
    Line::from(
        [
            vec![Span::raw("  ")],
            chip("pinky", Color::Magenta),
            chip("ring", Color::Blue),
            chip("middle", Color::Cyan),
            chip("index", Color::Green),
            chip("thumb", Color::White),
        ]
        .concat(),
    )
}

// ── Public render entry point ─────────────────────────────────────────────────

/// Renders the finger guide into `area`.
/// `next_char` is the character the player needs to type next.
pub fn render_finger_guide(f: &mut Frame, area: Rect, next_char: Option<char>) {
    let active = next_char.map(|c| c.to_ascii_lowercase());

    let mut lines: Vec<Line> = Vec::new();
    lines.push(cue_line(next_char));
    lines.push(Line::from(""));
    lines.extend(grid(0, &[ROW_TOP, ROW_HOME], active));
    lines.extend(grid(2, &[ROW_BOT], active));
    lines.push(space_line(next_char == Some(' ')));
    lines.push(legend_line());

    let block = Block::default()
        .title(" Finger Guide  [Tab] toggle ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(BORDER));

    f.render_widget(Paragraph::new(lines).block(block), area);
}

// ── Height constant (used by callers to reserve layout space) ─────────────────

/// Total height the finger guide occupies (border + cue + blank + 2 grids + space + legend).
pub const GUIDE_HEIGHT: u16 = 14;
