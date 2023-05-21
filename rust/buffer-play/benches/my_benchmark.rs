use core::time::Duration;
use criterion::{criterion_group, criterion_main, Criterion};
use std::{slice::ChunksMut, thread};

pub fn small_test_single() {
    let mut buffer = vec![0u64; 1024];
    let buffer_len = buffer.len();

    let num_tasks = 1;

    let chunk_size = buffer_len / num_tasks;

    let chunked_data = buffer.chunks_mut(chunk_size);

    multi_write_buffers(chunked_data, chunk_size);
}

pub fn small_test_threaded() {
    let mut buffer = vec![0u64; 1024];
    let buffer_len = buffer.len();

    let num_tasks = num_cpus::get();

    let chunk_size = buffer_len / num_tasks;

    let chunked_data = buffer.chunks_mut(chunk_size);

    multi_write_buffers(chunked_data, chunk_size);
}

pub fn med_test_single() {
    let mut buffer = vec![0u64; 1024000];
    let buffer_len = buffer.len();

    let num_tasks = 1;

    let chunk_size = buffer_len / num_tasks;

    let chunked_data = buffer.chunks_mut(chunk_size);

    multi_write_buffers(chunked_data, chunk_size);
}

pub fn med_test_threaded() {
    let mut buffer = vec![0u64; 1024000];
    let buffer_len = buffer.len();

    let num_tasks = num_cpus::get();

    let chunk_size = buffer_len / num_tasks;

    let chunked_data = buffer.chunks_mut(chunk_size);

    multi_write_buffers(chunked_data, chunk_size);
}

pub fn large_test_single() {
    let mut buffer = vec![0u64; 1024000000];
    let buffer_len = buffer.len();

    let num_tasks = 1;

    let chunk_size = buffer_len / num_tasks;

    let chunked_data = buffer.chunks_mut(chunk_size);

    multi_write_buffers(chunked_data, chunk_size);
}

pub fn large_test_threaded() {
    let mut buffer = vec![0u64; 1024000000];
    let buffer_len = buffer.len();

    let num_tasks = num_cpus::get();

    let chunk_size = buffer_len / num_tasks;

    let chunked_data = buffer.chunks_mut(chunk_size);

    multi_write_buffers(chunked_data, chunk_size);
}

pub fn multi_write_buffers(buffers: ChunksMut<u64>, chunk_size: usize) {
    thread::scope(|t| {
        for (i, buffer) in buffers.into_iter().enumerate() {
            t.spawn(move || {
                for j in 0..buffer.len() {
                    buffer[j] = ((i * chunk_size) + j) as u64;
                }
            });
        }
    });
}

fn criterion_benchmark(b: &mut Criterion) {
    let mut c = b.benchmark_group("Ble");
    c.measurement_time(Duration::from_secs(180));
    c.bench_function("sma_single", |b| {
        b.iter(|| {
            small_test_single();
        })
    });
    c.bench_function("sma_thr", |b| {
        b.iter(|| {
            small_test_threaded();
        })
    });
    c.bench_function("med_single", |b| {
        b.iter(|| {
            med_test_single();
        })
    });
    c.bench_function("med_thr", |b| {
        b.iter(|| {
            med_test_threaded();
        })
    });
    c.bench_function("lar_single", |b| {
        b.iter(|| {
            large_test_single();
        })
    });
    c.bench_function("lar_thr", |b| {
        b.iter(|| {
            large_test_threaded();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
