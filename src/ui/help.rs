use ratatui::layout::Alignment;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn render_help(f: &mut Frame) {
    let area = centered_rect(50, 24, f.area());

    let lines = vec![
        Line::from(vec![Span::styled(
            "Global",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  q / Ctrl+C   ", Style::default().fg(Color::Cyan)),
            Span::raw("Quit"),
        ]),
        Line::from(vec![
            Span::styled("  Esc          ", Style::default().fg(Color::Cyan)),
            Span::raw("Back / Cancel"),
        ]),
        Line::from(vec![
            Span::styled("  ?            ", Style::default().fg(Color::Cyan)),
            Span::raw("Toggle this screen"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "In-Game",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  Space        ", Style::default().fg(Color::Cyan)),
            Span::raw("Confirm word (Word Rush / Sentence)"),
        ]),
        Line::from(vec![
            Span::styled("  Enter        ", Style::default().fg(Color::Cyan)),
            Span::raw("Confirm line (Code Snippets)"),
        ]),
        Line::from(vec![
            Span::styled("  Backspace    ", Style::default().fg(Color::Cyan)),
            Span::raw("Delete last character"),
        ]),
        Line::from(vec![
            Span::styled("  Ctrl+W       ", Style::default().fg(Color::Cyan)),
            Span::raw("Clear entire current input"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Navigation",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  1-4          ", Style::default().fg(Color::Cyan)),
            Span::raw("Select mode or difficulty"),
        ]),
        Line::from(vec![
            Span::styled("  s            ", Style::default().fg(Color::Cyan)),
            Span::raw("Stats screen"),
        ]),
        Line::from(vec![
            Span::styled("  h            ", Style::default().fg(Color::Cyan)),
            Span::raw("Heatmap screen"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "[Esc] Back",
            Style::default().fg(Color::DarkGray),
        )]),
    ];

    let p = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" Help ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Left);
    f.render_widget(p, area);
}

fn centered_rect(width: u16, height: u16, area: ratatui::layout::Rect) -> ratatui::layout::Rect {
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    ratatui::layout::Rect::new(x, y, width.min(area.width), height.min(area.height))
}
