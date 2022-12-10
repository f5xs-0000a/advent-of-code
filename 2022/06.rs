use std::io::BufRead as _;
use std::collections::VecDeque;

fn first_unique_contiguous(string: &str, count: usize) -> usize {
    let mut chars = string.chars();
    let mut buffer_chars = chars
        .by_ref()
        .take(count - 1)
        .collect::<VecDeque<_>>();

    for (i, ch) in chars.enumerate() {
        buffer_chars.push_back(ch);

        // check for duplicates
        let mut array_clone = vec!['a'; count];
        for (from, to) in buffer_chars.iter().zip(array_clone.iter_mut()) {
            *to = *from;
        }
        array_clone.sort_unstable();
        if !array_clone.windows(2).any(|a| a[0] == a[1]) {
            return i + count;
        }

        buffer_chars.pop_front();
    }

    string.len()
}

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();

    eprintln!("Day 6.1: {}", first_unique_contiguous(&buffer, 4));
    eprintln!("Day 6.1: {}", first_unique_contiguous(&buffer, 14));
}
