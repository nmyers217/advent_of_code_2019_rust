use crate::intcode::Intcode;
use std::fs;

pub fn solve() {
    let filename = "res/day_09.txt";
    let input = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Could not read file: {}", filename));
    let memory: Vec<i64> = input
        .trim()
        .split(',')
        .map(|s| {
            s.parse()
                .unwrap_or_else(|_| panic!("Could not parse {} into a i64", s))
        })
        .collect();

    let mut ic = Intcode::new(memory.clone(), true);
    ic.queue_input(1);
    ic.progress_program().unwrap();
    while let Some(o) = ic.dequeue_output() {
        println!("{}", o);
    }

    ic = Intcode::new(memory, true);
    ic.queue_input(2);
    ic.progress_program().unwrap();
    while let Some(o) = ic.dequeue_output() {
        println!("{}", o);
    }
}
