use std::time::Instant;
use std::hint::black_box;

const BUFFER_SIZE: usize = 1024;
const NUM_SAMPLES: usize = 5_000;// Collect 5000 samples

fn create_on_stack(){
    let buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE]; // Allocate buffer on the stack
    black_box(buffer); // Prevent optimization
}

fn create_on_heap(){
    let buffer: Vec<u8> = Vec::with_capacity(BUFFER_SIZE); // Allocate buffer on the heap
    black_box(buffer); // Prevent optimization
}



fn main() {
    let mut stack_samples: Vec<u128> = Vec::with_capacity(NUM_SAMPLES);
    let mut heap_samples: Vec<u128> = Vec::with_capacity(NUM_SAMPLES);

    println!("Measuring stack allocation...");
    for _ in 0..NUM_SAMPLES {
        let start = Instant::now();
        create_on_stack();
        let duration = start.elapsed();
        stack_samples.push(duration.as_nanos());
    }

    println!("Measuring heap allocation...");
    for _ in 0..NUM_SAMPLES {
        let start = Instant::now();
        create_on_heap();
        let duration = start.elapsed();
        heap_samples.push(duration.as_nanos());
    }

    println!("\n--- Stack Allocation Stats ---");
    print_stats(&stack_samples);

    println!("\n--- Heap Allocation Stats ---");
    print_stats(&heap_samples);
}

fn print_stats(samples: &[u128]) {
    if samples.is_empty(){return; }

    let min: &u128 = samples.iter().min().unwrap_or(&0);
    let max: &u128 = samples.iter().max().unwrap_or(&0);
    let sum: u128 = samples.iter().sum();
    let avg: f64 = sum as f64 / samples.len() as f64;

    let jitter: u128 = max - min;

    println!("Samples: {}", samples.len());
    println!("Min: {} ns", min);
    println!("Max: {} ns (Observed WCET)", max);
    println!("Average: {:.2} ns", avg);
    println!("Jitter: {} ns (Max - Min)", jitter);
}