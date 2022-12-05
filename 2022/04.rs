use std::io::BufRead as _;

fn main() {
    let mut part_1_count = 0;
    let mut part_2_count = 0;

    let mut buffer = String::new();
    let mut stdin = std::io::stdin().lock();
    while {
        buffer.clear();
        match stdin.read_line(&mut buffer) {
            Ok(0) => false,
            Err(_) => false,
            _ => true
        }
    } {
        let mut vals = buffer.split_terminator(['-', ',', '\n']);

        let left_from = vals.next().unwrap().parse::<usize>().unwrap();
        let left_to = vals.next().unwrap().parse::<usize>().unwrap();
        let right_from = vals.next().unwrap().parse::<usize>().unwrap();
        let right_to = vals.next().unwrap().parse::<usize>().unwrap();

        if left_from <= right_from && right_from <= left_to &&
            left_from <= right_to && right_to <= left_to {
                part_1_count += 1;
        }

        else if right_from <= left_from && left_from <= right_to &&
            right_from <= left_to && left_to <= right_to {
                part_1_count += 1;
        }

        if (left_from <= right_from && right_from <= left_to) ||
            (left_from <= right_to && right_to <= left_to) {
                part_2_count += 1;
        }

        else if (right_from <= left_from && left_from <= right_to) ||
            (right_from <= left_to && left_to <= right_to) {
                part_2_count += 1;
        }
    }

    eprintln!("Day 4.1: {}", part_1_count);
    eprintln!("Day 4.2: {}", part_2_count);
}
