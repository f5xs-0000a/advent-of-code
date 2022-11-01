use std::io::BufRead as _;

enum Direction {
    Forward,
    Down,
    Up,
}

pub fn main() {
    let entries = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.expect("Expected line");
            let mut ws_split = line.split_whitespace();

            let direction = match ws_split.next().expect("Expected direction") {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => unreachable!(),
            };

            let units = ws_split
                .next()
                .expect("Expected units")
                .parse::<isize>()
                .expect("Expected number");

            (direction, units)
        })
        .collect::<Vec<_>>();

    let (x, y) = entries
        .iter()
        .fold((0, 0), |(mut x, mut y), (direction, units)| {
            match direction {
                Direction::Forward => x += units,
                Direction::Up => y -= units,
                Direction::Down => y += units,
            }

            (x, y)
        });

    eprintln!("Day 2.1: {}", x * y);

    let (x, y, _) = entries
        .iter()
        .fold((0, 0, 0), |(mut x, mut y, mut aim), (direction, units)| {
            match direction {
                Direction::Forward => {
                    x += units;
                    y += aim * units;
                },

                Direction::Up => {
                    //y -= units;
                    aim -= units;
                },

                Direction::Down => {
                    //y += units;
                    aim += units;
                },
            }

            (x, y, aim)
        });

    eprintln!("Day 2.2: {}", x * y);
}
