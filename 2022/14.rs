use std::io::BufRead as _;

fn build_sandcastle_rock_base(paths: Vec<Vec<(usize, usize)>>)
-> Vec<Vec<bool>> {
    use std::cmp::min;
    use std::cmp::max;

    // find the maximum width and height
    let mut max_height = 0;
    for path in paths.iter() {
        for (_, y) in path.iter() {
            if *y > max_height {
                max_height = *y;
            }
        }
    }

    // after finding the maximum height, find out what the maximum width is.
    // consider a point. find out how far it extends as a pyramid going
    // downwards until it reaches the maximum height.
    let mut max_width = 0;
    for path in paths.iter() {
        for (x, y) in path.iter() {
            if *x <= max_height - y {
                panic!("Not enough width");
            }

            let farthest_x = x + (max_height - y);

            if farthest_x > max_width {
                max_width = farthest_x;
            }
        }
    }

    // the +2 is necessary for the part 2 of the problem
    // but I have absolutely no idea why the offset of +11. yet any lower would
    // make the code go out-of-bounds
    let mut map = vec![vec![false; max_height + 2]; max_width + 11];

    for path in paths.into_iter() {
        let mut trace = path.into_iter();
        let (mut prev_x, mut prev_y) = match trace.next() {
            Some((x, y)) => (x, y),
            None => continue,
        };
        
        // trace the path that will create the rock base
        for (x, y) in trace {
            // same x, different y
            if prev_x == x {
                for y_val in min(prev_y, y) ..= max(prev_y, y) {
                    map[x][y_val] = true;
                }
            }

            else { // if prev_y == y
                for x_val in min(prev_x, x) ..= max(prev_x, x) {
                    map[x_val][y] = true;
                }
            }

            prev_x = x;
            prev_y = y;
        }
    }

    map
}

fn drop_sand_until_criterion(
    mut sandcastle: Vec<Vec<bool>>,
    criterion: impl Fn(usize, usize) -> bool,
) -> usize {
    let mut sands_dropped = 0;

    // instead of letting the sand fall all the way from the start, why not just
    // keep track of the path that sand will fall from? with that, you can just
    // start adding sand from the end of the path. the path will also only
    // change at the end of the path so just update accordingly.
    let mut sand_path = Vec::with_capacity(sandcastle.len());
    let height = sandcastle.first().map(|x| x.len()).unwrap_or(0);
    sand_path.push((500, 0));

    loop {
        // let the sand fall
        let last_sand_path = match sand_path.last() {
            Some(x) => x,
            None => break,
        };

        let next_sand_fall = {
            // check if it will be out of bounds
            if last_sand_path.1 + 1 >= height {
                None
            }

            // down
            else if !sandcastle[last_sand_path.0][last_sand_path.1 + 1] {
                Some((last_sand_path.0, last_sand_path.1 + 1))
            }

            // down left
            else if !sandcastle[last_sand_path.0 - 1][last_sand_path.1 + 1] {
                Some((last_sand_path.0 - 1, last_sand_path.1 + 1))
            }

            // down right
            else if !sandcastle[last_sand_path.0 + 1][last_sand_path.1 + 1] {
                Some((last_sand_path.0 + 1, last_sand_path.1 + 1))
            }

            // totally occupied
            else {
                None
            }
        };

        match next_sand_fall {
            None => {
                // fill the current spot with sand
                sandcastle[last_sand_path.0][last_sand_path.1] = true;
                
                sands_dropped += 1;

                if criterion(last_sand_path.0, last_sand_path.1) {
                    return sands_dropped;
                }

                drop(last_sand_path);
                sand_path.pop();
            },
            Some(lsp) => {
                drop(last_sand_path);
                sand_path.push(lsp);
            },
        }
    }

    sands_dropped
}

fn main() {
    let mut paths = vec![];

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
        let mut path_string = vec![];

        for path in buffer.split(" -> ").flat_map(|x| x.split_whitespace()) {
            let mut coords = path
                .split(",")
                .map(|x| x.parse::<usize>().unwrap());
            path_string.push(
                (coords.next().unwrap(), coords.next().unwrap())
            );
        }

        paths.push(path_string);
    }

    let sand_castle = build_sandcastle_rock_base(paths);

    let sand_dropped = drop_sand_until_criterion(
        sand_castle.clone(),
        |_, y| y + 1 >= sand_castle.first().map(|x| x.len()).unwrap_or(0)
    );

    eprintln!("Day 14.1: {}", sand_dropped - 1);

    let sand_dropped = drop_sand_until_criterion(
        sand_castle,
        |x, y| (x, y) == (500, 0)
    );

    eprintln!("Day 14.2: {}", sand_dropped);
}
