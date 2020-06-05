use std::collections::VecDeque;

#[derive(PartialEq, Clone, Copy)]
pub enum IntcodeState {
    NotStarted,
    Running,
    PollingInput,
    Done,
}

const MEMORY_SIZE: u32 = 4096;

pub struct Intcode {
    memory: Vec<i64>,
    ip: usize,
    rb: isize,
    state: IntcodeState,
    input_queue: VecDeque<i64>,
    output_queue: VecDeque<i64>,
}

impl Intcode {
    pub fn new(memory: Vec<i64>, pad_memory: bool) -> Intcode {
        let padding: i32 = MEMORY_SIZE as i32 - memory.len() as i32;
        let memory: Vec<i64> = if pad_memory && padding > 0 {
            vec![memory, vec![0; padding as usize]].concat()
        } else {
            memory
        };
        Intcode {
            memory,
            ip: 0,
            rb: 0,
            state: IntcodeState::NotStarted,
            input_queue: VecDeque::new(),
            output_queue: VecDeque::new(),
        }
    }

    pub fn get_state(&self) -> IntcodeState {
        self.state
    }

    pub fn queue_input(&mut self, n: i64) {
        if self.state != IntcodeState::Done {
            self.input_queue.push_back(n)
        }
    }

    pub fn dequeue_output(&mut self) -> Option<i64> {
        self.output_queue.pop_front()
    }

    fn parse_opcode(opcode: u32) -> (u8, Vec<u8>) {
        let mut opcode = opcode;
        let op = (opcode % 100) as u8;
        opcode /= 100;
        let mut modes = Vec::new();
        while opcode > 0 {
            modes.push((opcode % 10) as u8);
            opcode /= 10;
        }
        while modes.len() < 3 {
            modes.push(0);
        }
        (op, modes)
    }

    fn read_next_ins_param(&mut self, mode: u8) -> i64 {
        self.ip += 1;
        let mut result = self.memory[self.ip];
        if mode == 0 {
            result = self.memory[result as usize];
        }
        if mode == 2 {
            result = self.memory[(self.rb + result as isize) as usize];
        }
        result
    }

    fn write_next_ins_param(&mut self, val: i64, mode: u8) {
        self.ip += 1;
        let dest_addr = self.memory[self.ip] as usize;
        if mode == 0 {
            self.memory[dest_addr] = val;
        }
        if mode == 2 {
            self.memory[(self.rb + dest_addr as isize) as usize] = val;
        }
    }

    fn compute_next_op(&mut self) -> Result<&IntcodeState, String> {
        if self.state == IntcodeState::NotStarted {
            self.state = IntcodeState::Running;
        }

        if self.state == IntcodeState::Done {
            return Ok(&self.state);
        }

        if self.state == IntcodeState::PollingInput {
            if self.input_queue.is_empty() {
                return Ok(&self.state);
            } else {
                self.state = IntcodeState::Running;
            }
        }

        if self.ip >= self.memory.len() {
            return Err(format!(
                "Invalid program counter (perhaps the machine needs more memory?): {}",
                self.ip
            ));
        }

        let opcode = self.memory[self.ip] as u32;
        let (op, modes) = Intcode::parse_opcode(opcode);

        match op {
            1 => {
                let left = self.read_next_ins_param(modes[0]);
                let right = self.read_next_ins_param(modes[1]);
                self.write_next_ins_param(left + right, modes[2]);
                self.ip += 1;
            }
            2 => {
                let left = self.read_next_ins_param(modes[0]);
                let right = self.read_next_ins_param(modes[1]);
                self.write_next_ins_param(left * right, modes[2]);
                self.ip += 1;
            }
            3 => {
                if self.input_queue.is_empty() {
                    self.state = IntcodeState::PollingInput;
                    return Ok(&self.state);
                }
                let i = self.input_queue.pop_front().unwrap();
                self.write_next_ins_param(i, modes[0]);
                self.ip += 1;
            }
            4 => {
                let o = self.read_next_ins_param(modes[0]);
                self.output_queue.push_back(o);
                self.ip += 1;
            }
            5 => {
                let cond = self.read_next_ins_param(modes[0]) != 0;
                let jump = self.read_next_ins_param(modes[1]) as usize;
                if cond {
                    self.ip = jump;
                } else {
                    self.ip += 1;
                }
            }
            6 => {
                let cond = self.read_next_ins_param(modes[0]) == 0;
                let jump = self.read_next_ins_param(modes[1]) as usize;
                if cond {
                    self.ip = jump;
                } else {
                    self.ip += 1;
                }
            }
            7 => {
                let first = self.read_next_ins_param(modes[0]);
                let snd = self.read_next_ins_param(modes[1]);
                let result = if first < snd { 1 } else { 0 };
                self.write_next_ins_param(result, modes[2]);
                self.ip += 1;
            }
            8 => {
                let first = self.read_next_ins_param(modes[0]);
                let snd = self.read_next_ins_param(modes[1]);
                let result = if first == snd { 1 } else { 0 };
                self.write_next_ins_param(result, modes[2]);
                self.ip += 1;
            }
            9 => {
                self.rb += self.read_next_ins_param(modes[0]) as isize;
                self.ip += 1;
            }
            99 => {
                self.state = IntcodeState::Done;
                return Ok(&self.state);
            }
            _ => return Err(format!("Unsupported opcode: {}", opcode)),
        }

        Ok(&self.state)
    }

    pub fn progress_program(&mut self) -> Result<(), String> {
        while self.state != IntcodeState::Done {
            let state = self.compute_next_op()?;
            if *state == IntcodeState::PollingInput {
                break;
            }
        }
        Ok(())
    }
}

impl Clone for Intcode {
    fn clone(&self) -> Self {
        Intcode {
            memory: self.memory.clone(),
            ip: self.ip,
            rb: self.rb,
            state: self.state,
            input_queue: self.input_queue.clone(),
            output_queue: self.output_queue.clone(),
        }
    }
}

#[test]
fn can_parse_opcode_and_modes() {
    assert_eq!(Intcode::parse_opcode(1002), (2, vec![0, 1, 0]));
    assert_eq!(Intcode::parse_opcode(31204), (4, vec![2, 1, 3]));
}

#[test]
fn can_run_intcode_programs() {
    let mut ic = Intcode::new([1, 0, 0, 0, 99].to_vec(), false);
    ic.progress_program().unwrap();
    assert_eq!(ic.memory, [2, 0, 0, 0, 99].to_vec());

    ic = Intcode::new([2, 3, 0, 3, 99].to_vec(), false);
    ic.progress_program().unwrap();
    assert_eq!(ic.memory, [2, 3, 0, 6, 99].to_vec());

    ic = Intcode::new([2, 4, 4, 5, 99, 0].to_vec(), false);
    ic.progress_program().unwrap();
    assert_eq!(ic.memory, [2, 4, 4, 5, 99, 9801].to_vec());

    ic = Intcode::new([1, 1, 1, 4, 99, 5, 6, 0, 99].to_vec(), false);
    ic.progress_program().unwrap();
    assert_eq!(ic.memory, [30, 1, 1, 4, 2, 5, 6, 0, 99].to_vec());
}

#[test]
fn intcode_can_do_io() {
    let mut ic = Intcode::new([3, 0, 4, 0, 99].to_vec(), false);
    ic.queue_input(1337);
    ic.progress_program().unwrap();
    assert_eq!(ic.output_queue.len(), 1);
    assert_eq!(ic.dequeue_output(), Some(1337));
}

#[test]
fn intcode_handles_jumps_and_conditionals() {
    let memory = vec![
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ];

    let mut ic = Intcode::new(memory.clone(), false);
    ic.queue_input(7);
    ic.progress_program().unwrap();
    assert_eq!(ic.output_queue.len(), 1);
    assert_eq!(ic.dequeue_output(), Some(999));

    ic = Intcode::new(memory.clone(), false);
    ic.queue_input(8);
    ic.progress_program().unwrap();
    assert_eq!(ic.output_queue.len(), 1);
    assert_eq!(ic.dequeue_output(), Some(1000));

    ic = Intcode::new(memory, false);
    ic.queue_input(9);
    ic.progress_program().unwrap();
    assert_eq!(ic.output_queue.len(), 1);
    assert_eq!(ic.dequeue_output(), Some(1001));
}

#[test]
fn intcode_handles_mode_2_relative_positions_and_memory_padding() {
    let memory = vec![
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];
    let mut ic = Intcode::new(memory.clone(), true);
    ic.progress_program().unwrap();
    let result: Vec<i64> = ic.output_queue.into_iter().collect();
    assert_eq!(result, memory);
}

#[test]
fn intcode_handles_large_numbers() {
    let mut memory = vec![1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0];
    let mut ic = Intcode::new(memory.clone(), true);
    ic.progress_program().unwrap();
    let result = ic.dequeue_output().unwrap();
    assert_eq!(result.to_string().len(), 16);

    memory = vec![104, 1_125_899_906_842_624, 99];
    ic = Intcode::new(memory, true);
    ic.progress_program().unwrap();
    let result = ic.dequeue_output().unwrap();
    assert_eq!(result, 1_125_899_906_842_624);
}
