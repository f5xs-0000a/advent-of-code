use std::io::Read as _;
use std::collections::HashSet;

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut buffer)
        .expect("Unable to read from stdin");

    let mut passed = HashSet::new();
    let mut coords = (0, 0);

    passed.insert(coords.clone());
    for c in buffer.split_whitespace().flat_map(|s| s.chars()) {
        match c {
            '^' => coords.1 += 1,
            '<' => coords.0 -= 1,
            '>' => coords.0 += 1,
            'v' => coords.1 -= 1,
            u => panic!("Unexpected character: {}", u),
        }

        passed.insert(coords.clone());
    }

    eprintln!("Day 2.1: {}", passed.len());

    passed.clear();
    coords = (0, 0);
    let mut robo_coords = (0, 0);

    passed.insert(coords.clone());
    for (p, c) in buffer.split_whitespace().flat_map(|s| s.chars()).enumerate()
    {
        let parity = p % 2;

        match (parity, c) {
            (0, '^') => coords.1 += 1,
            (0, '<') => coords.0 -= 1,
            (0, '>') => coords.0 += 1,
            (0, 'v') => coords.1 -= 1,
            (1, '^') => robo_coords.1 += 1,
            (1, '<') => robo_coords.0 -= 1,
            (1, '>') => robo_coords.0 += 1,
            (1, 'v') => robo_coords.1 -= 1,
            (_, u) => panic!("Unexpected character: {}", u),
        }

        if parity == 0 {
            passed.insert(coords.clone());
        }

        else {
            passed.insert(robo_coords.clone());
        }
    }

    eprintln!("Day 2.1: {}", passed.len());
}
