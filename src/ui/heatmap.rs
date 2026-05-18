use crate::app::App;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

const ROW1: &[char] = &['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'];
const ROW2: &[char] = &['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'];
const ROW3: &[char] = &['z', 'x', 'c', 'v', 'b', 'n', 'm'];

pub fn render_heatmap(f: &mut Frame, app: &App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // title
            Constraint::Length(2), // legend
            Constraint::Length(5), // keyboard rows
            Constraint::Length(1), // spacer
            Constraint::Min(6),    // worst keys
            Constraint::Length(1), // nav
        ])
        .split(area);

    let title = Paragraph::new(Line::from(vec![Span::styled(
        "Key Heatmap (all-time)",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )]))
    .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let legend = Paragraph::new(Line::from(vec![
        Span::styled("Green", Style::default().fg(Color::Green)),
        Span::raw(" = fast & clean    "),
        Span::styled("Red", Style::default().fg(Color::Red)),
        Span::raw(" = slow or error-prone"),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(legend, chunks[1]);

    render_keyboard(f, app, chunks[2]);
    render_worst_keys(f, app, chunks[4]);

    let nav = Paragraph::new(Line::from(vec![Span::styled(
        "[Esc] Back",
        Style::default().fg(Color::DarkGray),
    )]))
    .alignment(Alignment::Center);
    f.render_widget(nav, chunks[5]);
}

fn key_color(errors: i64, avg_delay: f64) -> Color {
    let error_score = (errors as f64 * 2.0).min(10.0);
    let delay_score = ((avg_delay - 100.0).max(0.0) / 50.0).min(10.0);
    let total = (error_score + delay_score) / 2.0;

    if total < 2.0 {
        Color::Green
    } else if total < 5.0 {
        Color::Yellow
    } else {
        Color::Red
    }
}

fn render_keyboard(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let hmap = &app.cached_heatmap;

    let build_row = |row: &[char], offset: &str| -> Line<'static> {
        let mut spans = vec![Span::raw(offset.to_string())];
        for ch in row {
            let (errors, avg_delay) = hmap.get(ch).copied().unwrap_or((0, 0.0));
            let color = key_color(errors, avg_delay);
            spans.push(Span::styled(
                format!("[{}]", ch),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ));
        }
        Line::from(spans)
    };

    let lines = vec![
        build_row(ROW1, "  "),
        build_row(ROW2, "   "),
        build_row(ROW3, "    "),
    ];

    let p = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Left);
    f.render_widget(p, area);
}

fn render_worst_keys(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let hmap = &app.cached_heatmap;
    let mut sorted: Vec<(char, i64, f64)> = hmap
        .iter()
        .map(|(ch, (errors, delay))| (*ch, *errors, *delay))
        .collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    let mut lines = vec![Line::from(vec![Span::styled(
        "Worst keys:",
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    )])];
    for (ch, errors, delay) in sorted.iter().take(5) {
        lines.push(Line::from(vec![Span::styled(
            format!("  {} — {} errors  (avg {:.0}ms)", ch, errors, delay),
            Style::default().fg(Color::Red),
        )]));
    }
    if sorted.is_empty() {
        lines.push(Line::from(vec![Span::styled(
            "  No data yet.",
            Style::default().fg(Color::DarkGray),
        )]));
    }

    let p = Paragraph::new(lines).block(Block::default().borders(Borders::ALL));
    f.render_widget(p, area);
}
