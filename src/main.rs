use std::time::{Duration, Instant};

use rubiks_cube::{self, Cube, Operation};

fn operation_vec_to_string(operations: Vec<Operation>) -> String {
    return operations
        .iter()
        .map(|op| op.to_string())
        .collect::<Vec<_>>()
        .join(" ");
}

fn main() {
    let num_trials = 30;
    let num_shuffles = 6;
    let search_depth = 6;
    let mut total_time = Duration::ZERO;

    for i in 0..num_trials {
        println!("Trial #{}/{num_trials}", i + 1);

        let (cube, applied) = Cube::random(num_shuffles);
        println!("Applied rotation: {}", operation_vec_to_string(applied));

        let start_time = Instant::now();
        let result = rubiks_cube::solve(cube, search_depth);
        let process_time = start_time.elapsed();

        if let Some(result) = result {
            print!("Found: {}", operation_vec_to_string(result));
        } else {
            print!("Not found")
        }
        println!(" ({:.2} secs)", process_time.as_secs_f64());

        total_time += process_time
    }

    println!(
        "Average time: {:.2} secs",
        total_time.as_secs_f64() / (num_trials as f64)
    );
}
