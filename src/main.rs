use std::env::args;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::{spawn, Thread};
use std::time::Instant;

fn main() {
    let threads = args().nth(1).unwrap().parse::<usize>().unwrap();

    let cpus = num_cpus::get();
    eprintln!("Number of CPUs: {}", cpus);

    let start = Instant::now();

    let mut joins = Vec::new();

    let sink = Arc::new(AtomicUsize::new(0));

    for i in 0..threads {
        let sink = sink.clone();
        joins.push(spawn(move || {
            let thread_start = Instant::now();
            for i in 0..1_000_000_000 {
                sink.store((i as f32).log(7f32) as usize, Ordering::Relaxed);
            }
            let thread_end = Instant::now();
            eprintln!("Thread {} finished in {} ms", i, thread_end.duration_since(thread_start).as_millis());
        }));
    }

    for join in joins {
        join.join().unwrap();
    }

    let main_end = Instant::now();
    eprintln!("Main finished in {} ms", main_end.duration_since(start).as_millis());
    eprintln!("Sink: {}", sink.load(Ordering::Relaxed));
}
