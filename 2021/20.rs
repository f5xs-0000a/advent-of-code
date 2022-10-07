use std::io::Read as _;

struct Matrix {
    inner_vec: Box<[bool]>,
    rows: usize,
    cols: usize,
    outside_state: bool,
}

impl Matrix {
    fn new(
        inner: Box<[bool]>,
        rows: usize,
        cols: usize,
        outside_state: bool
    ) -> Matrix {
        assert_eq!(inner.len(), rows * cols);

        Matrix {
            inner_vec: inner,
            rows,
            cols,
            outside_state,
        }
    }

    fn get_flag_at_coords(&self, row: usize, col: usize) -> bool {
        self.inner_vec.get(row * self.cols + col).cloned().unwrap()
    }

    fn get_enhanced_at_output_coords(
        &self,
        row: usize,
        col: usize,
        enhancer: &[bool]
    ) -> bool {
        // NOTE: you have to consider for when the enhancer has # at the zeroth
        // bit. this means all outside edges become #. it will flip back to .
        // on the second iteration only if the final bit is ".". this results
        // in an alternating edge pattern every odd number of enhancement
        let kernel = [
            (0, 0, (2usize).pow(8)),
            (1, 0, (2usize).pow(7)),
            (2, 0, (2usize).pow(6)),
            (0, 1, (2usize).pow(5)),
            (1, 1, (2usize).pow(4)),
            (2, 1, (2usize).pow(3)),
            (0, 2, (2usize).pow(2)),
            (1, 2, (2usize).pow(1)),
            (2, 2, (2usize).pow(0)),
        ];
        // remember, you still have to offset it by 1

        let mut index = 0;
        
        for (col_offset, row_offset, exponent) in kernel.iter() {
            let mut flag = false;

            // check for out of bounds
            if row + row_offset < 2
                || row + row_offset >= self.rows + 2
                || col + col_offset < 2
                || col + col_offset >= self.cols + 2
            {
                flag = self.outside_state;
            }

            else {
                flag = self.get_flag_at_coords(
                    row + row_offset - 2,
                    col + col_offset - 2
                );
            }

            if flag {
                index += exponent;
            }
        }

        enhancer.get(index).cloned().unwrap()
    }

    fn upscale(&self, enhancer: &[bool]) -> Matrix {
        assert_eq!(enhancer.len(), 512);

        let mut new_matrix = Vec::with_capacity(
            (self.rows + 2) * (self.cols + 2)
        );

        for r in 0 .. self.cols + 2 {
            for c in 0 .. self.rows + 2 {
                let flag = self.get_enhanced_at_output_coords(r, c, enhancer);
                new_matrix.push(flag);
            }
        }

        let zero_bit_true = enhancer[0];

        // OS  ZB   OS+ZB
        //  0   0     0  ( no flipping )
        //  0   1     1  ( flip )
        //  1   0     1  ( no flip but is flipped )
        //  1   1     0  ( flip from flipped )
        // operator: XNOR (or the != operator)

        Matrix {
            inner_vec: new_matrix.into_boxed_slice(),
            rows: self.rows + 2,
            cols: self.cols + 2,
            outside_state: self.outside_state != zero_bit_true, 
        }
    }

    fn count(&self) -> usize {
        self.inner_vec.iter().filter(|b| **b).count()
    }
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut buffer)
        .expect("Unable to read from stdin");

    let mut lines = buffer.lines();

    // find the enhancer pattern
    let mut enhancer = vec![];
    let enhancer_line = lines.next().unwrap();
    for ch in enhancer_line.split_whitespace().flat_map(|s| s.chars()) {
        let value = match ch {
            '.' => false,
            '#' => true,
            _ => panic!("Unexpected character"),
        };

        enhancer.push(value);
    }

    lines.next(); // dump the next line. it's empty.

    // populate the matrix
    let mut matrix = vec![];
    let mut columns = 0;
    let mut rows = 0;
    for line in lines {
        for ch in line.chars() {
            let c = match ch {
                '.' => false,
                '#' => true,
                _ => panic!("Unexpected character"),
            };

            matrix.push(c);
        }

        if columns == 0 {
            columns = matrix.len();
        }

        rows += 1;
    }

    let mut matrix = Matrix::new(matrix.into_boxed_slice(), rows, columns, false);
    matrix = matrix.upscale(&enhancer);
    matrix = matrix.upscale(&enhancer);
    eprintln!("Day 20.1: {}", matrix.count());

    for _ in 2 .. 50 {
        matrix = matrix.upscale(&enhancer);
    }

    eprintln!("Day 20.2: {}", matrix.count());
}
