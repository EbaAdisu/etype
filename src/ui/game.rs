use crate::engine::scorer::{cpm, wpm};
use crate::modes::code_snippet::CodeState;
use crate::modes::sentence::SentenceState;
use crate::modes::survival::SurvivalState;
use crate::modes::word_rush::WordRushState;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Gauge, Paragraph};
use ratatui::Frame;

pub fn render_word_rush(f: &mut Frame, state: &WordRushState) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // timer bar
            Constraint::Length(3), // stats
            Constraint::Length(1), // separator
            Constraint::Length(3), // word line
            Constraint::Length(3), // input
            Constraint::Min(0),
        ])
        .split(area);

    let (remaining, fraction) = state
        .timer
        .as_ref()
        .map(|t| (t.remaining, t.fraction))
        .unwrap_or((state.duration_secs as f64, 1.0));

    let elapsed = state.duration_secs as f64 - remaining;
    let w = wpm(state.correct_chars, elapsed.max(0.1));
    let c = cpm(state.correct_chars, elapsed.max(0.1));
    let total = state.correct_chars + state.total_keystrokes.saturating_sub(state.correct_chars);
    let acc = if total == 0 {
        100.0
    } else {
        state.correct_chars as f64 / total as f64 * 100.0
    };

    let gauge_label = format!("{:.0}s", remaining);
    let gauge_ratio = fraction.clamp(0.0, 1.0);
    let gauge_color = if fraction > 0.5 {
        Color::Green
    } else if fraction > 0.25 {
        Color::Yellow
    } else {
        Color::Red
    };
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(" Word Rush "))
        .gauge_style(Style::default().fg(gauge_color))
        .ratio(gauge_ratio)
        .label(gauge_label);
    f.render_widget(gauge, chunks[0]);

    let stats = Paragraph::new(Line::from(vec![
        Span::styled(format!("WPM: {:.0}", w), Style::default().fg(Color::Cyan)),
        Span::raw("   "),
        Span::styled(format!("CPM: {:.0}", c), Style::default().fg(Color::Cyan)),
        Span::raw("   "),
        Span::styled(
            format!("Accuracy: {:.1}%", acc),
            Style::default().fg(Color::Cyan),
        ),
    ]))
    .block(Block::default().borders(Borders::NONE));
    f.render_widget(stats, chunks[1]);

    // Word display
    let word_line = build_word_line(state);
    let word_p =
        Paragraph::new(word_line).block(Block::default().borders(Borders::LEFT | Borders::RIGHT));
    f.render_widget(word_p, chunks[3]);

    // Input
    let input_style = if state.error_flash {
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Yellow)
    };
    let input_p = Paragraph::new(Line::from(vec![
        Span::raw("> "),
        Span::styled(state.input.clone(), input_style),
        Span::styled("█", Style::default().fg(Color::Yellow)),
    ]))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(input_p, chunks[4]);
}

fn build_word_line(state: &WordRushState) -> Line<'static> {
    let start = state.current_idx.saturating_sub(2);
    let end = (state.current_idx + 8).min(state.words.len());
    let mut spans = Vec::new();
    for (i, word) in state.words[start..end].iter().enumerate() {
        let actual_idx = start + i;
        if actual_idx < state.current_idx {
            spans.push(Span::styled(
                word.clone(),
                Style::default().fg(Color::DarkGray),
            ));
        } else if actual_idx == state.current_idx {
            spans.push(Span::styled(
                format!("[{}]", word),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ));
        } else {
            spans.push(Span::styled(word.clone(), Style::default().fg(Color::Gray)));
        }
        spans.push(Span::raw("  "));
    }
    Line::from(spans)
}

pub fn render_sentence(f: &mut Frame, state: &SentenceState) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // stats
            Constraint::Length(1),
            Constraint::Min(5),    // quote display
            Constraint::Length(3), // input
        ])
        .split(area);

    let elapsed = state
        .start_time
        .map(|t| t.elapsed().as_secs_f64())
        .unwrap_or(0.1);
    let c = cpm(state.correct_chars, elapsed.max(0.1));
    let total = state.correct_chars + state.total_keystrokes.saturating_sub(state.correct_chars);
    let acc = if total == 0 {
        100.0
    } else {
        state.correct_chars as f64 / total as f64 * 100.0
    };

    let stats = Paragraph::new(Line::from(vec![
        Span::styled(format!("CPM: {:.0}", c), Style::default().fg(Color::Cyan)),
        Span::raw("   "),
        Span::styled(
            format!("Accuracy: {:.1}%", acc),
            Style::default().fg(Color::Cyan),
        ),
        Span::raw("   "),
        Span::styled(
            format!("Words: {}/{}", state.current_word, state.words.len()),
            Style::default().fg(Color::Cyan),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Sentence Mode "),
    );
    f.render_widget(stats, chunks[0]);

    let quote_line = build_sentence_display(state);
    let quote_p = Paragraph::new(quote_line)
        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT))
        .wrap(ratatui::widgets::Wrap { trim: false });
    f.render_widget(quote_p, chunks[2]);

    let input_p = Paragraph::new(Line::from(vec![
        Span::raw("> "),
        Span::styled(state.input.clone(), Style::default().fg(Color::Yellow)),
        Span::styled("█", Style::default().fg(Color::Yellow)),
    ]))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(input_p, chunks[3]);
}

fn build_sentence_display(state: &SentenceState) -> Line<'static> {
    let mut spans = Vec::new();
    for (i, word) in state.words.iter().enumerate() {
        if i < state.current_word {
            spans.push(Span::styled(
                word.clone(),
                Style::default().fg(Color::DarkGray),
            ));
        } else if i == state.current_word {
            // Show the word being typed with char coloring
            for (ci, ch) in word.chars().enumerate() {
                if ci < state.input.len() {
                    let typed = state.input.chars().nth(ci).unwrap();
                    let style = if typed == ch {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default().fg(Color::Red)
                    };
                    spans.push(Span::styled(ch.to_string(), style));
                } else {
                    spans.push(Span::styled(
                        ch.to_string(),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ));
                }
            }
        } else {
            spans.push(Span::styled(word.clone(), Style::default().fg(Color::Gray)));
        }
        if i < state.words.len() - 1 {
            spans.push(Span::raw(" "));
        }
    }
    Line::from(spans)
}

pub fn render_code(f: &mut Frame, state: &CodeState) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(area);

    let title = format!(" Code Snippets — {} ", state.lang.label());
    let stats_title = format!("Line: {}/{}", state.current_line + 1, state.lines.len());
    let stats = Paragraph::new(Line::from(vec![Span::styled(
        stats_title,
        Style::default().fg(Color::Cyan),
    )]))
    .block(Block::default().borders(Borders::ALL).title(title));
    f.render_widget(stats, chunks[0]);

    // Code display
    let mut code_lines: Vec<Line> = Vec::new();
    for (i, line) in state.lines.iter().enumerate() {
        if i < state.current_line {
            code_lines.push(Line::from(vec![
                Span::styled("  ✓  ", Style::default().fg(Color::Green)),
                Span::styled(line.clone(), Style::default().fg(Color::DarkGray)),
            ]));
        } else if i == state.current_line {
            code_lines.push(Line::from(vec![
                Span::styled("  ►  ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    line.clone(),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
        } else {
            code_lines.push(Line::from(vec![
                Span::raw("     "),
                Span::styled(line.clone(), Style::default().fg(Color::Gray)),
            ]));
        }
    }
    let code_p = Paragraph::new(code_lines).block(Block::default().borders(Borders::ALL));
    f.render_widget(code_p, chunks[1]);

    let input_style = if state.error_flash {
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Yellow)
    };
    let input_p = Paragraph::new(Line::from(vec![
        Span::raw("> "),
        Span::styled(state.input.clone(), input_style),
        Span::styled("█", Style::default().fg(Color::Yellow)),
    ]))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(input_p, chunks[2]);
}

pub fn render_survival(f: &mut Frame, state: &SurvivalState) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(area);

    let lives_str = "♥ ".repeat(state.lives as usize);
    let stats = Paragraph::new(Line::from(vec![
        Span::styled(
            format!("Lives: {}", lives_str.trim()),
            Style::default().fg(Color::Red),
        ),
        Span::raw("   "),
        Span::styled(
            format!("Score: {}", state.score),
            Style::default().fg(Color::Cyan),
        ),
        Span::raw("   "),
        Span::styled(
            format!("Speed: {}x", state.speed_level),
            Style::default().fg(Color::Yellow),
        ),
    ]))
    .block(Block::default().borders(Borders::ALL).title(" Survival "));
    f.render_widget(stats, chunks[0]);

    // Falling words area
    render_falling_words(f, state, chunks[1]);

    let input_p = Paragraph::new(Line::from(vec![
        Span::raw("> "),
        Span::styled(state.input.clone(), Style::default().fg(Color::Yellow)),
        Span::styled("█", Style::default().fg(Color::Yellow)),
    ]))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(input_p, chunks[2]);
}

fn render_falling_words(f: &mut Frame, state: &SurvivalState, area: Rect) {
    let block = Block::default().borders(Borders::ALL);
    let inner = block.inner(area);
    f.render_widget(block, area);

    let height = inner.height;
    let danger_row = (height as f32 * 0.85) as u16;

    // Draw danger zone line
    let danger_line = "─".repeat(inner.width as usize) + " danger zone ";
    let danger_p = Paragraph::new(Line::from(vec![Span::styled(
        danger_line,
        Style::default().fg(Color::Red),
    )]));
    let danger_area = Rect::new(
        inner.x,
        inner.y + danger_row.min(height.saturating_sub(1)),
        inner.width,
        1,
    );
    f.render_widget(danger_p, danger_area);

    // Draw each falling word
    for word in &state.words {
        let row = word.row.min(height.saturating_sub(1));
        let col = word.col.min(inner.width.saturating_sub(1));
        let word_area = Rect::new(inner.x + col, inner.y + row, word.text.len() as u16, 1);
        if word_area.x + word_area.width <= inner.x + inner.width
            && word_area.y < inner.y + inner.height
        {
            let style = if row >= danger_row {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            // Highlight matched prefix
            let input = state.input.trim();
            let spans = if !input.is_empty() && word.text.starts_with(input) {
                vec![
                    Span::styled(
                        input.to_string(),
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(word.text[input.len()..].to_string(), style),
                ]
            } else {
                vec![Span::styled(word.text.clone(), style)]
            };
            f.render_widget(Paragraph::new(Line::from(spans)), word_area);
        }
    }
}
