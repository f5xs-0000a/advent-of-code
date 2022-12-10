use std::io::BufRead as _;

struct CRT {
    pub x: isize,
    cursor: usize,
    screen: [bool; 40 * 6],
}

impl CRT {
    fn new() -> CRT {
        CRT {
            x: 1,
            cursor: 0,
            screen: [false; 6 * 40],
        }
    }

    fn get_cycle_number(&self) -> usize {
        self.cursor
    }

    fn step(&mut self) {
        let cursor = (self.cursor % 40) as isize;
        if self.x - 1 <= cursor && cursor <= self.x + 1 {
            self.screen[self.cursor] = true;
        }

        self.cursor += 1;
    }

    fn display(&self) {
        let mut line_buffer = String::with_capacity(40);
        for line in self.screen.chunks(40) {           
            line_buffer.clear();
            for pixel in line {
                let ch = match pixel {
                    false => "  ",
                    true => "##",
                };

                line_buffer += ch;
            }

            eprintln!("{}", line_buffer);
            line_buffer.clear();
        }
    }
}

fn main() {
    let cycle_markers = [20, 60, 100, 140, 180, 220];
    let mut marker_peek = cycle_markers.iter().peekable();
    let mut signal_strength = 0;
    let mut crt = CRT::new();

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
        let previous_x = crt.x;

        let mut words = buffer.split_whitespace();
        match words.next() {
            Some("noop") => crt.step(),
            Some("addx") => {
                crt.step();
                crt.step();
                crt.x += words
                    .next()
                    .and_then(|x| x.parse::<isize>().ok())
                    .unwrap();
            }
            _ => unimplemented!(),
        }

        if let Some(marker) = marker_peek.peek().cloned() {
            if *marker <= crt.get_cycle_number() {
                marker_peek.next();
                signal_strength += (*marker as isize) * previous_x;
            }
        }
    }

    eprintln!("Day 10.1: {}", signal_strength);
    eprintln!("Day 10.2:");
    crt.display();
}
