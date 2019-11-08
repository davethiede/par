use chrono;
use rand;
use rayon::prelude::*;
use std::{thread, time};

#[derive(Debug, Copy, Clone)]
struct Job<'a> {
    filename: &'a str,
    created: chrono::DateTime<chrono::Utc>,
}
impl<'a> Job<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            filename: name,
            created: chrono::Utc::now(),
        }
    }
    fn run(self) {
        let r = rand::random::<u8>();
        let delay = time::Duration::from_millis(r as u64);
        thread::sleep(delay);
    }
}

fn main() {
    println!("Hello, world!");
    let mut jobs: Vec<Job> = vec![];
    jobs.insert(0, Job::new("one"));
    jobs.push(Job::new("two"));
    jobs.push(Job::new("three"));
    jobs.insert(0, Job::new("four"));

    let jobiter = jobs.par_iter().map(|&x|x.run());
    dbg!(&jobiter);
}
