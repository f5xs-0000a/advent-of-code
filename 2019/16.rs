use std::io::BufRead as _;
fn main() {
    let mut buffer = String::new();
    std::io::stdin().lock().read_line(&mut buffer).unwrap();

    let input = buffer
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|x| x as i32)
        .collect::<Vec<_>>();

    let fft_1 = fft(&*input, 100)
        .iter()
        .take(8)
        .rev()
        .enumerate()
        .map(|(i, x)| x * 10i32.pow(i as u32)).sum::<i32>();
    eprintln!("Day 16.1: {}", fft_1);

    let skipped_digits = input[0 .. 7]
        .iter()
        .rev()
        .enumerate()
        .map(|(exp, base)| *base as usize * 10usize.pow(exp as u32))
        .sum::<usize>();

    let input_len = input.len();
    let repeated = input
        .into_iter()
        .cycle()
        .take(input_len * 10000)
        .collect::<Vec<_>>();

    let fft_2 = fft(&*repeated, 100)
        .iter()
        .skip(skipped_digits)
        .take(8)
        .rev()
        .enumerate()
        .map(|(i, x)| x * 10i32.pow(i as u32)).sum::<i32>();
    eprintln!("Day 16.2: {}", fft_2);
}

fn fft(input_array: &[i32], passes: usize) -> Vec<i32> {
    use std::cmp::min;

    let mut array_1 = vec![0; input_array.len()];
    let mut array_2 = vec![0; input_array.len()];

    let length = input_array.len();

    for pass in 0 .. passes {
        eprintln!("Pass {}", pass);

        let ref input;
        let ref mut output;

        // define where to put the output
        if pass == 0 {
            input = input_array;
            output = &mut array_1;
        }

        else if pass % 2 == 1 {
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

        // try to visualize the multiplier table (the -1, 0, and 1) using a
        // visualizer
        //
        // if you try to notice an expanded form of the multiplier table, you'll
        // notice bands like rays of light coming from the origin (top left of
        // the table). you can utilize these multipliers in order to not
        // needlessly sum what has been already summed. assume a single "band"
        // then create a horizontal slice going from top to bottom. notice that
        // as you go down, the band moves in a predictable way: removing from
        // the left and adding from the right. perhaps you can exploit this.

        for i in 0 .. length / 2 {
            let mut current_sum = 0;
            let mut starting_index = 0;
            let mut ending_index = 0;

            let multiplier = match i % 2 {
                0 => 1,
                1 => -1,
                _ => unreachable!(),
            };

            for (n, output) in output.iter_mut().enumerate() {
                // modify indices
                let new_starting_index = n * (2 * i + 1) + 2 * i;
                let new_ending_index = new_starting_index + n + 1;

                // don't even consider those outside the bounds
                if length <= new_starting_index {
                    break;
                }

                // modify sum
                if ending_index <= new_starting_index {
                    current_sum = input[new_starting_index .. min(new_ending_index, length)]
                        .iter()
                        .sum();
                }

                // this part is the optimization. it's a very great
                // optimization.
                else {
                    // subtract those in the back
                    current_sum -= input[starting_index .. min(new_starting_index, length)]
                        .iter()
                        .sum::<i32>();

                    // and add those in the front
                    if ending_index < length {
                        current_sum += input[ending_index .. min(new_ending_index, length)]
                            .iter()
                            .sum::<i32>();
                    }
                }

                // perform summation
                *output += current_sum * multiplier;

                starting_index = new_starting_index;
                ending_index = new_ending_index;
            }
        }

        for output in output.iter_mut() {
            *output = output.abs() % 10;
        }
    }

    if passes % 2 == 1 {
        array_1
    }

    else {
        array_2
    }
}
