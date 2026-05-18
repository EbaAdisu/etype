use crate::app::App;
use crate::engine::xp::{level_from_xp, xp_for_next_level};
use crate::modes::ModeResult;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Gauge, Paragraph};
use ratatui::Frame;

pub fn render_results(f: &mut Frame, app: &App, result: &ModeResult) {
    let area = centered_rect(46, 24, f.area());

    let level = level_from_xp(app.profile.total_xp);
    let xp_to_next = xp_for_next_level(app.profile.total_xp).unwrap_or(0);
    let xp_current_level_start = level_xp_start(level);
    let xp_for_level = xp_for_next_level(xp_current_level_start).unwrap_or(1);
    let xp_in_level = app.profile.total_xp.saturating_sub(xp_current_level_start);
    let level_progress = (xp_in_level as f64 / xp_for_level as f64).clamp(0.0, 1.0);

    let new_best_str = if result.is_new_best {
        "  ★ NEW BEST!"
    } else {
        ""
    };

    let level_up = app.level_up_anim.is_some();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // title
            Constraint::Length(4), // mode/duration
            Constraint::Length(5), // stats
            Constraint::Length(6), // xp/streak
            Constraint::Length(3), // nav
        ])
        .split(area);

    let title_text = if level_up {
        vec![
            Line::from(vec![Span::styled(
                "★  LEVEL UP!  ★",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(vec![Span::styled(
                "Session Complete!",
                Style::default().fg(Color::White),
            )]),
        ]
    } else {
        vec![Line::from(vec![Span::styled(
            "Session Complete!",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )])]
    };
    let title = Paragraph::new(title_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let mode_info = Paragraph::new(vec![
        Line::from(vec![
            Span::raw("Mode:      "),
            Span::styled(
                format!("{} — {}", result.mode.label(), result.difficulty.label()),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Duration:  "),
            Span::styled(
                format!("{}s", result.duration_s),
                Style::default().fg(Color::White),
            ),
        ]),
    ])
    .block(
        Block::default()
            .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
            .border_style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(mode_info, chunks[1]);

    let new_best_style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);
    let stats = Paragraph::new(vec![
        Line::from(vec![
            Span::raw("WPM:       "),
            Span::styled(
                format!("{:.0}", result.wpm),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(new_best_str.to_string(), new_best_style),
        ]),
        Line::from(vec![
            Span::raw("CPM:       "),
            Span::styled(
                format!("{:.0}", result.cpm),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(vec![
            Span::raw("Accuracy:  "),
            Span::styled(
                format!("{:.1}%", result.accuracy),
                Style::default().fg(Color::Green),
            ),
        ]),
    ])
    .block(
        Block::default()
            .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
            .border_style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(stats, chunks[2]);

    let xp_label = format!("Level {} in {} XP", level + 1, xp_to_next);
    let xp_section_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(chunks[3]);

    let xp_block = Block::default()
        .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
        .border_style(Style::default().fg(Color::Cyan));
    f.render_widget(xp_block, chunks[3]);

    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::raw("XP Earned:    "),
            Span::styled(
                format!("+{}", result.xp_earned),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ])),
        xp_section_chunks[0],
    );
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::raw("Total XP:     "),
            Span::styled(
                format!("{}", app.profile.total_xp),
                Style::default().fg(Color::White),
            ),
        ])),
        xp_section_chunks[1],
    );
    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(Color::Blue))
        .ratio(level_progress)
        .label(xp_label);
    f.render_widget(gauge, xp_section_chunks[2]);
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::raw("Streak:       "),
            Span::styled(
                format!("{} days", app.profile.streak_days),
                Style::default().fg(Color::Yellow),
            ),
        ])),
        xp_section_chunks[3],
    );

    let nav = Paragraph::new(Line::from(vec![
        Span::styled("[r] Play again", Style::default().fg(Color::Green)),
        Span::raw("    "),
        Span::styled("[m] Main menu", Style::default().fg(Color::Blue)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(Color::Cyan)),
    )
    .alignment(Alignment::Center);
    f.render_widget(nav, chunks[4]);
}

fn level_xp_start(level: u32) -> u64 {
    match level {
        1 => 0,
        2 => 500,
        3 => 1_500,
        4 => 3_500,
        5 => 7_000,
        n => 7_000 + (n as u64 - 5) * 4_000,
    }
}

fn centered_rect(width: u16, height: u16, area: ratatui::layout::Rect) -> ratatui::layout::Rect {
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    ratatui::layout::Rect::new(x, y, width.min(area.width), height.min(area.height))
}
