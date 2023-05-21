use num_cpus;
use std::{slice::ChunksMut, thread};

fn main() {
    // Create a buffer
    let mut buffer = vec![0u64; 1024000000];
    let buffer_len = buffer.len();

    let num_tasks = num_cpus::get();

    let chunk_size = buffer_len / num_tasks;

    let chunked_data = buffer.chunks_mut(chunk_size);

    multi_write_buffers(chunked_data, chunk_size);

    //println!("Buffer: {:?}", buffer);
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
