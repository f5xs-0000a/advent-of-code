use std::io::Read as _;
use std::collections::HashSet;

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut buffer)
        .expect("Unable to read from stdin");

    let mut single_passed = HashSet::new();
    let mut single_coords = (0, 0);

    let mut dual_passed = HashSet::new();
    let mut half_coords = (0, 0);
    let mut robo_coords = (0, 0);

    single_passed.insert(single_coords.clone());
    dual_passed.insert(half_coords.clone());
    for (p, c) in buffer.split_whitespace().flat_map(|s| s.chars()).enumerate()
    {
        let (dx, dy) = match c {
            '^' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            'v' => (0, -1),
            u => panic!("Unexpected character: {}", u),
        };

        single_coords.0 += dx;
        single_coords.1 += dy;

        single_passed.insert(single_coords.clone());

        if p % 2 == 0 {
            half_coords.0 += dx;
            half_coords.1 += dy;
            dual_passed.insert(half_coords.clone());
        }

        else {
            robo_coords.0 += dx;
            robo_coords.1 += dy;
            dual_passed.insert(robo_coords.clone());
        }
    }

    eprintln!("Day 2.1: {}", single_passed.len());
    eprintln!("Day 2.2: {}", dual_passed.len());
}
