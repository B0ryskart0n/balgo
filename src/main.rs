use std::hint::black_box;
use std::time::Instant;

const N: usize = 10000;

fn main() {
    let mut elapsed = Vec::from([0.0; N]);
    let mut now;

    for i in 0..N {
        now = Instant::now();
        black_box(());
        elapsed[i] = now.elapsed().as_secs_f64();
    }
    print_elapsed_stats("I", &elapsed);
    for i in 0..N {
        now = Instant::now();
        black_box(());
        elapsed[i] = now.elapsed().as_secs_f64();
    }
    print_elapsed_stats("II", &elapsed);
}

fn print_elapsed_stats(id: &str, elapsed: &Vec<f64>) {
    println!("{}:\t{:.3e}", id, elapsed.iter().sum::<f64>() / N as f64);
}
