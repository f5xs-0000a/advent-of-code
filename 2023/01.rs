use std::io::BufRead as _;
use std::collections::VecDeque;

fn main() {
    let stdin_lock = std::io::stdin().lock();

    let mut first_sum = 0;
    let mut first_pass_inspect = |l: &str| {
        let mut chars = l
            .chars()
            .filter_map(|c| c.to_digit(10));

        let first_digit = chars.next().unwrap_or(0);
        let last_digit = chars.last().unwrap_or(first_digit);

        first_sum += first_digit * 10 + last_digit;
    };

    let mut second_sum = 0;
    let mut second_pass_inspect = |l: &str| {
        let mut deque = VecDeque::with_capacity(6);

        let mut first_num = None;
        let mut last_num = None;

        for ch in l.chars() {
            // push the character in front
            deque.push_back(ch);

            // remove the character at the back if the deque is full
            while 5 < deque.len() {
                deque.pop_front();
            }

            let mut this_num = None;

            // try to check if the character is a number first
            if let Some(num) = ch.to_digit(10) {
                this_num = Some(num);
            }

            // if not, see if we can match the last n characters
            if 5 <= deque.len() {
                this_num = match deque.range(deque.len() - 5 ..).cloned() {
                    x if x.clone().eq("three".chars()) => Some(3),
                    x if x.clone().eq("seven".chars()) => Some(7),
                    x if x.clone().eq("eight".chars()) => Some(8),
                    _ => this_num,
                };
            }

            if 4 <= deque.len() {
                this_num = match deque.range(deque.len() - 4 ..).cloned() {
                    x if x.clone().eq("zero".chars()) => Some(0),
                    x if x.clone().eq("four".chars()) => Some(4),
                    x if x.clone().eq("five".chars()) => Some(5),
                    x if x.clone().eq("nine".chars()) => Some(9),
                    _ => this_num,
                };
            }

            if 3 <= deque.len() {
                this_num = match deque.range(deque.len() - 3 ..).cloned() {
                    x if x.clone().eq("one".chars()) => Some(1),
                    x if x.clone().eq("two".chars()) => Some(2),
                    x if x.clone().eq("six".chars()) => Some(6),
                    _ => this_num,
                };
            }

            if let Some(num) = this_num {
                if first_num.is_none() {
                    first_num = Some(num);
                }

                else {
                    last_num = Some(num);
                }
            }
        }

        second_sum += match (first_num.unwrap(), last_num) {
            (f, Some(l)) => f * 10 + l,
            (f, None) => f * 10 + f,
        };
    };

    stdin_lock.lines()
        .map(|l| l.unwrap())
        // first pass inspection
        .inspect(|l| first_pass_inspect(l))
        .inspect(|l| second_pass_inspect(l))
        .for_each(|_| {});

    eprintln!("Day 1.1: {}\nDay 1.2: {}", first_sum, second_sum);
}
