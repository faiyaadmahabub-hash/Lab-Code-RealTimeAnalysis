use criterion::{criterion_group, criterion_main, Criterion, black_box};
use std::hint;

const BUFFER_SIZE: usize = 1024;

fn create_on_stack() {
    let buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE]; // Allocate buffer on the stack
    black_box(buffer); // Prevent optimization
}

fn create_on_heap() {
    let buffer: Vec<u8> = Vec::with_capacity(BUFFER_SIZE); // Allocate buffer on the heap
    black_box(buffer); // Prevent optimization
}

fn push_to_vec() {
    let mut vec = Vec::new();
    vec.push(1);
    black_box(vec);
}

fn benchmark_allocations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Stack vs Heap");

    group.bench_function("Stack", |b| b.iter(|| create_on_stack()));

    group.bench_function("Heap", |b| b.iter(|| create_on_heap()));

    group.bench_function("Push to Vec", |b| b.iter(|| push_to_vec()));

    group.finish();

}

criterion_group!(benches, benchmark_allocations);
criterion_main!(benches);