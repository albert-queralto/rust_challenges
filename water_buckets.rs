
// https://medium.com/rustaceans/rust-challenge-two-bucket-d8991c76a6bb
use std::collections::{HashSet, VecDeque};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct State {
    bucket1: i32,
    bucket2: i32,
    actions: i32,
}

fn measure_water(bucket1_size: i32, bucket2_size: i32, target: i32, start_with_bucket1: bool) -> Option<(i32, i32, i32)> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    let initial_state = if start_with_bucket1 {
        State { bucket1: bucket1_size, bucket2: 0, actions: 1 }
    } else {
        State { bucket1: 0, bucket2: bucket2_size, actions: 1 }
    };

    queue.push_back(initial_state);
    visited.insert((initial_state.bucket1, initial_state.bucket2));

    while let Some(current) = queue.pop_front() {
        if current.bucket1 == target || current.bucket2 == target {
            return Some((current.actions, current.bucket1, current.bucket2));
        }

        let next_states = vec![
            State { bucket1: 0, bucket2: current.bucket2, actions: current.actions + 1 },
            State { bucket1: current.bucket1, bucket2: 0, actions: current.actions + 1 },
            State { bucket1: bucket1_size, bucket2: current.bucket2, actions: current.actions + 1 },
            State { bucket1: current.bucket1, bucket2: bucket2_size, actions: current.actions + 1 },
            State {
                bucket1: (current.bucket1 + current.bucket2).min(bucket1_size),
                bucket2: (current.bucket1 + current.bucket2 - bucket1_size).max(0),
                actions: current.actions + 1,
            },
            State {
                bucket1: (current.bucket1 + current.bucket2 - bucket2_size).max(0),
                bucket2: (current.bucket1 + current.bucket2).min(bucket2_size),
                actions: current.actions + 1,
            },
        ];

        for next_state in next_states {
            if visited.insert((next_state.bucket1, next_state.bucket2)) {
                queue.push_back(next_state);
            }
        }
    }

    None
}

fn main() {
    let bucket1_size = 7;
    let bucket2_size = 11;
    let target = 6;
    let start_with_bucket1 = true;

    match measure_water(bucket1_size, bucket2_size, target, start_with_bucket1) {
        Some((actions, bucket1, bucket2)) => {
            println!("Total actions: {}", actions);
            println!("Bucket 1: {} liters", bucket1);
            println!("Bucket 2: {} liters", bucket2);
        }
        None => println!("No solution found"),
    }
}