use std::io::BufRead as _;
use std::fmt::Write as _;

fn shuffle(digits: &[usize], rounds: usize) -> Vec<usize> {
    let mut adjacency = vec![(0, 0); digits.len()];

    for i in 0 .. digits.len() {
        adjacency[digits[i] as usize] = (
            digits[(i + digits.len() - 1) % digits.len()] as usize, // before
            digits[(i + 1) % digits.len()] as usize, // after
        );
    }

    let mut current_digit = digits[0] as usize;
    let mut stored = [0; 3];
    for _ in 0 .. rounds {
        stored[0] = adjacency[current_digit].1;
        stored[1] = adjacency[stored[0]].1;
        stored[2] = adjacency[stored[1]].1;

        let mut new_destination
            = (current_digit + digits.len() - 1) % digits.len();
        while stored.contains(&new_destination) {
            new_destination
                = (new_destination + digits.len() - 1) % digits.len();
        }

        //    D   E
        //      ^
        // F [A B C] G
        //
        // F - From
        // D - Destination
        // A, B, C - three digits
        //
        // drop connection from F to A, replace with D to A
        // drop connection from C to G, replace with C to E

        // F is `current_digit`
        // A, B, and C are in `stored`
        // D is `new_destination`
        
        // G is `post destination`
        // E is `after digit`
        
        let after_digit = adjacency[new_destination].1;
        let post_destination = adjacency[stored[2]].1;

        // fix bottom connectivity
        adjacency[current_digit].1 = post_destination;
        adjacency[post_destination].0 = current_digit;

        // fix top left connectivity
        adjacency[new_destination].1 = stored[0];
        adjacency[stored[0]].0 = new_destination;

        // fix top right connectivity
        adjacency[stored[2]].1 = after_digit;
        adjacency[after_digit].0 = stored[2];

        current_digit = adjacency[current_digit].1;
    }

    let mut finalized_digits = Vec::with_capacity(digits.len());
    let mut current_final_digit = 0;
    for _ in 0 .. digits.len() {
        finalized_digits.push(current_final_digit + 1);
        current_final_digit = adjacency[current_final_digit].1;
    }

    finalized_digits
}

fn main() {
    let mut buffer = String::new();
    let mut stdin = std::io::stdin().lock();
    stdin.read_line(&mut buffer).unwrap();

    let mut digits = Vec::with_capacity(9);
    for ch in buffer.chars() {
        if let Some(digit) = ch.to_digit(10) {
            digits.push(digit as usize - 1);
        }
    }

    assert_eq!(digits.len(), 9);

    let shuffled = shuffle(&*digits, 100)
        .into_iter()
        .skip(1)
        .fold(String::new(), |mut s, digit| {
            write!(&mut s, "{}", digit).unwrap();
            s
        });

    eprintln!("Day 23.1: {}", &shuffled);

    let big_digits = digits
        .into_iter()
        .chain(9 .. 1_000_000)
        .collect::<Vec<_>>();

    let shuffled = shuffle(&*big_digits, 10000000);
    eprintln!("Day 23.2: {}", shuffled[1] * shuffled[2]);
}
