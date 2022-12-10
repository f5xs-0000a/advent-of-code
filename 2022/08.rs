use std::io::BufRead as _;
use std::collections::HashSet;

fn parse() -> Vec<Vec<u8>> {
    let mut lines = vec![];
    let mut current_line = vec![];

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
        for ch in buffer.chars() {
            if let Some(d) = ch.to_digit(10) {
                current_line.push(d as u8);
            }
        }

        lines.push(current_line);
        current_line = vec![];
    }

    lines
}

fn part_1_solve(forest: &[Vec<u8>]) -> usize {
    // iterate through all the inner cells for row (then column)
    // identify the coordinates of the trees that are strictly greater in value
    //     than the previous tree
    // record such trees into the set
    let rows = forest.len();
    let columns = forest[0].len();
    let mut recorded_trees = HashSet::new();

    // add the edge trees
    for row in 0 .. rows {
        recorded_trees.insert((0, row));
        recorded_trees.insert((columns - 1, row));
    }
    for col in 0 .. columns {
        recorded_trees.insert((col, 0));
        recorded_trees.insert((col, rows - 1));
    }

    let folding_method = |
        mut recorded: Vec<(u8, (usize, usize))>,
        (value, coords): (u8, (usize, usize)),
    | {
        if let Some((last_value, _)) = recorded.last() {
            if *last_value < value {
                recorded.push((value, coords));
            }
        }

        else {
            recorded.push((value, coords));
        }

        recorded
    };

    // add the inner trees
    // do for vertical checking
    for col in 1 .. columns - 1 {
        // go southbound, find trees in strictly ascending order
        (0 .. rows - 1)
            .map(|row| (forest[col][row], (col, row)))
            .fold(vec![], folding_method)
            .into_iter()
            .for_each(|(_, coords)| { recorded_trees.insert(coords); });

        // go northbound, find trees in strictly ascending order
        (1 .. rows)
            .rev()
            .map(|row| (forest[col][row], (col, row)))
            .fold(vec![], folding_method)
            .into_iter()
            .for_each(|(_, coords)| { recorded_trees.insert(coords); });
    }

    // do for horizontal checking
    for row in 1 .. rows - 1 {
        // go southbound, find trees in strictly ascending order
        (0 .. columns - 1)
            .map(|col| (forest[col][row], (col, row)))
            .fold(vec![], folding_method)
            .into_iter()
            .for_each(|(_, coords)| { recorded_trees.insert(coords); });

        // go northbound, find trees in strictly ascending order
        (1 .. columns)
            .rev()
            .map(|col| (forest[col][row], (col, row)))
            .fold(vec![], folding_method)
            .into_iter()
            .for_each(|(_, coords)| { recorded_trees.insert(coords); });
    }

    recorded_trees.len()
}

fn part_2_solve(forest: &[Vec<u8>]) -> usize {
    let mut best_score = 0;

    let rows = forest.len();
    let columns = forest[0].len();

    for scenic_col in 1 .. columns - 1 {
        for scenic_row in 1 .. rows - 1 {
            let scenic_tree_height = forest[scenic_col][scenic_row];

            let mut north = 0;
            for row in (0 .. scenic_row).rev() {
                north += 1;
                if scenic_tree_height <= forest[scenic_col][row] {
                    break;
                }
            }
            if north == 0 {
                continue;
            }

            let mut south = 0;
            for row in scenic_row + 1 .. rows {
                south += 1;
                if scenic_tree_height <= forest[scenic_col][row] {
                    break;
                }
            }

            if south == 0 {
                continue;
            }

            let mut west = 0;
            for col in (0 .. scenic_col).rev() {
                west += 1;
                if scenic_tree_height <= forest[col][scenic_row] {
                    break;
                }
            }

            if west == 0 {
                continue;
            }

            let mut east = 0;
            for col in scenic_col + 1 .. columns {
                east += 1;
                if scenic_tree_height <= forest[col][scenic_row] {
                    break;
                }
            }

            let scenic_score = north * west * east * south;
            if best_score < scenic_score {
                best_score = scenic_score;
            }
        }
    }

    best_score
}

fn main() {
    let forest = parse();
    eprintln!("Day 8.1: {}", part_1_solve(&forest));
    eprintln!("Day 8.2: {}", part_2_solve(&forest));
}
