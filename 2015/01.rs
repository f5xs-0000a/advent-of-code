use std::io::BufRead as _;

fn main() {
    let mut sum = 0i16;
    let mut count = 0;
    let mut basement = None;

    let lines = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.expect("Cannot split line"));

    for line in lines {
        for c in line.split_whitespace().flat_map(|s| s.chars()) {
            count += 1;
            match c {
                '(' => sum += 1,
                ')' => sum -= 1,
                u => panic!("Unexpected character: {}", u),
            }

            if sum == -1 && basement.is_none(){
                basement = Some(count);
            }
        }
    }

    eprintln!("Day 1.1: {}", sum);
    eprintln!("Day 1.2: {}", basement.unwrap());
}
