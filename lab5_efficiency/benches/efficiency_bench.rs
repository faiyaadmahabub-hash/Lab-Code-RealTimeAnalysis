use criterion::{black_box,criterion_group, criterion_main, Criterion};
use rand::prelude::*;

fn get_random_data() -> Vec<i64> {
    let mut rng = rand::thread_rng();
    (0..1_000_000).map(|_| rng.gen_range(0..100)).collect()
}

fn sum_loop(data: &[i64]) -> i64 {
    let mut sum = 0;
    for i in 0..data.len() {
        sum += data[i];
    }
    sum
}

fn sum_iter(data: &[i64]) -> i64 {
    data.iter().sum()
}

fn bench_zca(c: &mut Criterion) {
    let data = get_random_data();
    
    let mut group = c.benchmark_group("Zero Cost Abstraction");



    group.bench_function("C-Style Loop", |b| b.iter(|| sum_loop(black_box(&data))));
    group.bench_function("Iterator", |b| b.iter(|| sum_iter(black_box(&data))));

    group.finish();
}


struct Particle{
    x:f64, 
    y:f64,
    z:f64,
    vx:f64,
    vy:f64,
    vz:f64,

    pad:[u64; 4],
}

struct Particles{
    vx: Vec<f64>,
    vy: Vec<f64>,
    vz: Vec<f64>,
}

fn process_aos(particles: &mut [Particle]) {
    for p in particles {
        // Simulate some processing
        p.vx += 1.0;
        p.vy += 1.0;
        p.vz += 1.0;
    }
}

fn process_soa(particles: &mut Particles) {
    for i in 0..particles.vx.len() {
        // Simulate some processing
        particles.vx[i] += 1.0;
        particles.vy[i] += 1.0;
        particles.vz[i] += 1.0;
    }
}

fn bench_dod(c: &mut Criterion) {
    let mut group = c.benchmark_group("Data Layout");
    let count = 1_000_000;

    let mut aos = Vec::with_capacity(count);
    for _ in 0..count {
        aos.push(Particle {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            pad: [0; 4],
        });
    }


    let mut soa = Particles {
        vx: vec![0.0; count],
        vy: vec![0.0; count],
        vz: vec![0.0; count],
    };

    group.bench_function("AoS (Object Oriented)", |b| b.iter(|| process_aos(black_box(&mut aos))));
    group.bench_function("SoA (Data Oriented)", |b| b.iter(|| process_soa(black_box(&mut soa))));

    group.finish();
}

criterion_group!(benches, bench_zca, bench_dod);
criterion_main!(benches);
