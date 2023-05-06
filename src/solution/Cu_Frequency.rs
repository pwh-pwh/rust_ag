use std::cmp::min;
use std::collections::HashMap;
use std::mem::take;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut answers = HashMap::<char, usize>::new();
    if input.is_empty() {
        return answers;
    }
    let input = input.join("");
    if input.is_empty() {
        return answers;
    }
    let mut chrunk = input.chars();
    let real_worker_count = min(worker_count, input.len());
    let mut thread_pool = Vec::with_capacity(real_worker_count);
    let mut worker_len = (input.len() / real_worker_count).max(1);
    if worker_len * real_worker_count < input.len() {
        worker_len += 1;
    }
    (0..real_worker_count).for_each(|_| {
        let chunk = chrunk.by_ref().take(worker_len).collect::<String>();
        let handle = thread::spawn(move || {
            let mut answer = HashMap::<char, usize>::new();
            chunk.chars().for_each(|c| {
                if c.is_alphabetic() {
                    *answer.entry(c.to_ascii_lowercase()).or_default() += 1;
                }
            });
            answer
        });
        thread_pool.push(handle);
    });
    for thread in thread_pool {
        let map = thread.join().unwrap();
        for (key, value) in map {
            *answers.entry(key).or_default() += value;
        }
    }
    answers
}
