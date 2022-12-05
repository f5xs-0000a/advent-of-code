use std::io::BufRead as _;
use std::mem::swap;

fn main() {
    let mut stacks = [
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];

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
        if buffer == "\n" {
            break;
        }

        buffer.chars()
            .skip(1)
            .step_by(4)
            .zip(stacks.iter_mut())
            .filter(|(ch, _)| *ch != ' ')
            .for_each(|(ch, stack)| stack.push(ch));
    }

    for stack in stacks.iter_mut() {
        stack.pop(); // we've actually inserted the indices of the stacks
        stack.reverse();
    }

    let mut temp_from = vec![];
    let mut temp_to = vec![];
    let other_stacks = stacks.clone();

    let mut instructions = vec![];

    while {
        buffer.clear();
        match stdin.read_line(&mut buffer) {
            Ok(0) => false,
            Err(_) => false,
            _ => true
        }
    } {
        let mut words = buffer
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .map(|x| x.parse::<usize>().unwrap());

        let count = words.next().unwrap();
        let from = words.next().unwrap() - 1;
        let to = words.next().unwrap() - 1;

        instructions.push((count, from, to));
    }

    for (count, from, to) in instructions.iter().cloned() {
        swap(&mut temp_from, &mut stacks[from]);
        swap(&mut temp_to, &mut stacks[to]);
        for _ in 0 .. count {
            temp_to.push(temp_from.pop().unwrap());
        }
        swap(&mut temp_from, &mut stacks[from]);
        swap(&mut temp_to, &mut stacks[to]);
    }

    let top = stacks
        .iter()
        .filter_map(|stack| stack.last())
        .collect::<String>();

    eprintln!("Day 5.1: {}", &top);

    stacks = other_stacks;
    let mut temp_stack = vec![];
    for (count, from, to) in instructions.into_iter() {
        swap(&mut temp_from, &mut stacks[from]);
        swap(&mut temp_to, &mut stacks[to]);
        for _ in 0 .. count {
            temp_stack.push(temp_from.pop().unwrap());
        }
        for _ in 0 .. count {
            temp_to.push(temp_stack.pop().unwrap());
        }
        swap(&mut temp_from, &mut stacks[from]);
        swap(&mut temp_to, &mut stacks[to]);
    }

    let top = stacks
        .iter()
        .filter_map(|stack| stack.last())
        .collect::<String>();

    eprintln!("Day 5.2: {}", &top);
}
