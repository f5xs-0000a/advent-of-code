use std::io::BufRead as _;
use std::collections::HashSet;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse() -> Vec<(Direction, usize)> {
    use Direction::*;

    let mut instructions = vec![];

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
        let mut words = buffer.split_whitespace();
        let direction = match words.next() {
            Some("U") => Up,
            Some("D") => Down,
            Some("L") => Left,
            Some("R") => Right,
            _ => unimplemented!(),
        };

        let steps = words.next().unwrap().parse::<usize>().unwrap();

        instructions.push((direction, steps));
    }

    instructions
}

fn follow_head_returning_updated(
    tail: &mut (isize, isize),
    head: &(isize, isize)
) -> bool {
    use std::cmp::Ordering::*;

    // if the tail does not lie somewhere outside the head's bounds,
    // don't do anything
    if !(tail.0 < head.0 - 1 || head.0 + 1 < tail.0 ||
        tail.1 < head.1 - 1 || head.1 + 1 < tail.1) {
        return false;
    }
    
    match tail.0.cmp(&head.0) {
        Greater => tail.0 -= 1,
        Less => tail.0 += 1,
        _ => {},
    }

    match tail.1.cmp(&head.1) {
        Greater => tail.1 -= 1,
        Less => tail.1 += 1,
        _ => {},
    }

    true
}

fn snake(
    length: usize,
    directions: impl Iterator<Item = (Direction, usize)>
) -> usize {
    use Direction::*;

    let mut snake = vec![(0, 0); length];
    let mut tail_steps = HashSet::new();
    tail_steps.insert((0, 0));

    for (direction, steps) in directions {
        let offset = match direction {
            Up => (0, 1),
            Down => (0, -1),
            Right => (1, 0),
            Left => (-1, 0),
        };

        for _ in 0 .. steps {
            snake[0].0 += offset.0;
            snake[0].1 += offset.1;

            let mut last_updated = false;

            for index in 0 .. snake.len() - 1 {
                let head = snake[index].clone();
                last_updated = follow_head_returning_updated(
                    &mut snake[index + 1],
                    &head,
                );
                if !last_updated {
                    break;
                }
            }

            if last_updated {
                tail_steps.insert(snake.last().unwrap().clone());
            }
        }
    }

    tail_steps.len()
}

fn main() {
    let instructions = parse();

    eprintln!("Day 9.1: {}", snake(2, instructions.iter().cloned()));
    eprintln!("Day 9.1: {}", snake(10, instructions.iter().cloned()));
}
