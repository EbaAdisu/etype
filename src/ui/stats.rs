use crate::app::App;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn render_stats(f: &mut Frame, app: &App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // title
            Constraint::Length(8), // PBs
            Constraint::Length(1), // separator
            Constraint::Min(12),   // recent sessions
            Constraint::Length(1), // nav
        ])
        .split(area);

    let title = Paragraph::new(Line::from(vec![Span::styled(
        "Stats & History",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )]))
    .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    render_personal_bests(f, app, chunks[1]);

    let sep = Paragraph::new(Line::from(vec![Span::styled(
        "─".repeat(area.width as usize),
        Style::default().fg(Color::DarkGray),
    )]));
    f.render_widget(sep, chunks[2]);

    render_recent_sessions(f, app, chunks[3]);

    let nav = Paragraph::new(Line::from(vec![Span::styled(
        "[Esc] Back",
        Style::default().fg(Color::DarkGray),
    )]))
    .alignment(Alignment::Center);
    f.render_widget(nav, chunks[4]);
}

fn render_personal_bests(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let pb_map = &app.cached_pbs;

    let modes = ["word_rush", "sentence", "code", "survival"];
    let mode_labels = ["Word Rush", "Sentence ", "Code     ", "Survival "];
    let diffs = ["easy", "medium", "hard", "insane"];
    let _diff_labels = ["Easy", "Med ", "Hard", "Ins "];

    let header = Line::from(vec![
        Span::styled(
            "Personal Bests",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("         "),
        Span::styled("Easy  ", Style::default().fg(Color::Green)),
        Span::styled("Med   ", Style::default().fg(Color::Yellow)),
        Span::styled("Hard  ", Style::default().fg(Color::Red)),
        Span::styled("Ins   ", Style::default().fg(Color::Magenta)),
    ]);

    let mut lines = vec![header, Line::from("─".repeat(50))];
    for (mi, mode) in modes.iter().enumerate() {
        let mut spans = vec![Span::styled(
            format!("  {}  ", mode_labels[mi]),
            Style::default().fg(Color::White),
        )];
        for diff in &diffs {
            let key = (mode.to_string(), diff.to_string());
            let val = pb_map.get(&key);
            let text = val.map_or("  — ".to_string(), |wpm| format!("{:>3.0} ", wpm));
            spans.push(Span::styled(text, Style::default().fg(Color::Cyan)));
            spans.push(Span::raw("  "));
        }
        lines.push(Line::from(spans));
    }

    let p = Paragraph::new(lines).block(Block::default().borders(Borders::ALL));
    f.render_widget(p, area);
}

fn render_recent_sessions(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let sessions = &app.cached_sessions;

    let header = Line::from(vec![Span::styled(
        "  Date         Mode          Diff    WPM    Acc",
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    )]);
    let sep = Line::from("  ".to_string() + &"─".repeat(48));

    let mut lines = vec![
        Line::from(vec![Span::styled(
            "Recent Sessions",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )]),
        header,
        sep,
    ];

    for s in sessions {
        let date = &s.played_at[..10.min(s.played_at.len())];
        lines.push(Line::from(vec![Span::styled(
            format!(
                "  {}  {:12}  {:6}  {:>5.0}  {:>5.1}%",
                date, s.mode, s.difficulty, s.wpm, s.accuracy
            ),
            Style::default().fg(Color::Cyan),
        )]));
    }

    if sessions.is_empty() {
        lines.push(Line::from(vec![Span::styled(
            "  No sessions yet.",
            Style::default().fg(Color::DarkGray),
        )]));
    }

    let p = Paragraph::new(lines).block(Block::default().borders(Borders::ALL));
    f.render_widget(p, area);
}
