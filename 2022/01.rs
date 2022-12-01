use std::io::BufRead as _;

fn main() {
    let mut total_calories = vec![];

    let mut stdin_lock = std::io::stdin().lock();
    let mut buffer = String::new();
    let mut cur_elf_calories = 0;
    loop {
        buffer.clear();
        match stdin_lock.read_line(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(_) => {
                buffer.pop();
                match buffer.parse::<usize>() {
                    Ok(cal) => cur_elf_calories += cal,
                    Err(_) => {
                        total_calories.push(cur_elf_calories);
                        cur_elf_calories = 0;
                    },
                }
            },
            Err(_) => panic!("Unable to read from stdin"),
        }
    }

    total_calories.sort_unstable();
    eprintln!("Day 1.1: {}", total_calories.last().unwrap());
    eprintln!(
        "Day 1.2: {}",
        total_calories
            .iter()
            .skip(total_calories.len() - 3)
            .sum::<usize>()
    );
}
