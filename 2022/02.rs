use std::io::BufRead as _;

#[derive(Clone, Copy, Debug)]
enum RPS {
    Rock     = 0isize,
    Paper    = 1,
    Scissors = 2,
}

impl RPS {
    fn from_value(x: isize) -> RPS {
        use RPS::*;

        match x {
            0 => Rock,
            1 => Paper,
            2 => Scissors,
            _ => unreachable!(),
        }
    }

    fn battle(&self, other: &RPS) -> Option<bool> {
        match (*self) as isize - (*other) as isize {
            0 => None,
            1 | -2 => Some(true),
            2 | -1 => Some(false),
            _ => unreachable!(),
        }
    }


    fn cook(&self, win: Option<bool>) -> RPS {
        let addend = match win {
            None => 0,
            Some(true) => 1,
            Some(false) => -1,
        };

        RPS::from_value((*self as isize + addend + 3) % 3)
    }

    fn value(&self) -> usize {
        use RPS::*;

        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

fn get_win_value(win: Option<bool>) -> usize {
    match win {
        Some(true) => 6,
        None => 3,
        Some(false) => 0,
    }
}

fn main() {
    use RPS::*;

    let mut part_1_value = 0;
    let mut part_2_value = 0;

    let mut stdin = std::io::stdin().lock();
    let mut buffer = String::new();
    while {
        buffer.clear();
        match stdin.read_line(&mut buffer) {
            Ok(0) => false,
            Err(_) => false,
            _ => true
        }
    } {
        let mut chars = buffer.chars();
        let opponent = match chars.next() {
            Some('A') => Rock,
            Some('B') => Paper,
            Some('C') => Scissors,
            _ => unimplemented!(),
        };

        // drop the space
        chars.next();

        // your move in part 1
        // your win rule in part 2
        let (you_1, win_2) = match chars.next() {
            Some('X') => (Rock, Some(false)),
            Some('Y') => (Paper, None),
            Some('Z') => (Scissors, Some(true)),
            _ => unimplemented!(),
        };

        // your win rule in part 2
        let win_1 = you_1.battle(&opponent);

        // your move in part 2
        let you_2 = opponent.cook(win_2);

        part_1_value += you_1.value() + get_win_value(win_1);
        part_2_value += you_2.value() + get_win_value(win_2);
    }

    eprintln!("Day 2.1: {}", part_1_value);
    eprintln!("Day 2.2: {}", part_2_value);
}
