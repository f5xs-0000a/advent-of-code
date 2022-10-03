use std::io::Read as _;

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut buffer)
        .expect("Unable to read from stdin");

    let mut wrapper = 0;
    let mut ribbon = 0;
    for line in buffer.lines() {
        let mut sep = line.split('x');

        let l = sep.next().unwrap().parse::<u32>().unwrap();
        let w = sep.next().unwrap().parse::<u32>().unwrap();
        let h = sep.next().unwrap().parse::<u32>().unwrap();

        let lw = l * w;
        let wh = w * h;
        let lh = l * h;

        let wrapper_lowest = lw.min(wh).min(lh);
        let ribbon_highest = l.max(w).max(h);

        wrapper += 2 * (lw + wh + lh) + wrapper_lowest;
        ribbon += 2 * (l + w + h - ribbon_highest) + l * w * h;
    }

    eprintln!("Day 2.1: {}", wrapper);
    eprintln!("Day 2.2: {}", ribbon);
}
