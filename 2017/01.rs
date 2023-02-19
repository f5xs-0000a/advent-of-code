use std::io::Read as _;

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut buffer)
        .expect("Unable to read from stdin");

    let digits = buffer
        .chars()
        .filter_map(|x| x.to_digit(10))
        .collect::<Vec<_>>();

    let mut iter = digits
        .iter()
        .chain(digits.iter().take(1));

    let mut sum = 0;
    let mut current = iter.clone().next().unwrap();
    while let Some(next) = iter.next(){
        if current == next {
            sum += current;
        }

        current = next;
    }

    eprintln!("Day 1.1: {}", sum);

    let mut sum = 0;
    for (i, j) in digits.iter().skip(digits.len() / 2).zip(digits.iter()) {
        if i == j {
            sum += i * 2;
        }
    }

    eprintln!("Day 1.2: {}", sum);
}
