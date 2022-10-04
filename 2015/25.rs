use std::io::BufRead as _;

fn determine_index(row: usize, col: usize) -> usize {
    let row = row - 1;
    let col = col - 1;
    let level = row + col;

    // to get to the current level, we must sum from 1 to level - 1
    // luckily, OEIS has already helped us on that. given level, find f(level)
    // such that f(level) = sum from 1 to level
    // https://oeis.org/A000217
    let steps_required = level * (level + 1) / 2;

    steps_required + col
}

fn main() {
    let mut buffer = String::new();
    eprint!("Please enter the row number: ");
    std::io::stdin()
        .lock()
        .read_line(&mut buffer)
        .expect("Unable to read from stdin");
    let row = buffer
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    buffer.clear();
    eprint!("Please enter the column number: ");
    std::io::stdin()
        .lock()
        .read_line(&mut buffer)
        .expect("Unable to read from stdin");
    let col = buffer
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut number = 20151125u64;
    for _ in 0 .. determine_index(row, col) {
        number = (number * 252533) % 33554393;
    }

    eprintln!("Day 25: {}", number);
}
