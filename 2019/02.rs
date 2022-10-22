use std::io::Read as _;

fn run_program(mut intcode: Box<[usize]>, noun: usize, verb: usize) -> usize {
    intcode[1] = noun;
    intcode[2] = verb;

    let mut cursor = 0;

    loop {
        match intcode[cursor] {
            1 => {
                let x_index = intcode[cursor + 1];
                let y_index = intcode[cursor + 2];
                let z_index = intcode[cursor + 3];

                let x = intcode[x_index];
                let y = intcode[y_index];

                intcode[z_index] = x + y;
            },

            2 => {
                let x_index = intcode[cursor + 1];
                let y_index = intcode[cursor + 2];
                let z_index = intcode[cursor + 3];

                let x = intcode[x_index];
                let y = intcode[y_index];

                intcode[z_index] = x * y;
            },

            99 => break,

            _ => unimplemented!(),
        }

        cursor += 4;
    }

    intcode[0]
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut buffer)
        .expect("Unable to read from stdin");

    let intcode = buffer
        .split(',')
        .flat_map(|s| s.split_whitespace())
        .map(|x| x.parse::<usize>().expect("Expected an unsigned integer."))
        .collect::<Vec<_>>();

    eprintln!(
        "Day 2.1: {}",
        run_program(intcode.clone().into_boxed_slice(), 12, 2),
    );

    for noun in 0 .. 100 {
        for verb in 0 .. 100 {
            let output = run_program(
                intcode.clone().into_boxed_slice(),
                noun,
                verb,
            );

            if output == 19690720 {
                eprintln!(
                    "Day 2.2: {}",
                    100 * noun + verb,
                );

                return
            }
        }
    }
}
