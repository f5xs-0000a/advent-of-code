use std::io::BufRead as _;

#[path = "intcode.rs"]
mod intcode;

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .lock()
        .read_line(&mut buffer)
        .expect("Unable to read from stdin");

    let mut intcode = intcode::generate_intcode(&*buffer);

    let mut o = String::new();
    let writer = |src: isize, dest: &mut String| {
        use std::fmt::Write as _;
        dest.clear();
        write!(dest, "{}", src).unwrap();
    };

    intcode::run_program(&mut intcode.clone(), || 1, |s| writer(s, &mut o));
    eprintln!("Day 5.1: {}", o);

    intcode::run_program(&mut *intcode, || 5, |s| writer(s, &mut o));
    eprintln!("Day 5.2: {}", o);
}
