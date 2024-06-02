use std::io::Read as _;

#[path = "intcode.rs"]
mod intcode;

fn run_program_emplacing(
    intcode: &mut [isize],
    noun: isize,
    verb: isize,
) -> isize {
    intcode[1] = noun;
    intcode[2] = verb;

    intcode::run_program(intcode, || unimplemented!(), |_| {});

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
        .map(|x| x.parse::<isize>().expect("Expected a signed integer."))
        .collect::<Vec<_>>();

    eprintln!(
        "Day 2.1: {}",
        run_program_emplacing(&mut intcode.clone(), 12, 2),
    );

    for noun in 0 .. 100 {
        for verb in 0 .. 100 {
            let output =
                run_program_emplacing(&mut intcode.clone(), noun, verb);

            if output == 19690720 {
                eprintln!("Day 2.2: {}", 100 * noun + verb,);

                return;
            }
        }
    }
}
