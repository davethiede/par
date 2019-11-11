use chrono;
use rand;
use rayon::prelude::*;
use std::{thread, time};

#[derive(Debug, Copy, Clone)]
struct Job<'a> {
    filename: &'a str,
    created: chrono::DateTime<chrono::Utc>,
    duration: f32,
    thread_id: thread::ThreadId,
}
impl<'a> Job<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            filename: name,
            created: chrono::Utc::now(),
            duration: 0.0,
            thread_id: thread::current().id(),
        }
    }
    fn run(&mut self) {
        let start = time::Instant::now();
        let r = rand::random::<u8>();
        let delay = time::Duration::from_millis(r as u64);
        thread::sleep(delay);
        let duration = start.elapsed();
        self.duration = duration.as_secs_f32();
        self.thread_id = thread::current().id();
        println!("{}", self.duration);
    }
}

fn main() {
    println!("Hello, world!");
    let mut jobs: Vec<Job> = vec![];
    jobs.insert(0, Job::new("one"));
    jobs.push(Job::new("two"));
    jobs.push(Job::new("three"));
    jobs.insert(0, Job::new("four"));
    jobs.push(Job::new("five"));
    jobs.push(Job::new("six"));
    jobs.push(Job::new("seven"));
    jobs.push(Job::new("eight"));
    jobs.push(Job::new("nine"));

    let jobiter = jobs.par_iter_mut().map(|x: &mut Job| x.run());
    let mut results = vec![];
    jobiter.collect_into_vec(&mut results);
    dbg!(&results);
    dbg!(&jobs);
}
