use std::io::Read as _;
use std::collections::HashSet;

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut buffer)
        .expect("Unable to read from stdin");

    // what's interesting about this problem is that the encoded pathway string
    // towards a tile is an Abelian group, with concatenation as the group
    // operation. that means the string "eww" is equal to the strings "wew" and
    // "wwe" due to the commutative property of Abelian groups. we also need the
    // inverse property of groups. we can have the simplest inverse units, e.g.
    // "ne" and "sw", "e" and "w", and "nw" and "se" are inverses of each other.
    // therefore, for example, if you have a string of "ew", it's equal to an
    // empty string.
    //
    // with these out of the way, you can probably liken the properties above to
    // vector addition, which is also an Abelian group. we can just count how
    // further east, northeast, and northwest a tile is by incrementing or
    // decrementing one of the three by one given the respective token. from the
    // net sum of those three, we can determine the coordinates of the tile

    // there are multiple ways to encode hex tiling coordinates. we'll be
    // choosing one from three in this post:
    // https://math.stackexchange.com/questions/2254655
    // i prefer the axial coordinates.
    let coord_iter = buffer
        .lines()
        .map(|line| {
            let mut east = 0;
            let mut northeast = 0;
            let mut northwest = 0;

            let mut prev_char = None;
            for ch in line.chars() {
                match (ch, &prev_char) {
                    ('n', Some('n')) => unreachable!(),
                    ('s', Some('n')) => unreachable!(),
                    ('e', Some('n')) => {
                        northeast += 1;
                        prev_char = None;
                    },
                    ('w', Some('n')) => {
                        northwest += 1;
                        prev_char = None;
                    },
                    ('n', None) => prev_char = Some('n'),

                    ('n', Some('s')) => unreachable!(),
                    ('s', Some('s')) => unreachable!(),
                    ('e', Some('s')) => {
                        northwest -= 1;
                        prev_char = None;
                    },
                    ('w', Some('s')) => {
                        northeast -= 1;
                        prev_char = None;
                    },
                    ('s', None) => prev_char = Some('s'),

                    ('w', None) => east -= 1,
                    ('e', None) => east += 1,

                    _ => unreachable!(),
                }
            }

            (east, northeast, northwest)
        })
        .map(|(east, northeast, northwest)| {
            let x = east + northeast;
            let y = -northeast - northwest;
            (x, y)
        });

    let mut black_tiles = HashSet::new();
    for coords in coord_iter {
        if !black_tiles.remove(&coords) {
            black_tiles.insert(coords);
        }
    }

    eprintln!("Day 24.1: {}", black_tiles.len());

    for _ in 0 .. 100 {
        apply_cellular_automata(&mut black_tiles);
    }

    eprintln!("Day 24.2: {}", black_tiles.len());
}

fn find_adjacents((x, y): (isize, isize))
-> impl Iterator<Item = (isize, isize)> {
    vec![
        (x + 1, y),
        (x, y + 1),
        (x - 1, y + 1),
        (x - 1, y),
        (x, y - 1),
        (x + 1, y - 1)
    ].into_iter()
}

fn apply_cellular_automata(black_tiles: &mut HashSet<(isize, isize)>) {
    let mut white_to_black = HashSet::new();
    let mut black_to_white = HashSet::new();

    let mut probed_white_coords = HashSet::new();

    // black to white is easy.
    for black_coords in black_tiles.iter() {
        let mut black_adjacents = 0;
        for coords in find_adjacents(*black_coords) {
            // if black
            if black_tiles.contains(&coords) {
                black_adjacents += 1;
            }

            // if white
            else {
                probed_white_coords.insert(coords);
            }
        }

        if black_adjacents == 0 || black_adjacents > 2 {
            black_to_white.insert(*black_coords);
        }
    }

    // white to black is hard.
    // since you have infinitely many white tiles that span in all directions
    // and you only care about those that are adjacent to black tiles. you're
    // also not allowed to count a tile twice which can be a problem if you're
    // not careful
    for white_coords in probed_white_coords.iter() {
        let black_adjacents = find_adjacents(*white_coords)
            .filter(|coords| black_tiles.contains(coords))
            .count();

        if black_adjacents == 2 {
            white_to_black.insert(*white_coords);
        }
    }

    for coords in black_to_white.into_iter() {
        assert!(black_tiles.remove(&coords));
    }

    for coords in white_to_black.into_iter() {
        assert!(black_tiles.insert(coords));
    }
}
