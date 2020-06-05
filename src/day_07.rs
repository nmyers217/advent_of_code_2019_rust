use crate::intcode::{Intcode, IntcodeState};
use std::collections::{HashSet, VecDeque};
use std::fs;

pub fn solve() {
    let filename = "res/day_07.txt";
    let input = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Could not read file: {}", filename));

    let memory: Vec<i64> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut largest = 0;
    {
        let phase_permutations = permutations(0..5);
        for phases in phase_permutations {
            let signal = run_amplifier_controller(phases, &memory);
            if signal > largest {
                largest = signal;
            }
        }
    }

    let mut largest_two = 0;
    {
        let phase_permutations = permutations(5..10);
        for phases in phase_permutations {
            let signal = run_amplifier_controller(phases, &memory);
            if signal > largest_two {
                largest_two = signal;
            }
        }
    }

    println!("{}\n{}", largest, largest_two);
}

fn permutations(digits: std::ops::Range<u8>) -> HashSet<Vec<u8>> {
    let mut result: HashSet<Vec<u8>> = HashSet::new();

    fn recursive_helper(current: Vec<u8>, digits_left: HashSet<u8>, result: &mut HashSet<Vec<u8>>) {
        if digits_left.is_empty() {
            result.insert(current);
            return;
        }
        for d in &digits_left {
            let mut next_current = current.clone();
            let mut next_digits_left = digits_left.clone();
            next_current.push(*d);
            next_digits_left.remove(d);
            recursive_helper(next_current, next_digits_left, result);
        }
    };

    let mut digits_set = HashSet::new();
    for n in digits {
        digits_set.insert(n);
    }
    recursive_helper(Vec::new(), digits_set, &mut result);
    result
}

fn run_amplifier_controller(phases: Vec<u8>, memory: &[i64]) -> i64 {
    let mut intcodes: VecDeque<Intcode> = VecDeque::new();
    for (i, phase_setting) in phases.iter().enumerate() {
        let mut ic = Intcode::new(memory.to_vec(), false);
        ic.queue_input(*phase_setting as i64);
        if i == 0 {
            ic.queue_input(0);
        }
        intcodes.push_back(ic.clone());
    }

    let mut last_outputs: VecDeque<i64> = VecDeque::new();
    'feedback: loop {
        for (i, ic) in intcodes.iter_mut().enumerate() {
            while let Some(n) = last_outputs.pop_front() {
                ic.queue_input(n);
            }

            ic.progress_program().unwrap();

            while let Some(n) = ic.dequeue_output() {
                last_outputs.push_back(n);
            }

            if i == 4 && ic.get_state() == IntcodeState::Done {
                break 'feedback;
            }
        }
    }

    last_outputs.pop_front().unwrap()
}

#[test]
fn can_get_permutations() {
    let mut result = HashSet::new();
    result.insert(vec![0, 1]);
    result.insert(vec![1, 0]);
    assert_eq!(permutations(0..2), result);

    result.clear();
    result.insert(vec![0, 1, 2]);
    result.insert(vec![0, 2, 1]);
    result.insert(vec![1, 0, 2]);
    result.insert(vec![1, 2, 0]);
    result.insert(vec![2, 0, 1]);
    result.insert(vec![2, 1, 0]);
    assert_eq!(permutations(0..3), result);
}

#[test]
fn can_run_amplifier_controller() {
    let mut memory: Vec<i64> = vec![
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ];
    let result = run_amplifier_controller(vec![4, 3, 2, 1, 0], &memory);
    assert_eq!(result, 43210);

    memory = vec![
        3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
        0, 0,
    ];
    let result = run_amplifier_controller(vec![0, 1, 2, 3, 4], &memory);
    assert_eq!(result, 54321);

    memory = vec![
        3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
        31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
    ];
    let result = run_amplifier_controller(vec![1, 0, 4, 3, 2], &memory);
    assert_eq!(result, 65210);
}

#[test]
fn can_run_amplifier_controller_in_feedback_loob() {
    let mut memory: Vec<i64> = vec![
        3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
        1005, 28, 6, 99, 0, 0, 5,
    ];
    let result = run_amplifier_controller(vec![9, 8, 7, 6, 5], &memory);
    assert_eq!(result, 139_629_729);

    memory = vec![
        3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54, -5,
        54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53,
        1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
    ];
    let result = run_amplifier_controller(vec![9, 7, 8, 5, 6], &memory);
    assert_eq!(result, 18216);
}
