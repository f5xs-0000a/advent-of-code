use std::io::BufRead as _;
use std::collections::HashMap;
use std::num::Wrapping;
use std::collections::HashSet;

#[derive(Clone, Debug)]
enum Value {
    Constant(Wrapping<u16>),
    Variable(String),
}

impl Value {
    fn get(&self, wires: &HashMap<String, Wrapping<u16>>)
    -> Option<Wrapping<u16>> {
        use Value::*;

        match self {
            Constant(x) => Some(*x),
            Variable(x) => wires.get(x).cloned(),
        }
    }

    fn parse(word: &str) -> Value {
        use Value::*;

        match word.parse::<u16>() {
            Ok(num) => Constant(Wrapping(num)),
            Err(_) => Variable(word.to_string()),
        }
    }

    fn get_required_variable(&self) -> Option<&str> {
        use Value::*;

        match self {
            Variable(x) => Some(&x),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
enum Operation {
    And(Value, Value),
    Or(Value, Value),
    LShift(Value, Value),
    RShift(Value, Value),
    Not(Value),
    Val(Value),
}

impl Operation {
    fn perform(&self, wires: &HashMap<String, Wrapping<u16>>)
    -> Option<Wrapping<u16>> {
        use Operation::*;

        let retval = match self {
            And(x, y) => x.get(wires)? & y.get(wires)?,
            Or(x, y) => x.get(wires)? | y.get(wires)?,
            LShift(x, y) => x.get(wires)? << y.get(wires)?.0 as usize,
            RShift(x, y) => x.get(wires)? >> y.get(wires)?.0 as usize,
            Not(x) => !x.get(wires)?,
            Val(x) => x.get(wires)?,
        };
        Some(retval)
    }

    fn parse(words: &[&str]) -> Option<Operation> {
        match words[1] {
            "AND" => {
                let left = Value::parse(&words[0]);
                let right = Value::parse(&words[2]);
                return Some(Operation::And(left, right))       
            },
            "OR" => {
                let left = Value::parse(&words[0]);
                let right = Value::parse(&words[2]);
                return Some(Operation::Or(left, right))       
            },
            "LSHIFT" => {
                let left = Value::parse(&words[0]);
                let right = Value::parse(&words[2]);
                return Some(Operation::LShift(left, right))       
            },
            "RSHIFT" => {
                let left = Value::parse(&words[0]);
                let right = Value::parse(&words[2]);
                return Some(Operation::RShift(left, right))       
            },
            _ => {},
        }

        if words[0] == "NOT" {
            let val = Value::parse(&words[1]);
            return Some(Operation::Not(val));
        }

        Some(Operation::Val(Value::parse(&words[0])))
    }

    fn get_required_variables(&self) -> impl Iterator<Item = &str> {
        use Operation::*;

        match self {
            And(x, y) => x
                .get_required_variable()
                .into_iter()
                .chain(y.get_required_variable().into_iter()),
            Or(x, y) => x
                .get_required_variable()
                .into_iter()
                .chain(y.get_required_variable().into_iter()),
            LShift(x, y) => x
                .get_required_variable()
                .into_iter()
                .chain(y.get_required_variable().into_iter()),
            RShift(x, y) => x
                .get_required_variable()
                .into_iter()
                .chain(y.get_required_variable().into_iter()),
            Not(x) => x
                .get_required_variable()
                .into_iter()
                .chain(None.into_iter()),
            Val(x) => x
                .get_required_variable()
                .into_iter()
                .chain(None.into_iter()),
        }
    }
}

#[derive(Debug)]
struct Equation {
    operation: Operation,
    target: String,
}

impl Equation {
    fn evaluate_and_assign(&self, wires: &mut HashMap<String, Wrapping<u16>>) -> Option<()> {
        let value = self.operation.perform(wires)?;
        *wires.entry(self.target.clone()).or_insert(Wrapping(0)) = value;
        Some(())
    }

    fn parse(words: &[&str]) -> Option<Equation> {
        let operation = Operation::parse(words)?;
        let target = words.last()?.to_string();

        Some(Equation {
            operation,
            target,
        })
    }

    fn get_required_variables(&self) -> impl Iterator<Item = &str> {
        self.operation.get_required_variables()
    }

    fn get_target_variable(&self) -> &str {
        &self.target
    }
}

/// Reorganize instructions such that declarations come first and equations that
/// depend on what's been declared come next
fn reorganize_instructions(mut instructions: Vec<Equation>) -> Vec<Equation> {
    let mut reordered = Vec::with_capacity(instructions.len());
    let mut declared_variables = HashSet::with_capacity(instructions.len() * 2);
    let mut dump = Vec::with_capacity(instructions.len());

    while !instructions.is_empty() {
        for instruction in instructions.drain(..) {
            let all_vars_in_reordered = instruction
                .get_required_variables()
                .all(|var| declared_variables.contains(var));

            if all_vars_in_reordered {
                declared_variables.insert(instruction.get_target_variable().to_owned());
                reordered.push(instruction);
            }

            else {
                dump.push(instruction);
            }
        }

        core::mem::swap(&mut instructions, &mut dump);
    }

    reordered
}

fn main() {
    let mut instructions = vec![];
    let mut wires = HashMap::new();

    let mut buffer = String::new();
    let mut stdin = std::io::stdin().lock();
    while {
        buffer.clear();
        match stdin.read_line(&mut buffer) {
            Ok(0) => false,
            Err(_) => false,
            _ => true
        }
    } {
        let words = buffer.split_whitespace().collect::<Vec<_>>();
        let equation = match Equation::parse(&words) {
            Some(x) => x,
            None => {
                eprintln!("Cannot parse {}", buffer);
                continue;
            }
        };

        instructions.push(equation);
    }

    instructions = reorganize_instructions(instructions);

    for instruction in instructions.iter() {
        instruction.evaluate_and_assign(&mut wires);
    }

    eprintln!("Day 7.1: {}", wires["a"]);

    let overridden = Operation::Val(Value::Constant(wires["a"]));
    wires.clear();
    for mut instruction in instructions.into_iter() {
        if instruction.get_target_variable() == "b" {
            instruction.operation = overridden.clone();
        }

        instruction.evaluate_and_assign(&mut wires);
    }

    eprintln!("Day 7.2: {}", wires["a"]);
}
