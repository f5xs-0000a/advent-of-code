use std::io::Read;

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut buffer)
        .expect("Unable to read from stdin");

    let mut nice_1 = 0;
    let mut nice_2 = 0;
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    for line in buffer.lines() {
        let mut vowels = 0;
        let mut consecutive = false;
        let mut naughty = false;
        let mut prev_characters = (None, None); // .0 <- .1 <- current
        let mut in_between = false;
        let mut letter_counter = [0; 26];

        for c in line.chars() {
            // check for vowels (part 1)
            if "aeiou".chars().any(|v| v == c) {
                vowels += 1;
            }
            
            // check for consecutives (part 1)
            if !consecutive {
                if let Some(pc) = prev_characters.1.as_ref() {
                    consecutive = *pc == c;
                }
            }

            if !in_between {
                if let Some(pc) = prev_characters.0.as_ref() {
                    in_between = *pc == c;
                }
            }

            if let Some(pc) = prev_characters.1.as_ref() {
                match *pc {
                    'a' => if c == 'b' {
                        naughty = true;
                    },
                    'c' => if c == 'd' {
                        naughty = true;
                    },
                    'p' => if c == 'q' {
                        naughty = true;
                    },
                    'x' => if c == 'y' {
                        naughty = true;
                    },
                    _ => {}
                }
            }

            // counter for two existing letters
            alphabet.chars()
                .zip(letter_counter.iter_mut())
                .any(|(a, lc)| {
                    if a == c {
                        *lc += 1;
                        true
                    }

                    else {
                        false
                    }
                });

            // perform push back
            prev_characters.0 = prev_characters.1;
            prev_characters.1 = Some(c);
        }

        if !naughty && vowels >= 3 && consecutive {
            nice_1 += 1;
        }

        let mut has_double_existing = false;
        // iterate through the list of letters that are valid, i.e. those that
        // exist twice
        let valid_letters = alphabet
            .chars()
            .zip(letter_counter.iter())
            .filter(|(_, lc)| **lc >= 2)
            .map(|(c, _)| c);
        'outer: for valid_char in valid_letters {
            let mut line_chars = line.chars();
            while let Some(ch) = line_chars.next() {
                if ch != valid_char {
                    continue;
                }

                // at this point, we've reached a valid character.
                let mut inner_iter = line_chars.clone();
                let next_char = match inner_iter.next() {
                    Some(c) => c,
                    None => break,
                };

                let mut second_inner_iter = inner_iter.clone();
                while second_inner_iter
                    .by_ref()
                    .filter(|c| *c == valid_char)
                    .next()
                    .is_some()
                {
                    if Some(next_char) == second_inner_iter.clone().next() {
                        has_double_existing = true;
                        break 'outer;
                    }
                }
            }
        }

        if has_double_existing && in_between {
            nice_2 += 1;
        }
    }

    eprintln!("Day 5.1: {}", nice_1);
    eprintln!("Day 5.2: {}", nice_2);
}
