use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: bench_parse <archive> [iterations]");
        std::process::exit(2);
    }
    let path = &args[1];
    let iterations: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(10);

    println!("Running parse benchmark on {} for {} iterations", path, iterations);
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = big_core::parse_archive(path);
    }
    let dur = start.elapsed();
    println!("Total: {:?}, avg per run: {:?}", dur, dur / (iterations as u32));
}
