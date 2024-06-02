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
        use self::Mode::*;

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

/// Runs an intcode program.
pub fn run_program<'a>(
    intcode: &mut [isize],
    mut stdin: impl FnMut() -> isize,
    mut stdout: impl FnMut(isize),
) {
    use self::Mode::*;

    let mut cursor = 0;

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
                let index = intcode[cursor + 1];
                intcode[index as usize] = stdin();
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
                stdout(x);

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
            },
        }
    }
}

pub fn run_ascii_program(
    intcode: &mut [isize],
    mut stdin: impl FnMut(&mut String),
    stdout: impl Fn(&str),
) {
    let mut stdout_buffer = String::new();
    let mut stdin_buffer = String::new();
    let mut stdin_index = 0;

    let write_to_stdout = |s: isize| {
        use std::convert::TryInto as _;

        if s == 10 {
            stdout(&*stdout_buffer);
            stdout_buffer.clear();
            return;
        }

        match s.try_into().ok().and_then(|s| char::from_u32(s)) {
            Some(c) => stdout_buffer.push(c),
            None => panic!("unable to convert number {} into ascii", s),
        }
    };

    let read_from_stdin = || {
        loop {
            // return the byte at that index, then increment pointer
            if let Some(x) = stdin_buffer.as_bytes().get(stdin_index) {
                stdin_index += 1;
                return *x as isize;
            }
            // refresh the buffer
            else {
                stdin_buffer.clear();
                stdin(&mut stdin_buffer);
                stdin_index = 0;
            }
        }
    };

    run_program(intcode, read_from_stdin, write_to_stdout);
}

pub fn generate_intcode(s: &str) -> Vec<isize> {
    let mut intcode = s
        .split(',')
        .flat_map(|s| s.split_whitespace())
        .map(|x| x.parse::<isize>().expect("Expected an integer."))
        .collect::<Vec<_>>();

    intcode.shrink_to_fit();
    intcode
}
