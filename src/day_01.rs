use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    let input_file = "res/day_01.txt";
    let file = File::open(input_file).unwrap_or_else(|_| panic!("Could not open {}", input_file));
    let reader = BufReader::new(file);

    let mut part_one = 0;
    let mut part_two = 0;

    for line in reader.lines() {
        let mass = line.expect("Could not read line");
        let mass: u32 = mass
            .parse()
            .unwrap_or_else(|_| panic!("Could not parse {}", mass));

        part_one += fuel(mass);
        part_two += fuel_recursive(mass);
    }

    println!("{}\n{}", part_one, part_two);
}

fn fuel(mass: u32) -> u32 {
    let temp: i32 = mass as i32 / 3 - 2;
    if temp <= 0 {
        0
    } else {
        temp as u32
    }
}

fn fuel_recursive(mass: u32) -> u32 {
    if mass == 0 {
        0
    } else {
        let f = fuel(mass);
        f + fuel_recursive(f)
    }
}

#[test]
fn can_calc_fuel() {
    assert_eq!(fuel(12), 2);
    assert_eq!(fuel(14), 2);
    assert_eq!(fuel(1969), 654);
    assert_eq!(fuel(100_756), 33583);
}

#[test]
fn can_calc_fuel_recursively() {
    assert_eq!(fuel_recursive(14), 2);
    assert_eq!(fuel_recursive(1969), 966);
    assert_eq!(fuel_recursive(100_756), 50346);
}
