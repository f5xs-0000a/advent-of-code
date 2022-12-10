use std::io::BufRead as _;

fn main() {
    let mut cycle_number = 0;
    let mut x = 1;
    let cycle_markers = [20, 60, 100, 140, 180, 220];
    let mut marker_peek = cycle_markers.iter().peekable();
    let mut signal_strength = 0;

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
        let previous_x = x;

        let mut words = buffer.split_whitespace();
        match words.next() {
            Some("noop") => cycle_number += 1,
            Some("addx") => {
                cycle_number += 2;
                x += words
                    .next()
                    .and_then(|x| x.parse::<isize>().ok())
                    .unwrap();
            }
            _ => unimplemented!(),
        }

        if let Some(marker) = marker_peek.peek().cloned() {
            if *marker <= cycle_number {
                marker_peek.next();
                signal_strength += marker * previous_x;
            }
        }
    }

    dbg!(signal_strength);
}
