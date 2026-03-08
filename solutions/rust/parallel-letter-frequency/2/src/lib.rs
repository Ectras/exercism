use std::{collections::HashMap, thread};

fn extend_counts(base: &mut HashMap<char, usize>, update: &HashMap<char, usize>) {
    for (key, value) in update.iter() {
        *base.entry(*key).or_default() += value;
    }
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    assert!(worker_count > 0);
    if input.is_empty() {
        return Default::default();
    }

    // Compute the number of lines per worker
    let chunk_size = input.len().div_ceil(worker_count);
    let final_counts = thread::scope(|s| {
        // Start a thread for each chunk
        let handles = input
            .chunks(chunk_size)
            .map(|lines| {
                s.spawn(|| {
                    // Count all letters in the chunk
                    let mut counts: HashMap<char, usize> = HashMap::new();
                    lines
                        .iter()
                        .flat_map(|line| line.chars())
                        .filter(|c| c.is_alphabetic())
                        .map(|c| c.to_ascii_lowercase())
                        .for_each(|c| {
                            *counts.entry(c).or_default() += 1;
                        });
                    counts
                })
            })
            .collect::<Vec<_>>();

        // Join the threads and fold the results into a combined HashMap
        handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .fold(HashMap::new(), |mut acc, counts| {
                extend_counts(&mut acc, &counts);
                acc
            })
    });
    final_counts
}
