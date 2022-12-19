use std::io::BufRead as _;
use std::collections::HashMap;

enum Marker {
    Start,
    Path,
    End,
}

fn height_value(c: char) -> (u8, Marker) {
    use Marker::*;

    match c {
        'a' => (0, Path),
        'b' => (1, Path),
        'c' => (2, Path),
        'd' => (3, Path),
        'e' => (4, Path),
        'f' => (5, Path),
        'g' => (6, Path),
        'h' => (7, Path),
        'i' => (8, Path),
        'j' => (9, Path),
        'k' => (10, Path),
        'l' => (11, Path),
        'm' => (12, Path),
        'n' => (13, Path),
        'o' => (14, Path),
        'p' => (15, Path),
        'q' => (16, Path),
        'r' => (17, Path),
        's' => (18, Path),
        't' => (19, Path),
        'u' => (20, Path),
        'v' => (21, Path),
        'w' => (22, Path),
        'x' => (23, Path),
        'y' => (24, Path),
        'z' => (25, Path),
        'S' => (0, Start),
        'E' => (26, End),
        _ => unimplemented!(),
    }
}

fn main() {
    let mut relief_map = vec![];
    let mut start = None;
    let mut end = None;

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
        let mut row_relief_map: Vec<u8> = Vec::with_capacity(
            relief_map.last().map(|row: &Vec<u8>| row.len()).unwrap_or(0)
        );

        for (col, c) in buffer.chars().enumerate() {
            if c == '\n' {
                break;
            }

            let (height, marker) = height_value(c);

            // update start and end
            match marker {
                Marker::Start => start = Some((relief_map.len(), col)),
                Marker::End => end = Some((relief_map.len(), col)),
                _ => {}
            }

            row_relief_map.push(height);
        }

        relief_map.push(row_relief_map);
    }

    let start = start.unwrap();
    let end = end.unwrap();

    dbg!(find_shortest_distance_length_from_start_to_finish(&relief_map, start, end));


}

fn find_shortest_distance_length_from_start_to_finish(
    relief_map: &[Vec<u8>],
    start: (usize, usize),
    end: (usize, usize)
) -> usize {
    // maybe this should be a set
    let mut traversed = HashMap::new();
    let mut current_breadth = HashMap::new();
    traversed.insert(start, 0);
    current_breadth.insert(start, 0);

    let height = relief_map.len();
    let width = relief_map.first().unwrap().len();

    let mut next_breadth = HashMap::new();
    loop {
        macro_rules! check_direction {
            (
                $BOUNDS_CHECK:expr,
                $NEW_TARGET:expr
            ) => {
                for (coords, step) in current_breadth.iter() {
                    // don't go out of bounds
                    if !$BOUNDS_CHECK(coords) { // coords.0 == 0 {
                        continue;
                    }

                    let target_coords = $NEW_TARGET(coords);

                    // don't go to what's already traversed
                    if traversed.contains_key(&target_coords) {
                        continue;
                    }

                    // check target coords if it can be traversed from current
                    // coords
                    if relief_map[coords.0][coords.1] + 1 < relief_map[target_coords.0][target_coords.1] {
                        continue;
                    }

                    next_breadth.insert(target_coords, step + 1);
                }
            }
        }

        // north
        check_direction!(
            |coords: &(usize, usize)| coords.0 > 0,
            |coords: &(usize, usize)| (coords.0 - 1, coords.1)
        );
        // east
        check_direction!(
            |coords: &(usize, usize)| coords.1 < width - 1,
            |coords: &(usize, usize)| (coords.0, coords.1 + 1)
        );
        // south
        check_direction!(
            |coords: &(usize, usize)| coords.0 < height - 1,
            |coords: &(usize, usize)| (coords.0 + 1, coords.1)
        );
        // west
        check_direction!(
            |coords: &(usize, usize)| coords.1 > 0,
            |coords: &(usize, usize)| (coords.0, coords.1 - 1)
        );

        if next_breadth.is_empty() {
            panic!();
        }

        current_breadth.clear();

        // clear next breadth and put them into current breadth and traversed
        for (k, v) in next_breadth.drain() {
            if k == end {
                return v;
            }

            current_breadth.insert(k, v);
            traversed.insert(k, v);
        }
    }
}
