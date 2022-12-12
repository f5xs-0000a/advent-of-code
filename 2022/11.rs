use std::io::BufRead as _;

#[derive(Clone)]
enum LValue {
    Constant(usize),
    Old,
}

impl LValue {
    fn evaluate(&self, old: usize) -> usize {
        use LValue::*;

        match self {
            Old => old,
            Constant(x) => *x,
        }
    }
}

#[derive(Clone)]
enum Operation {
    Add(LValue, LValue),
    Multiply(LValue, LValue),
}

impl Operation {
    fn evaluate(&self, old: usize) -> usize {
        use Operation::*;

        match self {
            Add(l, r) => l.evaluate(old) + r.evaluate(old),
            Multiply(l, r) => l.evaluate(old) * r.evaluate(old),
        }
    }
}

#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,

    divisibility_test: usize,
    true_test: usize,
    false_test: usize,

    inspections: usize,
}

impl Monkey {
    fn new(
        items: Vec<usize>,
        operation: Operation,
        divisibility_test: usize,
        true_test: usize,
        false_test: usize,
    ) -> Monkey {
        Monkey {
            items,
            operation,
            divisibility_test,
            true_test,
            false_test,
            inspections: 0,
        }
    }

    fn handle_items_and_throw(
        &mut self,
        new_inventories: &mut [Vec<usize>],
        divisor: usize,
        clock: usize,
    ) {
        self.inspections += self.items.len();

        for item in self.items.drain(..) {
            let new_worry = (self.operation.evaluate(item) / divisor) % clock;

            if new_worry % self.divisibility_test == 0 {
                new_inventories[self.true_test].push(new_worry);
            }

            else {
                new_inventories[self.false_test].push(new_worry);
            }
        }
    }
}

fn euclidean_algorithm(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

fn euclidean_algorithm_multiple(mut values: impl Iterator<Item = usize>)
-> Option<usize> {
    let mut gcd = match values.next() {
        Some(x) => x,
        None => return None,
    };

    for value in values {
        gcd = euclidean_algorithm(gcd, value);
    }

    Some(gcd)
}

fn least_common_multiple(values: impl Iterator<Item = usize>) -> Option<usize>{
    let mut multiple = 1;
    let values = values.inspect(|x| multiple *= x);
    euclidean_algorithm_multiple(values).map(|gcd| multiple / gcd)
}

fn do_monkey_business_with_buffer(
    monkeys: &mut [Monkey],
    buffer: &mut [Vec<usize>],
    rounds: usize,
    divisor: usize,
) {
    let clock = least_common_multiple(
        monkeys.iter().map(|m| m.divisibility_test)
    ).unwrap();

    for _ in 0 .. rounds {
        // do the monkey business
        for (i, monkey) in monkeys.iter_mut().enumerate() {
            // extend the inventory of the monkey with the contents of the
            // buffer
            monkey.items.extend(buffer[i].drain(..));
            monkey.handle_items_and_throw(buffer, divisor, clock);
        }
    }
}

fn do_monkey_business(monkeys: &mut [Monkey], rounds: usize, divisor: usize) {
    let mut buffer = vec![vec![]; monkeys.len()];
    do_monkey_business_with_buffer(monkeys, &mut buffer, rounds, divisor);
}

/*
fn parse_single_monkey() {
    
}

fn parse() -> Vec<Monkey> {
}
*/

fn main() {
    /*
    let mut monkeys = vec![
        Monkey::new(
            vec![79, 98],
            Operation::Multiply(LValue::Old, LValue::Constant(19)),
            23,
            2,
            3,
        ),
        Monkey::new(
            vec![54, 65, 75, 74],
            Operation::Add(LValue::Old, LValue::Constant(6)),
            19,
            2,
            0,
        ),
        Monkey::new(
            vec![79, 60, 97],
            Operation::Multiply(LValue::Old, LValue::Old),
            13,
            1,
            3,
        ),
        Monkey::new(
            vec![74],
            Operation::Add(LValue::Old, LValue::Constant(3)),
            17,
            0,
            1,
        ),
    ];
    */

    let mut monkeys = vec![
        Monkey::new(
            vec![72, 97],
            Operation::Multiply(LValue::Old, LValue::Constant(13)),
            19,
            5,
            6,
        ),
        Monkey::new(
            vec![55, 70, 90, 74, 95],
            Operation::Multiply(LValue::Old, LValue::Old),
            7,
            5,
            0,
        ),
        Monkey::new(
            vec![74, 97, 66, 57],
            Operation::Add(LValue::Old, LValue::Constant(6)),
            17,
            1,
            0,
        ),
        Monkey::new(
            vec![86, 54, 53],
            Operation::Add(LValue::Old, LValue::Constant(2)),
            13,
            1,
            2,
        ),
        Monkey::new(
            vec![50, 65, 78, 50, 62, 99],
            Operation::Add(LValue::Old, LValue::Constant(3)),
            11,
            3,
            7,
        ),
        Monkey::new(
            vec![90],
            Operation::Add(LValue::Old, LValue::Constant(4)),
            2,
            4,
            6,
        ),
        Monkey::new(
            vec![88, 92, 63, 94, 96, 82, 53, 53],
            Operation::Add(LValue::Old, LValue::Constant(8)),
            5,
            4,
            7,
        ),
        Monkey::new(
            vec![70, 60, 71, 69, 77, 70, 98],
            Operation::Multiply(LValue::Old, LValue::Constant(7)),
            3,
            2,
            3,
        ),
    ];
    
    let mut first_pass_monkeys = monkeys.clone();
    do_monkey_business(&mut *first_pass_monkeys, 20, 3);

    let mut inspections = monkeys
        .iter()
        .map(|m| m.inspections)
        .collect::<Vec<_>>();
    inspections.sort_unstable();
    dbg!(inspections.iter().rev().take(2).product::<usize>());

    do_monkey_business(&mut *monkeys, 10000, 1);

    let mut inspections = monkeys
        .iter()
        .map(|m| m.inspections)
        .collect::<Vec<_>>();
    inspections.sort_unstable();
    dbg!(inspections.iter().rev().take(2).product::<usize>());

    /*
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
    }
    */
}
