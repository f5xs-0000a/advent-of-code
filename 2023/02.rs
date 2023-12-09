use std::io::BufRead as _;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Debug)]
enum Color {
    Red,
    Blue,
    Green,
}

fn main() {
    use Color::*;
    let stdin_lock = std::io::stdin().lock();

    // NOTE:
    // there are X games
    // each game has Y rounds
    // each round has Z colors
    // each color is pulled W times

    let mut games/*: Vec<Vec<Vec<(usize, Color)>>*/ = vec![];
    for line in stdin_lock.lines() {
        let line = line.unwrap();

        let mut rounds = vec![];
        for round in line.split(": ").skip(1).next().unwrap().split("; ") {
            let mut pulls = vec![];
            for color_pull in round.split(", ") {
                let mut pull_split = color_pull.split(" ");
                let count = pull_split
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();

                let color = match pull_split.next().unwrap() {
                    "red" => Red,
                    "green" => Green,
                    "blue" => Blue,
                    x => panic!("unexpected color {}", x),
                };

                pulls.push((count, color));
            }

            rounds.push(pulls);
        }

        games.push(rounds);
    }

    let mut sum_1 = 0;
    let mut sum_2 = 0;
    for (id, game) in games.iter().enumerate() {
        let id = id + 1;

        let mut p1_valid = true;
        let mut color_counts = HashMap::new();

        for round in game.iter() {
            for (count, color) in round.iter() {
                let is_over = match color {
                    Red => 12 < *count,
                    Green => 13 < *count,
                    Blue => 14 < *count,
                };

                // set maximum value
                let ptr = color_counts.entry(color).or_insert(*count);
                *ptr = (*ptr).max(*count);

                if is_over {
                    p1_valid = false;
                }
            }
        }

        if p1_valid {
            sum_1 += id;
        }

        sum_2 += color_counts.values().product::<usize>();
    }

    dbg!(&sum_1);
    dbg!(&sum_2);
}
