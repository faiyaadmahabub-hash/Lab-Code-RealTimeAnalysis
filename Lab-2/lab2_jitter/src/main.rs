use std::time::{Duration, Instant};
use std::thread;

//Set constants for how long we "want" to sleep
const SLEEP_DURATION: Duration = Duration::from_millis(10);
const NUM_SAMPLES: u32 = 100;

fn main() {
    let mut jitter_sample: Vec<u128> = Vec::with_capacity(NUM_SAMPLES as usize);

    println!("---Measuring Jitter for thread::sleep()---");
    println!("--- Running {} samples ---", NUM_SAMPLES);

    // --- Add This Code ---
    println!("Spawning 4 'noisy' threads to create CPU load ---");
    for i in 0..4 {
        let id = i;
        thread::spawn(move || {
            // This is a "busy-wait" loop where it does nothing but consumes the CPU.
            println!("Noisy thread {} starting...", id);
            loop {
                // Spin
            }
        });
    }

    // Give the threads a moment to start
    thread::sleep(Duration::from_millis(100));
    // --- End Added Code ---

    let start_time: Instant = Instant::now();

    for i in 0..NUM_SAMPLES{
        // Calculate exactly when we *should* wake up
        let expected_wakeup: Instant = start_time + SLEEP_DURATION * (i + 1);

        //Go to sleep for the required duration which asks the OS to wake up
        thread::sleep(SLEEP_DURATION);

        // We are awake! Measure the actual time.
        let actual_wakeup: Instant = Instant::now();

        // Calculate the error, or "jitter" where we use saturation_sub to avoid panic if we somehow woke up early.
        let jitter: Duration = actual_wakeup.duration_since(expected_wakeup).saturating_sub(SLEEP_DURATION);
        
        // Store the jitter in microseconds
        jitter_sample.push(jitter.as_micros());
    }

    println!("--- Done --");
    println!("Jitter samples (in microseconds)");
    println!("{:?}", jitter_sample);

    // Find the max jitter
    if let Some(max_jitter) = jitter_sample.iter().max() {
        println!("Max Jitter Observed: {} us ({} ms)",
            max_jitter,
            *max_jitter as f64 / 1000.0
        );
    }
}

