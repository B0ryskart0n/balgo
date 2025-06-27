use balgo::*;
use std::collections::HashMap;
use std::hint::black_box;
use std::time::Instant;

const N: usize = 100000;

fn main() {
    let mut elapsed = Vec::from([0.0; N]);
    let mut now;

    let mut graph = HashMap::new();
    graph.insert('A', vec![('B', 1), ('C', 3)]);
    graph.insert('B', vec![('D', 3), ('C', 1)]);
    graph.insert('C', vec![('D', 1)]);
    graph.insert('D', vec![]);

    let start_node = 'A';
    let goal_node = 'D';

    for i in 0..N {
        now = Instant::now();
        black_box(a_star::<char, u32>(
            black_box(&graph),
            black_box(start_node),
            black_box(goal_node),
        ));
        elapsed[i] = now.elapsed().as_secs_f64();
    }
    print_elapsed_stats("I", &elapsed);
    for i in 0..N {
        now = Instant::now();
        black_box(a_star2::<char, u32>(
            black_box(&graph),
            black_box(start_node),
            black_box(goal_node),
        ));
        elapsed[i] = now.elapsed().as_secs_f64();
    }
    print_elapsed_stats("II", &elapsed);
}

fn print_elapsed_stats(id: &str, elapsed: &Vec<f64>) {
    println!("{}:\t{:.3e}", id, elapsed.iter().sum::<f64>() / N as f64);
}
