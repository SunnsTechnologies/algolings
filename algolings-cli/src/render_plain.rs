//! `--plain` sequential-text trace rendering (design review's Accessibility
//! pass): one line per event, no cursor repositioning, no color — readable
//! by a screen reader or captured by a script, unlike the redrawing
//! `ratatui` TUI. Uses the same visual language (ASCII brackets/pipes, `*`
//! marker for highlighted positions) minus the color, since there's none
//! here to fall back from.

use crate::trace_frame::apply_event;
use algolings_trace::Event;

/// Replays `events` against `fixture`, returning one line per step (plus
/// the initial state) describing what happened and the array's state
/// afterward.
pub fn render_plain(fixture: &[i32], events: &[Event]) -> String {
    let mut arr = fixture.to_vec();
    let mut lines = vec![format_array(&arr, &[])];

    for event in events {
        let (highlighted, description) = apply_event(&mut arr, event);
        lines.push(format!("{description} -> {}", format_array(&arr, &highlighted)));
    }

    lines.join("\n")
}

fn format_array(arr: &[i32], highlighted: &[usize]) -> String {
    let parts: Vec<String> = arr
        .iter()
        .enumerate()
        .map(|(idx, value)| {
            if highlighted.contains(&idx) {
                format!("{value}*")
            } else {
                value.to_string()
            }
        })
        .collect();
    format!("[ {} ]", parts.join(" | "))
}

#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::Event;

    #[test]
    fn no_events_shows_just_the_initial_array() {
        let output = render_plain(&[5, 1, 4], &[]);
        assert_eq!(output, "[ 5 | 1 | 4 ]");
    }

    #[test]
    fn compare_event_marks_both_compared_positions_with_an_asterisk() {
        let output = render_plain(&[5, 1, 4], &[Event::Compare { i: 0, j: 1 }]);
        assert!(output.contains("[ 5* | 1* | 4 ]"));
    }

    #[test]
    fn swap_event_actually_reorders_the_displayed_array() {
        let output = render_plain(&[5, 1], &[Event::Swap { i: 0, j: 1 }]);
        assert!(output.contains("[ 1* | 5* ]"));
    }

    #[test]
    fn set_event_updates_the_value_at_that_position() {
        let output = render_plain(&[5, 1], &[Event::Set { i: 0, value: 9 }]);
        assert!(output.contains("[ 9* | 1 ]"));
    }

    #[test]
    fn mark_sorted_event_does_not_change_values_and_still_renders_a_line() {
        let output = render_plain(&[5, 1], &[Event::MarkSorted { i: 1 }]);
        assert!(output.contains("[ 5 | 1 ]"));
        assert!(output.to_lowercase().contains("sorted"));
    }

    #[test]
    fn never_uses_color_only_every_highlight_has_the_asterisk_marker() {
        // Accessibility requirement from the design review: color-only
        // highlighting fails for colorblind users, so the plain-text
        // fallback (which has no color at all) must rely entirely on the
        // `*` marker — this test is really just documenting/enforcing that
        // every highlighted event type above produces a `*`, not color
        // codes, which is trivially true for a plain-text renderer but
        // worth asserting explicitly since it's a real requirement.
        let output = render_plain(&[5, 1], &[Event::Compare { i: 0, j: 1 }]);
        assert!(!output.contains("\x1b[")); // no ANSI escape codes at all
        assert!(output.contains('*'));
    }
}
