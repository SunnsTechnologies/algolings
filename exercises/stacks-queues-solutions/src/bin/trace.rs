//! Same dispatcher as exercises/stacks-queues/src/bin/trace.rs, run against
//! the reference solutions instead of the learner-facing skeletons.

use algolings_trace::{enable, take_events};
use stacks_queues_solutions::{LinkedQueue, LinkedStack, VecQueue, VecStack};

fn main() {
    let mut args = std::env::args().skip(1);
    let name = args
        .next()
        .unwrap_or_else(|| usage_error("missing exercise name"));
    let fixture_json = args
        .next()
        .unwrap_or_else(|| usage_error("missing fixture JSON"));
    let fixture: Vec<i32> =
        serde_json::from_str(&fixture_json).unwrap_or_else(|e| usage_error(&e.to_string()));

    match name.as_str() {
        "stack_vec" => {
            let mut stack = VecStack::new();
            enable();
            for value in fixture {
                stack.push(value);
            }
            let _ = stack.peek();
            stack.pop();
        }
        "stack_linked_list" => {
            let mut stack = LinkedStack::new();
            enable();
            for value in fixture {
                stack.push(value);
            }
            let _ = stack.peek();
            stack.pop();
        }
        "queue_vecdeque" => {
            let mut queue = VecQueue::new();
            enable();
            for value in fixture {
                queue.enqueue(value);
            }
            let _ = queue.peek();
            queue.dequeue();
        }
        "queue_linked_list" => {
            let mut queue = LinkedQueue::new();
            enable();
            for value in fixture {
                queue.enqueue(value);
            }
            let _ = queue.peek();
            queue.dequeue();
        }
        other => {
            eprintln!("unknown exercise: {other}");
            std::process::exit(2);
        }
    }
    let events = take_events();

    println!(
        "{}",
        serde_json::to_string(&events).expect("events always serialize")
    );
}

fn usage_error(message: &str) -> ! {
    eprintln!("usage: trace <exercise-name> <fixture-json>\n{message}");
    std::process::exit(2);
}
