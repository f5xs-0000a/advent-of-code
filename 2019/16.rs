use std::io::BufRead as _;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().lock().read_line(&mut buffer);

    let input = buffer
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|x| x as i32)
        .collect::<Vec<_>>();

    dbg!(fft(&*input, 100, 1));
    dbg!(fft(&*repeated, 100, 10000));
}

fn fft(input_array: &[i32], passes: usize, repetitions: usize) -> Vec<i32> {
    use std::cmp::min;

    let mut array_1 = vec![0; input_array.len()];
    let mut array_2 = vec![0; input_array.len()];

    for i in 0 .. passes {
        eprintln!("Pass #{}", i + 1);
        let ref input;
        let ref mut output;

        // define where to put the output
        if i == 0 {
            input = input_array;
            output = &mut array_1;
        }

        else if i % 2 == 1 {
            input = &array_1;
            output = &mut array_2;
        }

        else {
            input = &array_2;
            output = &mut array_1;
        }

        // clear contents of output
        for x in output.iter_mut() {
            *x = 0;
        }

        // populate the output
        for (size, out) in output.iter_mut().enumerate() {
            let size = size + 1;
            eprintln!("Size #{}", size);
            
            for offset in 0 .. {
                // add the positive portion
                let positive_offset = size * (4 * offset + 1) - 1;
                let positive_max = min(
                    positive_offset + size,
                    input_array.len()
                );

                if positive_offset >= input_array.len() {
                    break;
                }

                for value in &input[positive_offset .. positive_max] {
                    *out += value;
                }

                // add the negative portion
                let negative_offset = size * (4 * offset + 3) - 1;
                let negative_max = min(
                    negative_offset + size,
                    input_array.len()
                );

                if negative_offset >= input_array.len() {
                    break;
                }

                for value in &input[negative_offset .. negative_max] {
                    *out -= value;
                }
            }
        }

        // fix the output
        for out in output.iter_mut() {
            *out = out.abs() % 10;
        }
    }

    if passes % 2 == 1 {
        array_1
    }

    else {
        array_2
    }
}
