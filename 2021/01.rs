use std::io::BufRead as _;
use std::str::FromStr as _;

pub fn main() {
    let depths = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.expect("Expected line");
            usize::from_str(&line).expect("Cannot read line")
        })
        .collect::<Vec<_>>();

    let ans1 = depths.windows(2).filter(|w| w[1] > w[0]).count();
    let ans2 = depths
        .windows(3)
        .map(|w| w.iter().sum::<usize>())
        .scan(None, |prev, now| {
            let retval = match prev.as_ref() {
                Some(p) => *p < now,
                None => false,
            };

            *prev = Some(now);

            Some(retval)
        })
        .filter(|x| *x)
        .count();

    eprintln!("Day 1.1: {}", ans1);
    eprintln!("Day 1.2: {}", ans2);
}
