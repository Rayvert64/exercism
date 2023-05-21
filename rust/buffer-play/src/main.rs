use num_cpus;
use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    // Create a buffer
    let mut buffer = vec![0u64; 10240000];
    let buffer_len = buffer.len();

    let num_tasks = num_cpus::get();

    let chunk_size = buffer_len / num_tasks;

    let mut chunked_data = vec![];
    buffer
        .chunks_mut(chunk_size)
        .for_each(|slice| chunked_data.push(Arc::new(Mutex::new(slice))));
    multi_write_buffers(chunked_data, chunk_size);

    println!("Buffer: {:?}", buffer);
}

pub fn multi_write_buffers<'a>(buffers: Vec<Arc<Mutex<&'a mut [u64]>>>, chunk_size: usize) {
    thread::scope(|t| {
        for (i, buffer) in buffers.into_iter().enumerate() {
            t.spawn(move || {
                let mut buffer = buffer.lock().unwrap();
                for j in 0..buffer.len() {
                    buffer[j] = ((i * chunk_size) + j) as u64;
                }
            });
        }
    });
}
