use std::io::Read as _;

fn determine_device_loop_number(public_key: usize) -> usize {
    let mut cur_val = 1;
    let mut loop_size = 0;
    let subj_number = 7;
    
    while cur_val != public_key {
        cur_val = (cur_val * subj_number) % 20201227;
        loop_size += 1;
    }

    loop_size
}

fn get_encryption_key(other_public_key: usize, loop_size: usize) -> usize {
    let mut cur_val = 1;
    for _ in 0 .. loop_size {
        cur_val = (cur_val * other_public_key) % 20201227;
    }

    cur_val
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut buffer)
        .expect("Unable to read from stdin");

    let mut lines = buffer.lines();
    let pk_1 = lines
        .next()
        .unwrap()
        .parse::<usize>()
        .expect("Cannot convert into usize");
    let pk_2 = lines
        .next()
        .unwrap()
        .parse::<usize>()
        .expect("Cannot convert into usize");
    assert!(lines.next().is_none());

    let ln_1 = determine_device_loop_number(pk_1);
    let ln_2 = determine_device_loop_number(pk_2);

    let ek_1 = get_encryption_key(pk_2, ln_1);
    let ek_2 = get_encryption_key(pk_1, ln_2);

    assert_eq!(ek_1, ek_2);
    eprintln!("Day 25: {}", ek_1);
}
