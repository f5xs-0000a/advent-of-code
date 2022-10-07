use std::io::Read as _;
use std::collections::HashSet;

#[derive(Copy, Clone)]
enum TurnDirection {
    Left,
    Right,
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn turn(self, d: TurnDirection) -> Direction {
        use TurnDirection::*;
        use Direction::*;

        match (self, d) {
            (North, Left) => West,
            (East, Left) => North,
            (South, Left) => East,
            (West, Left) => South,
            (North, Right) => East,
            (East, Right) => South,
            (South, Right) => West,
            (West, Right) => North,
        }
    }

    fn incrementor(self) -> (isize, isize) {
        use Direction::*;

        match self {
            North => (0, 1),
            East => (1, 0),
            South => (0, -1),
            West => (-1, 0),
        }
    }
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut buffer)
        .expect("Unable to read from stdin");

    let mut coords = (0, 0);
    let mut visited = HashSet::<(isize, isize)>::new();
    let mut first_visited = None;
    let mut direction = Direction::North;
    for val in buffer.split_whitespace().flat_map(|s| s.split(",")).filter(|s| !s.is_empty()) {
        let dir = match val.chars().next() {
            Some('L') => TurnDirection::Left,
            Some('R') => TurnDirection::Right,
            _ => panic!("Expected L or R"),
        };

        let count = val
            .split_at(1)
            .1
            .parse::<isize>()
            .expect("Expected a number");

        direction = direction.turn(dir);
        let inc = direction.incrementor();

        for _ in 0 .. count {
            coords.0 += inc.0;
            coords.1 += inc.1;

            if first_visited.is_none() {
                if visited.contains(&coords) {
                    first_visited = Some(coords);
                }

                else {
                    visited.insert(coords);
                }
            }
        }
    }

    let first_visited = first_visited.unwrap();
    eprintln!("Day 1.1: {}", coords.0.abs() + coords.1.abs());
    eprintln!("Day 1.2: {}", first_visited.0.abs() + first_visited.1.abs());
}
