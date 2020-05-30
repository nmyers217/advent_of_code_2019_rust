use std::fs;

struct Intcode {
    ip: usize,
    memory: Vec<u32>,
    done: bool,
}

impl Intcode {
    fn new(memory: Vec<u32>) -> Intcode {
        Intcode {
            memory: memory,
            ip: 0,
            done: false,
        }
    }

    fn compute_next_op(&mut self) -> Result<(), String> {
        if self.done {
            return Ok(());
        }

        if self.ip >= self.memory.len() {
            return Err(format!("Invalid program counter: {}", self.ip));
        }

        let opcode = self.memory[self.ip];

        if opcode == 99 {
            self.done = true;
            return Ok(());
        }

        self.ip += 1;
        let left_addr: usize = self.memory[self.ip] as usize;
        self.ip += 1;
        let right_addr: usize = self.memory[self.ip] as usize;
        self.ip += 1;
        let dest_addr: usize = self.memory[self.ip] as usize;
        self.ip += 1;
        match opcode {
            1 => self.memory[dest_addr] = self.memory[left_addr] + self.memory[right_addr],
            2 => self.memory[dest_addr] = self.memory[left_addr] * self.memory[right_addr],
            _ => return Err(format!("Unsupported opcode: {}", opcode)),
        }
        Ok(())
    }

    fn run_program(&mut self) -> Result<(), String> {
        while !self.done {
            self.compute_next_op()?;
        }
        Ok(())
    }
}

pub fn solve() {
    let filename = "res/day_02.txt";
    let input = fs::read_to_string(filename).expect(&format!("Could not read file: {}", filename));
    let memory: Vec<u32> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut ic = Intcode::new(memory.clone());
    ic.memory[1] = 12;
    ic.memory[2] = 2;
    ic.run_program().expect("Coult not run intcode program.");
    println!("{}", ic.memory[0]);

    let target = 19690720;
    for noun in 0..=99 {
        for verb in 0..=99 {
            ic = Intcode::new(memory.clone());
            ic.memory[1] = noun;
            ic.memory[2] = verb;
            ic.run_program().expect("Coult not run intcode program.");
            if ic.memory[0] == target {
                println!("{}", 100 * noun + verb);
                return;
            }
        }
    }
}

#[test]
fn can_run_intcode_programs() {
    let mut ic = Intcode::new([1, 0, 0, 0, 99].to_vec());
    ic.run_program().unwrap();
    assert_eq!(ic.memory, [2, 0, 0, 0, 99].to_vec());

    ic = Intcode::new([2, 3, 0, 3, 99].to_vec());
    ic.run_program().unwrap();
    assert_eq!(ic.memory, [2, 3, 0, 6, 99].to_vec());

    ic = Intcode::new([2, 4, 4, 5, 99, 0].to_vec());
    ic.run_program().unwrap();
    assert_eq!(ic.memory, [2, 4, 4, 5, 99, 9801].to_vec());

    ic = Intcode::new([1, 1, 1, 4, 99, 5, 6, 0, 99].to_vec());
    ic.run_program().unwrap();
    assert_eq!(ic.memory, [30, 1, 1, 4, 2, 5, 6, 0, 99].to_vec());
}
