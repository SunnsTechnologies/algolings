//! Pure step-through state machine for the trace visualization — decoupled
//! from actual terminal I/O so the stepping logic is unit-testable without
//! a real TTY. `main.rs`'s interactive loop (or the `ratatui` renderer)
//! owns reading keypresses and drawing; this just tracks "which step are we
//! on" and computes what that step looks like.

use crate::trace_frame::apply_event;
use algolings_trace::Event;

pub struct Frame {
    pub array: Vec<i32>,
    pub highlighted: Vec<usize>,
    pub description: String,
    pub step: usize,
    pub total_steps: usize,
}

pub struct TracePlayer {
    fixture: Vec<i32>,
    events: Vec<Event>,
    step: usize,
}

impl TracePlayer {
    pub fn new(fixture: &[i32], events: Vec<Event>) -> Self {
        Self {
            fixture: fixture.to_vec(),
            events,
            step: 0,
        }
    }

    /// Advances one step. Returns `false` (and does nothing) if already at
    /// the last event.
    pub fn advance(&mut self) -> bool {
        if self.step < self.events.len() {
            self.step += 1;
            true
        } else {
            false
        }
    }

    pub fn is_finished(&self) -> bool {
        self.step >= self.events.len()
    }

    /// Recomputes the frame at the current step by replaying events from
    /// the start. Trace lengths are tiny (bounded by the ≤20-element
    /// fixture cap) so re-deriving instead of caching snapshots is simple
    /// and plenty fast.
    pub fn current_frame(&self) -> Frame {
        let mut array = self.fixture.clone();
        let mut highlighted = Vec::new();
        let mut description = String::new();

        for event in &self.events[..self.step] {
            let (h, d) = apply_event(&mut array, event);
            highlighted = h;
            description = d;
        }

        Frame {
            array,
            highlighted,
            description,
            step: self.step,
            total_steps: self.events.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::Event;

    #[test]
    fn fresh_player_shows_the_initial_array_at_step_zero() {
        let player = TracePlayer::new(&[5, 1, 4], vec![]);
        let frame = player.current_frame();
        assert_eq!(frame.array, vec![5, 1, 4]);
        assert_eq!(frame.step, 0);
        assert_eq!(frame.total_steps, 0);
        assert!(frame.highlighted.is_empty());
        assert!(player.is_finished());
    }

    #[test]
    fn advance_applies_one_event_at_a_time() {
        let events = vec![
            Event::Compare { i: 0, j: 1 },
            Event::Swap { i: 0, j: 1 },
        ];
        let mut player = TracePlayer::new(&[5, 1], events);
        assert!(!player.is_finished());

        assert!(player.advance());
        let frame = player.current_frame();
        assert_eq!(frame.array, vec![5, 1]); // compare doesn't move values
        assert_eq!(frame.highlighted, vec![0, 1]);
        assert_eq!(frame.step, 1);
        assert_eq!(frame.total_steps, 2);
        assert!(!player.is_finished());

        assert!(player.advance());
        let frame = player.current_frame();
        assert_eq!(frame.array, vec![1, 5]); // now swapped
        assert!(player.is_finished());
    }

    #[test]
    fn advance_past_the_end_returns_false_and_stays_on_the_final_frame() {
        let mut player = TracePlayer::new(&[5, 1], vec![Event::Swap { i: 0, j: 1 }]);
        assert!(player.advance());
        assert!(!player.advance(), "should not advance past the last event");
        assert_eq!(player.current_frame().array, vec![1, 5]);
    }
}
