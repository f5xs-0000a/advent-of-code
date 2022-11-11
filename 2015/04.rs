extern crate md5;

use std::io::BufRead as _;

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .lock()
        .read_line(&mut buffer)
        .expect("Unable to read from stdin");
    buffer.pop();

    let mut i = 0;
    loop {
        let pre_hash = format!("{}{}", buffer, i);
        let digest = md5::compute(&pre_hash);

        // 240 (128 + 64 + 32 + 16) to mask out the last four bits; the last
        // four bits make up the sixth hex digit
        if digest[0] == 0 && digest[1] == 0 && digest[2] & 240 == 0 {
            break;
        }

        i += 1;
    }

    eprintln!("Day 4.1: {}", i);

    loop {
        let pre_hash = format!("{}{}", buffer, i);
        let digest = md5::compute(&pre_hash);

        if digest[0] == 0 && digest[1] == 0 && digest[2] == 0 {
            break;
        }

        i += 1;
    }

    eprintln!("Day 4.2: {}", i);
}
