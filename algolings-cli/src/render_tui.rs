//! Interactive `ratatui` trace renderer. `draw_frame` (the pure "what goes
//! on screen" logic) is tested with `TestBackend`, no real terminal needed.
//! The live keyboard event loop (`run_interactive`) touches a real TTY and
//! is deliberately kept thin, verified by manually running the CLI instead.

use crate::player::{Frame as PlayerFrame, TracePlayer};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

/// Visual language from the design review: plain ASCII brackets/pipes,
/// standard ANSI color, and a `*` marker glyph alongside color for
/// highlighted positions — never color alone, so it stays legible for
/// colorblind users.
pub fn draw_frame(
    f: &mut Frame,
    frame: &PlayerFrame,
    exercise_name: &str,
    auto_play: bool,
    target: Option<i32>,
) {
    let array_line = render_array_line(&frame.array, &frame.highlighted);
    let status_line = match target {
        Some(t) => format!(
            "{exercise_name} — target {t} — Step {} of {}",
            frame.step, frame.total_steps
        ),
        None => format!(
            "{exercise_name} — Step {} of {}",
            frame.step, frame.total_steps
        ),
    };
    let controls = if auto_play {
        "[space] step  [a] pause auto-play  [q] quit"
    } else {
        "[space] step  [a] auto-play  [q] quit"
    };

    let chunks = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .split(f.area());

    f.render_widget(Paragraph::new(array_line), chunks[0]);
    f.render_widget(Paragraph::new(status_line), chunks[1]);
    f.render_widget(Paragraph::new(frame.description.clone()), chunks[2]);
    f.render_widget(Paragraph::new(controls), chunks[3]);
}

fn render_array_line(arr: &[i32], highlighted: &[usize]) -> Line<'static> {
    let mut spans = vec![Span::raw("[ ")];
    for (idx, value) in arr.iter().enumerate() {
        if idx > 0 {
            spans.push(Span::raw(" | "));
        }
        if highlighted.contains(&idx) {
            spans.push(Span::styled(
                format!("{value}*"),
                Style::default().fg(Color::Yellow),
            ));
        } else {
            spans.push(Span::raw(value.to_string()));
        }
    }
    spans.push(Span::raw(" ]"));
    Line::from(spans)
}

/// Runs the interactive step-through/auto-play trace view in a real
/// terminal. Blocks until the user presses `q`. Not unit-tested (it reads
/// real keyboard input and owns the real terminal) — verified by manually
/// running the CLI end to end.
pub fn run_interactive(
    fixture: &[i32],
    events: Vec<algolings_trace::Event>,
    exercise_name: &str,
    target: Option<i32>,
) -> std::io::Result<()> {
    use crossterm::event::{self, Event as CEvent, KeyCode};
    use std::time::Duration;

    let mut terminal = ratatui::init();
    let mut player = TracePlayer::new(fixture, events);
    let mut auto_play = false;

    let result = (|| -> std::io::Result<()> {
        loop {
            terminal
                .draw(|f| draw_frame(f, &player.current_frame(), exercise_name, auto_play, target))?;

            if auto_play {
                if event::poll(Duration::from_millis(500))? {
                    if let CEvent::Key(key) = event::read()? {
                        match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Char('a') => auto_play = false,
                            KeyCode::Char(' ') => {
                                player.advance();
                                auto_play = false;
                            }
                            _ => {}
                        }
                    }
                } else if !player.advance() {
                    auto_play = false;
                }
            } else if let CEvent::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('a') => auto_play = true,
                    KeyCode::Char(' ') => {
                        player.advance();
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    })();

    ratatui::restore();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::buffer::Buffer;
    use ratatui::Terminal;

    fn frame(array: Vec<i32>, highlighted: Vec<usize>, step: usize, total_steps: usize) -> PlayerFrame {
        PlayerFrame {
            array,
            highlighted,
            description: String::new(),
            step,
            total_steps,
        }
    }

    #[test]
    fn draws_the_array_with_asterisk_on_highlighted_positions() {
        let backend = TestBackend::new(40, 4);
        let mut terminal = Terminal::new(backend).unwrap();
        let f = frame(vec![5, 1, 4], vec![0, 1], 1, 2);

        terminal
            .draw(|frm| draw_frame(frm, &f, "bubble_sort", false, None))
            .unwrap();

        let content = buffer_text(terminal.backend().buffer());
        assert!(content.contains("[ 5* | 1* | 4 ]"));
    }

    #[test]
    fn draws_step_counter_and_controls() {
        let backend = TestBackend::new(60, 4);
        let mut terminal = Terminal::new(backend).unwrap();
        let f = frame(vec![5, 1], vec![], 3, 8);

        terminal
            .draw(|frm| draw_frame(frm, &f, "bubble_sort", false, None))
            .unwrap();

        let content = buffer_text(terminal.backend().buffer());
        assert!(content.contains("Step 3 of 8"));
        assert!(content.contains("[space] step"));
        assert!(content.contains("[a] auto-play"));
        assert!(content.contains("[q] quit"));
    }

    #[test]
    fn controls_reflect_auto_play_state() {
        let backend = TestBackend::new(60, 4);
        let mut terminal = Terminal::new(backend).unwrap();
        let f = frame(vec![5, 1], vec![], 0, 2);

        terminal
            .draw(|frm| draw_frame(frm, &f, "bubble_sort", true, None))
            .unwrap();

        let content = buffer_text(terminal.backend().buffer());
        assert!(content.contains("[a] pause auto-play"));
    }

    #[test]
    fn status_line_is_unchanged_when_there_is_no_target_regression_guard() {
        // Sort exercises pass target: None — this must render byte-identical
        // to the status line before target support existed.
        let backend = TestBackend::new(60, 4);
        let mut terminal = Terminal::new(backend).unwrap();
        let f = frame(vec![5, 1], vec![], 3, 8);

        terminal
            .draw(|frm| draw_frame(frm, &f, "bubble_sort", false, None))
            .unwrap();

        let content = buffer_text(terminal.backend().buffer());
        assert!(content.contains("bubble_sort — Step 3 of 8"));
        assert!(!content.contains("target"));
    }

    #[test]
    fn status_line_shows_the_target_when_present() {
        let backend = TestBackend::new(60, 4);
        let mut terminal = Terminal::new(backend).unwrap();
        let f = frame(vec![3, 7, 2, 9, 5], vec![3], 1, 4);

        terminal
            .draw(|frm| draw_frame(frm, &f, "linear_search", false, Some(9)))
            .unwrap();

        let content = buffer_text(terminal.backend().buffer());
        assert!(content.contains("target 9"));
        assert!(content.contains("linear_search"));
    }

    #[test]
    fn description_line_shows_the_current_step_description() {
        let backend = TestBackend::new(60, 4);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut f = frame(vec![3, 7, 2, 9, 5], vec![3], 1, 4);
        f.description = "check [3]".to_string();

        terminal
            .draw(|frm| draw_frame(frm, &f, "linear_search", false, Some(9)))
            .unwrap();

        let content = buffer_text(terminal.backend().buffer());
        assert!(content.contains("check [3]"));
    }

    fn buffer_text(buffer: &Buffer) -> String {
        let area = buffer.area();
        let mut text = String::new();
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                text.push_str(buffer[(x, y)].symbol());
            }
            text.push('\n');
        }
        text
    }
}
