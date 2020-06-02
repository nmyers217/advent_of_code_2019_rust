use std::collections::HashMap;
use std::fs;

pub fn solve() {
    let filename = "res/day_06.txt";
    let input = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Could not read file: {}", filename));
    let orbital_data = get_orbital_data(&input);
    let mut cache: HashMap<String, Vec<String>> = HashMap::new();
    let mut sum = 0;
    for k in orbital_data.keys() {
        sum += trace_orbits(k, &orbital_data, &mut cache).len();
    }
    let shortest_path = path_between("YOU", "SAN", &orbital_data, &mut cache).len() - 1;
    println!("{}\n{}", sum, shortest_path);
}

fn get_orbital_data(input: &str) -> HashMap<&str, &str> {
    let mut orbital_data = HashMap::new();
    for line in input.trim().split('\n') {
        let items: Vec<&str> = line.trim().split(')').collect();
        orbital_data.insert(items[1], items[0]);
    }
    orbital_data
}

fn trace_orbits(
    planet: &str,
    orbital_data: &HashMap<&str, &str>,
    cache: &mut HashMap<String, Vec<String>>,
) -> Vec<String> {
    if let Some(orbiting) = orbital_data.get(planet) {
        if let Some(path) = cache.get(planet) {
            path.clone()
        } else {
            let result = vec![
                vec![(*orbiting).to_string()],
                trace_orbits(orbiting, orbital_data, cache),
            ]
            .concat();
            cache.insert(planet.to_string(), result.clone());
            result
        }
    } else {
        Vec::new()
    }
}

fn path_between(
    planet_a: &str,
    planet_b: &str,
    orbital_data: &HashMap<&str, &str>,
    cache: &mut HashMap<String, Vec<String>>,
) -> Vec<String> {
    let mut path_a = trace_orbits(planet_a, orbital_data, cache);
    path_a.reverse();
    let mut path_b = trace_orbits(planet_b, orbital_data, cache);
    path_b.reverse();

    let mut a_i = 0;
    let mut b_i = 0;
    while path_a[a_i] == path_b[b_i] {
        a_i += 1;
        b_i += 1;
    }

    let mut result_a = path_a[a_i..].to_vec();
    result_a.reverse();
    let mut result_b = path_b[b_i..].to_vec();
    result_b.reverse();

    vec![result_a, vec![path_a[a_i - 1].to_string()], result_b].concat()
}

#[test]
fn can_count_total_orbits() {
    let test_input = r#"
        COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L
    "#
    .to_string();
    let mut cache: HashMap<String, Vec<String>> = HashMap::new();
    let chart = get_orbital_data(&test_input);

    assert_eq!(trace_orbits("COM", &chart, &mut cache).len(), 0);
    assert_eq!(trace_orbits("B", &chart, &mut cache).len(), 1);
    assert_eq!(trace_orbits("B", &chart, &mut cache).len(), 1);
    assert_eq!(trace_orbits("C", &chart, &mut cache).len(), 2);
    assert_eq!(trace_orbits("D", &chart, &mut cache).len(), 3);
    assert_eq!(trace_orbits("F", &chart, &mut cache).len(), 5);
    assert_eq!(trace_orbits("B", &chart, &mut cache).len(), 1);
    assert_eq!(trace_orbits("E", &chart, &mut cache).len(), 4);
    assert_eq!(trace_orbits("I", &chart, &mut cache).len(), 4);
    assert_eq!(trace_orbits("L", &chart, &mut cache).len(), 7);
}

#[test]
fn can_get_shortest_path_between_planets() {
    let test_input = r#"
        COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L
        K)YOU
        I)SAN
    "#
    .to_string();
    let mut cache: HashMap<String, Vec<String>> = HashMap::new();
    let chart = get_orbital_data(&test_input);

    let result: Vec<String> = vec!["K", "J", "E", "D", "I"]
        .iter()
        .map(|s| (*s).to_string())
        .collect();
    assert_eq!(path_between("YOU", "SAN", &chart, &mut cache), result);
}
