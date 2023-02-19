use std::io::BufRead as _;

fn shorten(mut polymer: String) -> String {
    let mut shortened = String::new();

    loop {
        let mut polymer_chars = polymer.chars();
        let mut prev_ch = match polymer_chars.next() {
            Some(x) => x,
            None => return "".to_owned()
        };
        let mut add_prev_ch = true;

        loop {
            let next_ch = match polymer_chars.next() {
                Some('\n') => break,
                None => break,
                Some(x) => x,
            };

            // skip to the next iteration
            if (next_ch.to_uppercase().next().unwrap() == prev_ch
                || prev_ch.to_uppercase().next().unwrap() == next_ch)
                && next_ch != prev_ch
            {
                add_prev_ch = false;
                for ch in polymer_chars {
                    shortened.push(ch);
                }
                break;
            }

            shortened.push(prev_ch);
            prev_ch = next_ch;
        }

        if add_prev_ch {
            shortened.push(prev_ch);
        }

        if polymer.len() == shortened.len() {
            return shortened;
        }

        core::mem::swap(&mut polymer, &mut shortened);
        shortened.clear();
    }
}

fn main() {
    let mut buffer = String::new();
    let mut stdin = std::io::stdin().lock();
    stdin.read_line(&mut buffer).unwrap();

    let line = buffer.split_whitespace().next().unwrap().to_owned();
    eprintln!("Day 5.1: {}", shorten(line.clone()).len());

    let smallest = ('a' ..= 'z')
        .zip('A' ..= 'Z')
        .map(|(small, big)| line.chars().filter(|x| *x != small && *x !=  big).collect::<String>())
        .map(|new_string| shorten(new_string).len())
        .min();

    eprintln!("Day 5.2: {}", &smallest);
}
