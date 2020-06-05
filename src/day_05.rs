use crate::intcode::Intcode;
use std::fs;

pub fn solve() {
    let filename = "res/day_05.txt";
    let input = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Could not read file: {}", filename));
    let memory: Vec<i64> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut ic = Intcode::new(memory.clone(), false);
    ic.queue_input(1);
    ic.progress_program()
        .unwrap_or_else(|_| panic!("Could not progress program!"));
    while let Some(n) = ic.dequeue_output() {
        if n > 0 {
            println!("{}", n);
        }
    }

    ic = Intcode::new(memory, false);
    ic.queue_input(5);
    ic.progress_program()
        .unwrap_or_else(|_| panic!("Could not progress program!"));
    while let Some(n) = ic.dequeue_output() {
        println!("{}", n);
    }
}
