use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Point = (i32, i32);

pub fn solve() {
    let input = get_input();
    let (part_one, part_two) = trace_wires(input);
    println!("{}\n{}", part_one, part_two);
}

fn get_input() -> Vec<String> {
    let filename = "res/day_03.txt";
    let file = File::open(filename).unwrap_or_else(|_| panic!("Could not open file: {}", filename));
    let reader = BufReader::new(file);
    let mut result = Vec::new();
    for line in reader.lines() {
        result.push(line.unwrap());
    }
    result
}

fn trace_wires(input: Vec<String>) -> (i32, i32) {
    let mut sets: Vec<HashSet<Point>> = Vec::new();
    let mut maps: Vec<HashMap<Point, i32>> = Vec::new();

    for wire in input {
        let mut points: HashSet<Point> = HashSet::new();
        let mut steps: HashMap<Point, i32> = HashMap::new();
        let mut location = (0, 0);
        let mut step = 0;

        for move_str in wire.split(',') {
            let mut chars = move_str.chars();
            let d = chars.next().unwrap();
            let (dx, dy) = match d {
                'U' => (0, -1),
                'D' => (0, 1),
                'L' => (-1, 0),
                'R' => (1, 0),
                _ => panic!("Unsupport move!"),
            };
            let amt: i32 = chars.as_str().parse().unwrap();

            for _ in 0..amt {
                location = (location.0 + dx, location.1 + dy);
                points.insert(location);
                step += 1;
                steps.entry(location).or_insert(step);
            }
        }

        sets.push(points);
        maps.push(steps);
    }

    let mut least_man = std::i32::MAX;
    let mut least_steps = std::i32::MAX;
    for (x, y) in sets[0].intersection(&sets[1]) {
        let manhattan = x.abs() + y.abs();
        if manhattan < least_man {
            least_man = manhattan
        }
        let steps_a = maps[0].get(&(*x, *y)).unwrap();
        let steps_b = maps[1].get(&(*x, *y)).unwrap();
        let total_steps = steps_a + steps_b;
        if total_steps < least_steps {
            least_steps = total_steps
        }
    }

    (least_man, least_steps)
}

#[test]
fn can_calculate_min_manhattan_and_steps() {
    let mut input = [String::from("R8,U5,L5,D3"), String::from("U7,R6,D4,L4")].to_vec();
    let (a, b) = trace_wires(input);
    assert_eq!(a, 6);
    assert_eq!(b, 30);

    input = [
        String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
        String::from("U62,R66,U55,R34,D71,R55,D58,R83"),
    ]
    .to_vec();
    let (a, b) = trace_wires(input);
    assert_eq!(a, 159);
    assert_eq!(b, 610);

    input = [
        String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
        String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
    ]
    .to_vec();
    let (a, b) = trace_wires(input);
    assert_eq!(a, 135);
    assert_eq!(b, 410);
}
