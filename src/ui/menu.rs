use crate::app::App;
use crate::engine::xp::level_from_xp;
use crate::modes::{Difficulty, Mode};
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

pub fn render_menu(f: &mut Frame, app: &App) {
    let level = level_from_xp(app.profile.total_xp);
    let streak = app.profile.streak_days;
    let total_xp = app.profile.total_xp;

    let area = centered_rect(44, 22, f.area());

    let header_text = vec![
        Line::from(vec![Span::styled(
            "e t y p e",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            format!("Level {}  •  streak: {} days", level, streak),
            Style::default().fg(Color::Yellow),
        )]),
        Line::from(vec![Span::styled(
            format!("Total XP: {}", total_xp),
            Style::default().fg(Color::DarkGray),
        )]),
    ];

    let menu_items = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "[1]  Word Rush",
            Style::default().fg(Color::Green),
        )]),
        Line::from(vec![Span::styled(
            "[2]  Sentence Mode",
            Style::default().fg(Color::Green),
        )]),
        Line::from(vec![Span::styled(
            "[3]  Code Snippets",
            Style::default().fg(Color::Green),
        )]),
        Line::from(vec![Span::styled(
            "[4]  Survival",
            Style::default().fg(Color::Green),
        )]),
        Line::from(""),
    ];

    let footer_items = vec![
        Line::from(vec![Span::styled(
            "[s]  Stats & History",
            Style::default().fg(Color::Blue),
        )]),
        Line::from(vec![Span::styled(
            "[h]  Key Heatmap",
            Style::default().fg(Color::Blue),
        )]),
        Line::from(vec![Span::styled(
            "[?]  Help",
            Style::default().fg(Color::Blue),
        )]),
        Line::from(vec![Span::styled(
            "[q]  Quit",
            Style::default().fg(Color::DarkGray),
        )]),
    ];

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Length(8),
            Constraint::Length(6),
        ])
        .split(area);

    let header = Paragraph::new(header_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Center);
    f.render_widget(header, chunks[0]);

    let menu = Paragraph::new(menu_items)
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Center);
    f.render_widget(menu, chunks[1]);

    let footer = Paragraph::new(footer_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Center);
    f.render_widget(footer, chunks[2]);
}

pub fn render_difficulty_select(
    f: &mut Frame,
    app: &App,
    mode: &Mode,
    selected_diff: usize,
    selected_timer: usize,
) {
    let diffs = [
        Difficulty::Easy,
        Difficulty::Medium,
        Difficulty::Hard,
        Difficulty::Insane,
    ];
    let timers = ["30s", "60s", "120s"];

    let area = centered_rect(44, 16, f.area());
    let title = format!(" {} — Select Difficulty ", mode.label());

    let mut lines = vec![Line::from("")];
    for (i, diff) in diffs.iter().enumerate() {
        let locked = !app.difficulty_unlocked(diff);
        let marker = if i == selected_diff { "► " } else { "  " };
        let lock_str = if locked {
            format!("  (Lvl {}+) 🔒", diff.unlock_level())
        } else {
            format!("  (Lvl {}+)", diff.unlock_level())
        };
        let style = if locked {
            Style::default().fg(Color::DarkGray)
        } else if i == selected_diff {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        lines.push(Line::from(vec![Span::styled(
            format!("[{}] {}{}{}", i + 1, marker, diff.label(), lock_str),
            style,
        )]));
    }

    lines.push(Line::from(""));
    // Timer row (only for word rush mode)
    if *mode == Mode::WordRush {
        let timer_spans: Vec<Span> = timers
            .iter()
            .enumerate()
            .flat_map(|(i, t)| {
                let style = if i == selected_timer {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
                } else {
                    Style::default().fg(Color::White)
                };
                vec![Span::styled(format!("[{}]", t), style), Span::raw("  ")]
            })
            .collect();
        lines.push(Line::from(
            vec![Span::raw("Timer: ")]
                .into_iter()
                .chain(timer_spans)
                .collect::<Vec<_>>(),
        ));
        lines.push(Line::from(""));
    }

    lines.push(Line::from(vec![
        Span::styled("[Enter] Start", Style::default().fg(Color::Green)),
        Span::raw("    "),
        Span::styled("[Esc] Back", Style::default().fg(Color::DarkGray)),
    ]));

    let p = Paragraph::new(lines)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Center);
    f.render_widget(p, area);
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    Rect::new(x, y, width.min(area.width), height.min(area.height))
}
