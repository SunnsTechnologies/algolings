//! Same dispatcher as exercises/linked-list/src/bin/trace.rs, run against
//! the reference solutions instead of the learner-facing skeletons.

use algolings_trace::{enable, take_events};
use linked_list_solutions::{DoublyLinkedList, SinglyLinkedList};

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
        "insert" => {
            let mut list = SinglyLinkedList::new();
            enable();
            for value in fixture {
                list.push_back(value);
            }
        }
        "remove" => {
            let target = parse_target(&mut args);
            let mut list = SinglyLinkedList::from_values(&fixture);
            enable();
            list.remove(target);
        }
        "traverse" => {
            let target = parse_target(&mut args);
            let list = SinglyLinkedList::from_values(&fixture);
            enable();
            list.contains(target);
        }
        "reverse" => {
            let mut list = SinglyLinkedList::from_values(&fixture);
            enable();
            list.reverse();
        }
        "doubly_push" => {
            let mut list = DoublyLinkedList::new();
            enable();
            for value in fixture {
                list.push_back(value);
            }
        }
        "doubly_pop" => {
            let mut list = DoublyLinkedList::from_values(&fixture);
            enable();
            list.pop_front();
            list.pop_back();
        }
        "doubly_contains" => {
            let target = parse_target(&mut args);
            let list = DoublyLinkedList::from_values(&fixture);
            enable();
            list.contains(target);
        }
        "doubly_converge" => {
            let target = parse_target(&mut args);
            let list = DoublyLinkedList::from_values(&fixture);
            enable();
            list.contains_converging(target);
        }
        "floyds_cycle_detection" => {
            let cycle_to_index = args.next().and_then(|s| s.parse::<usize>().ok());
            let list = DoublyLinkedList::from_values_with_cycle(&fixture, cycle_to_index);
            enable();
            list.has_cycle();
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

fn parse_target(args: &mut impl Iterator<Item = String>) -> i32 {
    args.next()
        .unwrap_or_else(|| usage_error("missing target"))
        .parse()
        .unwrap_or_else(|e: std::num::ParseIntError| usage_error(&e.to_string()))
}

fn usage_error(message: &str) -> ! {
    eprintln!("usage: trace <exercise-name> <fixture-json> [target]\n{message}");
    std::process::exit(2);
}
