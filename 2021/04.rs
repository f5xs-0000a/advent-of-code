use std::io::BufRead as _;

struct BingoCard([(u8, bool); 25]);

impl BingoCard {
    fn is_win(&self) -> bool {
        self.is_win_vertical() || self.is_win_horizontal()
    }

    fn is_win_vertical(&self) -> bool {
        // find any column where all elements are marked
        (0 .. 5) // col
            .any(|col| {
                (0 .. 5).all(|i| self.0[col * 5 + i].1)
            })
    }

    fn is_win_horizontal(&self) -> bool {
        // find any row where all elements are marked
        (0 .. 5) // row
            .any(|row| {
                (0 .. 5).all(|i| self.0[i * 5 + row].1)
            })
    }

    fn mark_number(&mut self, number: u8) {
        for number_flag in self.0.iter_mut() {
            if number_flag.0 == number {
                number_flag.1 = true;

                // early return
            }
        }
    }

    fn sum_of_unmarked(&self) -> usize {
        self.0
            .iter()
            .filter(|(_number, flag)| !*flag)
            .map(|(number, _)| *number as usize)
            .sum::<usize>()
    }
}

fn main() {
    let stdin = std::io::stdin();
    let lock = stdin.lock();
    let mut lines = lock.lines();

    // extract first line
    let drawn_numbers = lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|val| val.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    // extract the rest of the lines
    let mut bingo_cards = lines
        .map(|line| line.unwrap())
        .filter(|line| line != "") // empty lines reee
        .flat_map(|line| {
            line.split_whitespace()
                .map(|num| num
                    .parse::<u8>()
                    .unwrap()
                )
                .collect::<Vec<_>>()
                .into_iter()
            }
        )
        .collect::<Vec<_>>()
        .chunks(25)
        .map(|chunk| {
            let mut empty = [(0u8, false); 25];

            for (from, (to, _)) in chunk.iter().zip(empty.iter_mut()) {
                *to = *from;
            }

            BingoCard(empty)
        })
        .collect::<Vec<_>>();

    let mut p1_solved = false;
    let mut previously_won_bingo_cards = Vec::new();

    for number in drawn_numbers.iter() {
        let mut new_bingo_cards = Vec::with_capacity(bingo_cards.len());

        for mut card in bingo_cards.into_iter() {
            card.mark_number(*number);

            if card.is_win() {
                if !p1_solved {
                    eprintln!(
                        "Day 4.1: {}",
                        card.sum_of_unmarked() * (*number as usize)
                    );

                    p1_solved = true;
                }

                previously_won_bingo_cards.push((*number, card));
            }

            else {
                new_bingo_cards.push(card);
            }
        }

        bingo_cards = new_bingo_cards;
    }

    let (number, card) = previously_won_bingo_cards.last().unwrap();
    eprintln!("Day 4.2: {}", card.sum_of_unmarked() * (*number as usize));
}
