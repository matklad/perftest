use core::panic;
use std::collections::HashSet;

fn main() {
    let start = std::time::Instant::now();

    let mut counter = perf_event::Builder::new()
        .build()
        .unwrap_or_else(|e| panic!("build: {}", e));
    counter.enable().unwrap_or_else(|e| panic!("enable: {}", e));

    let work = (0..5_000_000u32)
        .map(|i| i.wrapping_mul(i) ^ i)
        .collect::<HashSet<_>>();

    let count = counter.read().unwrap_or_else(|e| panic!("enable: {}", e));

    eprintln!("{} hash", work.len());
    eprintln!("{} instructions", count);
    eprintln!("{:.2?}", start.elapsed());
}
