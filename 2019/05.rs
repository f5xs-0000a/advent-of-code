use std::io::Read as _;
use std::io::BufRead as _;

enum Mode {
    Immediate,
    Position,
}

struct Instruction {
    intcode: isize,
    r1: Mode,
    r2: Mode,
    _r3: Mode,
}

impl Instruction {
    fn new(base: isize) -> Instruction {
        use Mode::*;

        let intcode = base % 100;

        let r1 = match (base / 100) % 10 {
            0 => Position,
            1 => Immediate,
            _ => unimplemented!(),
        };

        let r2 = match (base / 1000) % 10 {
            0 => Position,
            1 => Immediate,
            _ => unimplemented!(),
        };

        let r3 = match (base / 10000) % 10 {
            0 => Position,
            1 => Immediate,
            _ => unimplemented!(),
        };

        Instruction {
            intcode,
            r1,
            r2,
            _r3: r3,
        }
    }
}

fn run_program(mut intcode: Box<[isize]>) -> String {
    use Mode::*;

    let mut cursor = 0;
    let mut last_output = String::new();
    loop {
        let current = Instruction::new(intcode[cursor]);

        match current.intcode {
            99 => break,

            1 => {
                let x = match current.r1 {
                    Immediate => intcode[cursor + 1],
                    Position => {
                        let index = intcode[cursor + 1];
                        intcode[index as usize]
                    },
                };
                let y = match current.r2 {
                    Immediate => intcode[cursor + 2],
                    Position => {
                        let index = intcode[cursor + 2];
                        intcode[index as usize]
                    },
                };
                let z_index = intcode[cursor + 3];

                intcode[z_index as usize] = x + y;
                cursor += 4;
            },

            2 => {
                let x = match current.r1 {
                    Immediate => intcode[cursor + 1],
                    Position => {
                        let index = intcode[cursor + 1];
                        intcode[index as usize]
                    },
                };
                let y = match current.r2 {
                    Immediate => intcode[cursor + 2],
                    Position => {
                        let index = intcode[cursor + 2];
                        intcode[index as usize]
                    },
                };
                let z_index = intcode[cursor + 3];

                intcode[z_index as usize] = x * y;
                cursor += 4;
            },

            3 => {
                eprint!("The program asks for an input\n>>> ");

                let index = intcode[cursor + 1];
                let mut buffer = String::new();
                std::io::stdin().read_line(&mut buffer).unwrap();
                let value = buffer
                    .split_whitespace()
                    .next()
                    .expect("Expected value")
                    .parse::<isize>()
                    .expect("Expected an integer");

                intcode[index as usize] = value;
                cursor += 2;
            },

            4 => {
                let x = match current.r1 {
                    Immediate => intcode[cursor + 1],
                    Position => {
                        let index = intcode[cursor + 1];
                        intcode[index as usize]
                    },
                };
                println!("{}", x);
                last_output = format!("{}", x);

                cursor += 2;
            },

            5 => {
                let x = match current.r1 {
                    Immediate => intcode[cursor + 1],
                    Position => {
                        let index = intcode[cursor + 1];
                        intcode[index as usize]
                    },
                };
                let y = match current.r2 {
                    Immediate => intcode[cursor + 2],
                    Position => {
                        let index = intcode[cursor + 2];
                        intcode[index as usize]
                    },
                };

                if x != 0 {
                    cursor = y as usize;
                }

                else {
                    cursor += 3;
                }
            },

            6 => {
                let x = match current.r1 {
                    Immediate => intcode[cursor + 1],
                    Position => {
                        let index = intcode[cursor + 1];
                        intcode[index as usize]
                    },
                };
                let y = match current.r2 {
                    Immediate => intcode[cursor + 2],
                    Position => {
                        let index = intcode[cursor + 2];
                        intcode[index as usize]
                    },
                };

                if x == 0 {
                    cursor = y as usize;
                }

                else {
                    cursor += 3;
                }
            },

            7 => {
                let x = match current.r1 {
                    Immediate => intcode[cursor + 1],
                    Position => {
                        let index = intcode[cursor + 1];
                        intcode[index as usize]
                    },
                };
                let y = match current.r2 {
                    Immediate => intcode[cursor + 2],
                    Position => {
                        let index = intcode[cursor + 2];
                        intcode[index as usize]
                    },
                };
                let z_index = intcode[cursor + 3];

                intcode[z_index as usize] = match x < y {
                    true => 1,
                    false => 0,
                };

                cursor += 4;
            },

            8 => {
                let x = match current.r1 {
                    Immediate => intcode[cursor + 1],
                    Position => {
                        let index = intcode[cursor + 1];
                        intcode[index as usize]
                    },
                };
                let y = match current.r2 {
                    Immediate => intcode[cursor + 2],
                    Position => {
                        let index = intcode[cursor + 2];
                        intcode[index as usize]
                    },
                };
                let z_index = intcode[cursor + 3];

                intcode[z_index as usize] = match x == y {
                    true => 1,
                    false => 0,
                };

                cursor += 4;
            },

            x => {
                panic!("Unimplemented opcode {}", x);
            }
        }
    }

    last_output
}

fn main() {
    eprintln!("Paste your intcode here.");
    eprintln!("NOTE: This program is interactive. \
               It will pause as it will read an input from the user.");
    eprint!("Enter your puzzle input\n>>> ");

    let mut buffer = String::new();
    std::io::stdin()
        .lock()
        .read_line(&mut buffer)
        .expect("Unable to read from stdin");

    let intcode = buffer
        .split(',')
        .flat_map(|s| s.split_whitespace())
        .map(|x| x.parse::<isize>().expect("Expected an integer."))
        .collect::<Vec<_>>();

    dbg!(run_program(intcode.clone().into_boxed_slice()));

    /*
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
    */
}
