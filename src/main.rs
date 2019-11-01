use rayon::prelude::*;
use std::default::Default;
#[derive(Debug, Default)]
struct Job {
    filename: String,
}
impl Job {
    fn new() -> Self {
        Default::default()
    }
    fn add(s: &str) -> Self {
        Job {
            filename: s.to_string(),
        }
    }
}
fn main() {
    println!("Hello, world!");
    let mut jobs: Vec<Job> = vec!();
    jobs.insert(0, Job::add("one"));
    jobs.push(Job::add("two"));
    jobs.push(Job::add("three"));
    jobs.insert(0, Job::add("four"));

    let jobiter = jobs.par_iter_mut();
    dbg!(&jobiter);
}
