use std::iter::Peekable;
use std::cmp::Ordering;
use std::io::BufRead as _;

#[derive(Clone, PartialOrd, Eq)]
enum ValueOrList {
    Value(u8),
    List(Vec<ValueOrList>),
}

impl ValueOrList {
    fn parse(char_iter: &mut Peekable<impl Iterator<Item = char>>)
    -> Result<ValueOrList, ()> {
        // this is only an opening parse, not yet a confirmation parse
        match char_iter.peek() {
            Some('[') => {
                char_iter.next();
                ValueOrList::parse_list(char_iter)
            },
            Some(x) if ('0' ..= '9').contains(x)
                => Ok(ValueOrList::Value(ValueOrList::parse_num(char_iter))),
            _ => Err(()),
        }
    }

    fn parse_list(char_iter: &mut Peekable<impl Iterator<Item = char>>)
    -> Result<ValueOrList, ()> {
        let mut elements = vec![];
        loop {
            match char_iter.peek() {
                Some(']') => {
                    char_iter.next();
                    return Ok(ValueOrList::List(elements));
                },
                Some(',') => { char_iter.next(); },
                None => return Err(()),
                _ => elements.push(ValueOrList::parse(char_iter)?),
            }
        }
    }

    fn parse_num(char_iter: &mut Peekable<impl Iterator<Item = char>>) -> u8 {
        let mut value = 0;
        loop {
            match char_iter.peek() {
                Some(x) if ('0' ..= '9').contains(x) => {
                    value = value * 10 + x.to_digit(10).unwrap() as u8;
                    char_iter.next();
                },
                _ => return value,
            }
        }
    }
}

impl Ord for ValueOrList {
    fn cmp(&self, other: &Self) -> Ordering {
        use ValueOrList::*;

        match (self, other) {
            (Value(x), Value(y)) => x.cmp(y),
            (Value(x), List(_)) => List(vec![Value(*x)]).cmp(other),
            (List(_), Value(y)) => self.cmp(&List(vec![Value(*y)])),
            (List(x), List(y)) => {
                let mut x_iter = x.iter();
                let mut y_iter = y.iter();

                loop {
                    match (x_iter.next(), y_iter.next()) {
                        (Some(x), Some(y)) => {
                            let cmp = x.cmp(y);
                            if cmp.is_ne() {
                                return cmp;
                            }
                        },

                        (None, None) => return Ordering::Equal,
                        (None, _) => return Ordering::Less,
                        (_, None) => return Ordering::Greater,
                    }
                }
            },
        }
    }
}

impl PartialEq for ValueOrList {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

/*
fn mergesort<T>(vec: Vec<T>) -> Vec<T>
where T: Ord {
    let mut target = 
}

fn mergesort_inner() {
}
*/

fn main() {
    let mut signals = vec![];

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
        match ValueOrList::parse(&mut buffer.chars().peekable()) {
            Ok(x) => signals.push(x),
            Err(_) => {},
        }
    }

    let correct_order = signals
        .chunks(2)
        .enumerate()
        .filter(|(_, pair)| pair[0].cmp(&pair[1]).is_le())
        .map(|(idx, _)| idx + 1)
        .sum::<usize>();

    eprintln!("Day 13.2: {}", correct_order);

    let divider_1 = ValueOrList::parse(&mut "[[2]]".chars().peekable())
        .unwrap();
    let divider_2 = ValueOrList::parse(&mut "[[6]]".chars().peekable())
        .unwrap();

    let under_1 = signals.iter()
        .filter(|s| divider_1.cmp(s).is_ge())
        .count();
    let under_2 = signals.iter()
        .filter(|s| divider_2.cmp(s).is_ge())
        .count();

    let index_1 = under_1 + 1;
    let index_2 = under_2 + 2;

    eprintln!("Day 13.2: {}", index_1 * index_2);
}
