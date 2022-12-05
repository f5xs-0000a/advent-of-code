use std::collections::HashSet;
use std::io::BufRead as _;

////////////////////////////////////////////////////////////////////////////////

// don't worry, this will be optimized by the compiler.
// chars in rust are in UTF-8 and are a bitch to convert into ASCII
fn get_score(ch: char) -> usize {
    match ch {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => unreachable!(),
    }
}

fn main() {
    let mut part_1_value = 0;

    let mut line_sets = vec![];

    let mut stdin = std::io::stdin().lock();
    let mut buffer = String::new();
    while {
        buffer.clear();
        match stdin.read_line(&mut buffer) {
            Ok(0) => false,
            Err(_) => false,
            _ => true
        }
    } {
        let len = buffer.len() - 1; // ignore the newline
        let mut set = HashSet::with_capacity(len);

        // part 1 calculation
        let mut chars = buffer
            .chars()
            .take(len)
            .inspect(|c| { set.insert(c.clone()); });
        let existing = chars
            .by_ref()
            .take(len / 2).collect::<HashSet<_>>();
        let common = chars
            .by_ref()
            .filter(|c| existing.contains(&c))
            .next()
            .unwrap();
        part_1_value += get_score(common);

        // exhaust the chars iter so inspect takes full effect
        for _ in chars {}

        // part 2 prepping
        line_sets.push(set);
    }

    let mut part_2_value = 0;
    for set_group in line_sets.chunks_exact(3) {
        let common = set_group[0]
            .intersection(&set_group[1])
            .cloned()
            .collect::<HashSet<_>>()
            .intersection(&set_group[2])
            .cloned()
            .next()
            .unwrap();

        part_2_value += get_score(common);
    }

    eprintln!("Day 3.1: {}", &part_1_value);
    eprintln!("Day 3.2: {}", &part_2_value);
}
