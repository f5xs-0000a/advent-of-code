use std::io::BufRead as _;

fn problem_1(flags: &[String]) {
    let mut itered_lines = flags
        .iter()
        .map(|line| line.chars())
        .collect::<Vec<_>>();

    let mut most_common = vec![];

    // iterate through the characters
    'outer: loop {
        let mut ones = 0;
        let count = itered_lines.len();

        // count instances of ones
        for line in itered_lines.iter_mut() {
            match line.next() {
                None => break 'outer,
                Some('1') => ones += 1,
                Some('0') => {},
                _ => unreachable!(),
            }
        }

        if ones * 2 > count {
            most_common.push(1)
        }

        else {
            most_common.push(0)
        }
    }

    let mc = most_common
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| 2usize.pow(i as u32) * *x)
        .sum::<usize>();

    let lc = most_common
        .iter()
        .rev()
        .map(|x| match x {
            0 => 1,
            1 => 0,
            _ => unreachable!(),
        })
        .enumerate()
        .map(|(i, x)| 2usize.pow(i as u32) * x)
        .sum::<usize>();

    eprintln!("Day 3.1: {}", mc * lc);
}

fn main() {
    let flags = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    problem_1(&*flags);
    problem_2(&*flags);
}

fn problem_2(flags: &[String]) {
    let oxy = problem_2_inner(flags, true);
    let co2 = problem_2_inner(flags, false);

    eprintln!("Day 3.2: {}", oxy * co2);
}

fn problem_2_inner(flags: &[String], oxy_mode: bool) -> usize {
    let mut line_iters = flags
        .iter()
        .map(|line| line.chars().peekable())
        .collect::<Vec<_>>();
    let mut common_bits = vec![];

    // iterate through the characters
    'outer: loop {
        let mut ones = 0;
        let count = line_iters.len();

        if count == 1 {
            for c in line_iters.pop().unwrap() {
                let p = match c {
                    '0' => 0,
                    '1' => 1,
                    _ => unreachable!(),
                };

                common_bits.push(p);
            }

            break;
        }

        // count instances of ones
        for line in line_iters.iter_mut() {
            match line.peek() {
                None => break 'outer,
                Some('1') => ones += 1,
                Some('0') => {},
                _ => unreachable!(),
            }
        }

        if (oxy_mode) == (ones * 2 >= count) {
            line_iters = line_iters
                .into_iter()
                .filter_map(|mut i| (i.next() == Some('1')).then_some(i))
                .collect::<Vec<_>>();

            common_bits.push(1);
        }

        else {
            line_iters = line_iters
                .into_iter()
                .filter_map(|mut i| (i.next() == Some('0')).then_some(i))
                .collect::<Vec<_>>();

            common_bits.push(0);
        }
    }

    let val = common_bits
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, x)| 2usize.pow(i as u32) * x)
        .sum::<usize>();

    val
}
