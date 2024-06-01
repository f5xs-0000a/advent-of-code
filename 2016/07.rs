use std::{
    collections::{
        HashSet,
        VecDeque,
    },
    io::BufRead as _,
};

fn test_tls<'a>(s: impl Iterator<Item = &'a str>) -> bool {
    let mut scope = VecDeque::with_capacity(5);

    let mut has_found = false;

    // skip every other strings
    for (idx, string) in s.enumerate() {
        scope.clear();

        for ch in string.chars() {
            if scope.len() == 4 {
                scope.pop_back();
            }

            scope.push_front(ch);

            if scope.len() == 4 {
                if scope[0] == scope[3]
                    && scope[1] == scope[2]
                    && scope[0] != scope[2]
                {
                    if idx % 2 == 0 {
                        has_found = true;
                    }
                    else {
                        return false;
                    }
                }
            }
        }
    }

    has_found
}

fn test_ssl<'a>(s: impl Iterator<Item = &'a str>) -> bool {
    let mut scope = VecDeque::with_capacity(4);

    let mut inner_candidates = HashSet::new();
    let mut outer_candidates = HashSet::new();

    for (idx, string) in s.enumerate() {
        let is_outer = idx % 2 == 0; // test parity

        scope.clear();

        for ch in string.chars() {
            let mut should_push = false;

            if scope.len() == 3 {
                scope.pop_back();
            }

            scope.push_front(ch);

            if scope.len() == 3 {
                if scope[0] == scope[2] && scope[0] != scope[1] {
                    should_push = true;
                }
            };

            if should_push {
                if is_outer {
                    outer_candidates.insert((scope[0], scope[1]));
                }
                else {
                    inner_candidates.insert((scope[0], scope[1]));
                }
            }
        }
    }

    // test each of the outers and inners for matches
    for outer in outer_candidates.into_iter() {
        for inner in inner_candidates.iter() {
            if outer.0 == inner.1 && outer.1 == inner.0 {
                return true;
            }
        }
    }

    false
}

fn main() {
    let mut tls = 0;
    let mut ssl = 0;

    let mut buffer = String::new();
    let mut stdin = std::io::stdin().lock();
    while {
        buffer.clear();
        match stdin.read_line(&mut buffer) {
            Ok(0) => false,
            Err(_) => false,
            _ => true,
        }
    } {
        // split the string into pieces
        let splits = buffer.split(['[', ']']);

        if test_tls(splits.clone()) {
            tls += 1;
        }

        if test_ssl(splits.clone()) {
            ssl += 1;
        }
    }

    eprintln!("Day 7.1: {}", tls);
    eprintln!("Day 7.2: {}", ssl);
}
