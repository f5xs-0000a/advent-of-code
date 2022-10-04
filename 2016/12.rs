use std::io::Read as _;

#[derive(Clone)]
enum RValue {
    Value(i64),
    Reg(usize),
}

#[derive(Clone)]
enum Instruction {
    CPY(RValue, usize),
    INC(usize),
    DEC(usize),
    JNZ(RValue, RValue),
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut buffer)
        .expect("Unable to read from stdin");

    let mut instructions = vec![];
    for line in buffer.lines() {
        let mut split = line.split_whitespace();
        let instruction = match split.next() {
            Some("cpy") => {
                // see if the copied value is a variable
                let l = split
                    .next()
                    .expect("Expected rvalue for cpy instruction");
                let l = match (l.parse::<i64>(), l) {
                    (Ok(l), _) => RValue::Value(l),
                    (_, "a") => RValue::Reg(0),
                    (_, "b") => RValue::Reg(1),
                    (_, "c") => RValue::Reg(2),
                    (_, "d") => RValue::Reg(3),
                    _ => panic!("Unexpected register"),
                };

                let r = split
                    .next()
                    .expect("Expected rvalue for jnz instruction");
                let r = match r {
                    "a" => 0,
                    "b" => 1,
                    "c" => 2,
                    "d" => 3,
                    _ => panic!("Unexpected register"),
                };

                Instruction::CPY(l, r)
            },

            Some("inc") => {
                let r = split
                    .next()
                    .expect("Expected rvalue for jnz instruction");
                let r = match r {
                    "a" => 0,
                    "b" => 1,
                    "c" => 2,
                    "d" => 3,
                    _ => panic!("Unexpected register")
                };
                Instruction::INC(r)
            },
            Some("dec") => {
                let r = split
                    .next()
                    .expect("Expected rvalue for jnz instruction");
                let r = match r {
                    "a" => 0,
                    "b" => 1,
                    "c" => 2,
                    "d" => 3,
                    _ => panic!("Unexpected register"),
                };
                Instruction::DEC(r)
            },
            Some("jnz") => {
                let l = split
                    .next()
                    .expect("Expected rvalue for jnz instruction");
                let l = match (l.parse::<i64>(), l) {
                    (Ok(l), _) => RValue::Value(l),
                    (_, "a") => RValue::Reg(0),
                    (_, "b") => RValue::Reg(1),
                    (_, "c") => RValue::Reg(2),
                    (_, "d") => RValue::Reg(3),
                    _ => panic!("Unexpected register"),
                };

                let r = split
                    .next()
                    .expect("Expected rvalue for jnz instruction");
                let r = match (r.parse::<i64>(), r) {
                    (Ok(l), _) => RValue::Value(l),
                    (_, "a") => RValue::Reg(0),
                    (_, "b") => RValue::Reg(1),
                    (_, "c") => RValue::Reg(2),
                    (_, "d") => RValue::Reg(3),
                    _ => panic!("Unexpected register"),
                };
            
                Instruction::JNZ(l, r)
            },
            e => panic!("Unexpected instruction: {:?}", e),
        };

        instructions.push(instruction);
    }

    let a = perform_instructions_given_state_returning_reg_a(
        &instructions,
        [0; 4],
    );
    eprintln!("Day 12.1: {}", a);

    let b = perform_instructions_given_state_returning_reg_a(
        &instructions,
        [0, 0, 1, 0],
    );
    eprintln!("Day 12.2: {}", b);
}

fn perform_instructions_given_state_returning_reg_a(
    instructions: &[Instruction],
    mut registers: [i64; 4],
) -> i64 {
    let mut cursor = 0usize;

    while cursor < instructions.len() {
        use Instruction::*;
        use RValue::*;

        let mut jump_amount = 1i64;

        // get the instruction
        match instructions.get(cursor as usize).unwrap() {
            CPY(v, r) => {
                let value = match v {
                    Value(v) => v,
                    Reg(v) => registers.get(*v).unwrap(),
                };

                *registers.get_mut(*r).unwrap() = *value;
            },
            
            INC(v) => *registers.get_mut(*v).unwrap() += 1,
            DEC(v) => *registers.get_mut(*v).unwrap() -= 1,
            
            JNZ(v, j) => {
                let v = match v {
                    Value(v) => v,
                    Reg(v) => registers.get(*v).unwrap(),
                };

                if *v != 0 {
                    let j = match j {
                        Value(v) => v,
                        Reg(v) => registers.get(*v).unwrap(),
                    };

                    jump_amount = *j;
                }
            },
        }

        if jump_amount < 0 {
            cursor -= (-jump_amount) as usize;
        }

        else {
            cursor += jump_amount as usize;
        }
    }

    registers[0]
}
