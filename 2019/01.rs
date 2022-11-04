use std::io::Read as _;

fn get_fuel(mass: usize) -> usize {
    if mass <= 6 {
        0
    }

    else {
        mass / 3 - 2
    }
}

fn get_fuel_recursive(mass: usize) -> usize {
    let f = get_fuel(mass);

    if f == 0 {
        0
    }

    else {
        f + get_fuel_recursive(f)
    }
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut buffer)
        .expect("Unable to read from stdin");

    let mut mass_1 = 0;
    let mut mass_2 = 0;
    for mass in buffer.lines().map(|x| x.parse::<usize>().unwrap()) {
        mass_1 += get_fuel(mass);
        mass_2 += get_fuel_recursive(mass);
    }

    eprintln!("Day 1.1: {}", mass_1);
    eprintln!("Day 1.2: {}", mass_2);
}
