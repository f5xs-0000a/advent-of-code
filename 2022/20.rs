use std::io::BufRead as _;

#[derive(Clone)]
struct Pair {
    appearance: usize,
    value: isize,
}

fn main() {
    let mut values = vec![];

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
        let x = buffer
            .split_whitespace()
            .next()
            .unwrap()
            .parse::<isize>()
            .unwrap();
        let pair = Pair {
            appearance: values.len(),
            value: x,
        };


        values.push(pair);
    }

    let mut values_1 = values.clone();
    spin(&mut values_1, 1);
    let hash_sum = values_1
        .iter()
        .cycle()
        .map(|x| x.value)
        .skip_while(|x| *x != 0)
        .step_by(1000)
        .take(4)
        .sum::<isize>();
    eprintln!("Day 20.1: {}", hash_sum);

    let mut values_2 = values
        .into_iter()
        .map(|mut x| {
            x.value *= 811589153;
            x
        })
        .collect::<Vec<_>>();
    spin(&mut values_2, 10);
    let hash_sum = values_2
        .iter()
        .cycle()
        .map(|x| x.value)
        .skip_while(|x| *x != 0)
        .step_by(1000)
        .take(4)
        .sum::<isize>();
    eprintln!("Day 20.2: {}", hash_sum);
}

fn spin(pairs: &mut [Pair], mix_count: usize) {
    let mut current_index = 0;

    // determine the current index we're concerned about
    for i in (0 .. pairs.len()).cycle().take(pairs.len() * mix_count) {
        // while the current index does not point to the pair that shares the
        // same appearance value as i, update current index
        while pairs[current_index].appearance != i {
            current_index = (current_index + 1) % pairs.len();
        }

        // find out how much to move the current value clockwise
        // reminder: clockwise.
        // reminder: you took away a value therefore the clock's size shrinks
        let mut rotation = pairs[current_index].value;
        rotation %= (pairs.len() - 1) as isize;
        rotation += (pairs.len() - 1) as isize;
        rotation %= (pairs.len() - 1) as isize;

        // bubble the element clockwise
        for j in 0 .. rotation as usize {
            pairs.swap(
                (current_index + j) % pairs.len(),
                (current_index + j + 1) % pairs.len(),
            );
        }
    }
}
