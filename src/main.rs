use balgo::*;
use std::collections::HashMap;
use std::hint::black_box;
use std::time::Instant;

const N: usize = 100000;

fn main() {
    let mut graph: HashMap<i32, Vec<(i32, u32)>> = HashMap::new();
    let num_nodes = 150;
    let start_node = 0;
    let goal_node = num_nodes - 1; // 149

    let mut golden_path_nodes = vec![start_node];
    let mut current_node = start_node;
    while current_node < 135 {
        graph
            .entry(current_node)
            .or_insert_with(Vec::new)
            .push((current_node + 15, 1));
        current_node += 15;
        golden_path_nodes.push(current_node);
    }
    graph
        .entry(135)
        .or_insert_with(Vec::new)
        .push((goal_node, 1));
    golden_path_nodes.push(goal_node);

    for i in 0..num_nodes {
        graph.entry(i).or_insert_with(Vec::new);

        for j in 1..=3 {
            if i + j < num_nodes {
                let cost = (j * 4) as u32; // e.g., cost to i+1 is 4, i+2 is 8, etc.
                graph.entry(i).or_insert_with(Vec::new).push((i + j, cost));
            }
        }
    }

    graph
        .entry(start_node)
        .or_insert_with(Vec::new)
        .push((goal_node - 5, 100));

    graph.entry(0).or_insert_with(Vec::new).push((1, 1));
    graph.entry(1).or_insert_with(Vec::new).push((2, 1));
    graph.entry(2).or_insert_with(Vec::new).push((20, 50));
    graph.entry(2).or_insert_with(Vec::new).push((30, 50));

    for i in 0..num_nodes {
        let target1 = (i + 37) % num_nodes;
        let target2 = (i * 3) % num_nodes;

        if target1 != i {
            graph.entry(i).or_insert_with(Vec::new).push((target1, 25));
        }
        if target2 != i {
            graph.entry(i).or_insert_with(Vec::new).push((target2, 35));
        }
    }

    graph.insert(goal_node, vec![]);

    let mut elapsed = Vec::from([0.0; N]);
    let mut now;
    for i in 0..N {
        now = Instant::now();
        black_box(a_star(
            black_box(&graph),
            black_box(start_node),
            black_box(goal_node),
        ));
        elapsed[i] = now.elapsed().as_secs_f64();
    }
    print_elapsed_stats("I", &elapsed);
    for i in 0..N {
        now = Instant::now();
        black_box(a_star2(
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
