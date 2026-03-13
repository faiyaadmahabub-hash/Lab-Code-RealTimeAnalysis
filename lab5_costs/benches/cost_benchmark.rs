use criterion::{criterion_group, criterion_main, Criterion};
use std::panic;
use std::hint::black_box;

fn simple_function_ok() -> Result<u32, &'static str> {
    Ok(42)
}

fn simple_function_err() -> Result<u32, &'static str> {
    Err("This is a predictable error")
}

fn simple_function_panic() {
    panic!("This is a catrastrophic panic");
}

fn benchmark_errors(c: &mut Criterion) {
    let mut group = c.benchmark_group("Error Handling Benchmarks");

    group.bench_function("Ok Result", |b| {
        b.iter(|| {
            let val = simple_function_ok().unwrap();
            black_box(val);
        })
    });

    group.bench_function("Err Result", |b| {
        b.iter(|| {
            if let Err(e) = simple_function_err() {
                black_box(e);
            }
        })
    });

    group.bench_function("Panic", |b| {
        b.iter(|| {
            let result = panic::catch_unwind(|| {
                simple_function_panic();
            });
            black_box(result);
        })
    });
    group.finish();
}

trait Processor {
    fn process(&self, val: u32) -> u32;
}

struct Adder;

impl Processor for Adder {
    fn process(&self, val: u32) -> u32 {
        val + 1
    }
}

fn run_static <T: Processor>(processor: &T, val: u32) -> u32 {
    processor.process(val)
}

fn run_dynamic(proc: &dyn Processor, val: u32) -> u32 {
    proc.process(val)
}

fn benchmark_dispatch(c: &mut Criterion){
    let mut group = c.benchmark_group("Dispatch");
    let adder = Adder;
    let dyn_adder: &dyn Processor = &adder;

    group.bench_function("Static (Generic)",|b| {
        b.iter(|| {
           black_box(run_static(black_box(&adder), black_box(10)));
        })
    });

    group.bench_function("Dynamic (dyn Trait)", |b| {
        b.iter(|| {
            black_box(run_dynamic(black_box(&adder), black_box(10)));
        })
    });

    group.finish();
}
criterion_group!(benches, benchmark_errors, benchmark_dispatch);
criterion_main!(benches);

