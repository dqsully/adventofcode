use std::env::args;
use std::fs::read_to_string;

struct Board {
    // The input numbers range from 0-99 (inclusive), which means the upper-most
    // bit of the byte is never used. We use this last bit to keep track of if
    // a spot was marked or not.
    numbers: [u8; 25],

    // We store 5 3-bit numbers in a single 16-bit number to reduce total size.
    // The structure ends up being 0b0aaabbbcccdddeee.
    left_rows: u16,
    left_columns: u16,
}

/// Initial value for left_rows and left_columns, storing 5 3-bit values of 5
const INIT_LEFT_VALUE: u16 = 0b0101101101101101;

impl Board {
    /// Initialize a new Board from a list of numbers
    fn new(numbers: [u8; 25]) -> Self {
        Self {
            numbers,
            left_rows: INIT_LEFT_VALUE,
            left_columns: INIT_LEFT_VALUE,
        }
    }

    /// Mark a number on this board, optionally returning the answer to the
    /// challenge if this board gets solved.
    ///
    /// This algorithm is O(1) since the board size is constant at 25 spaces.
    fn mark(&mut self, num: u8) -> Option<u32> {
        for i in 0..25 {
            if self.numbers[i] == num {
                // Mark the number (set the highest bit, otherwise unused)
                self.numbers[i] |= 0x80;

                // Figure out our column and row
                let r = i / 5;
                let c = i % 5;

                // Update our unmarked per-column and per-row counts
                self.left_rows -= 1 << (r * 3);
                self.left_columns -= 1 << (c * 3);

                // If we finished a column or row, compute the challenge answer
                if self.left_rows & 0b111 << (r * 3) == 0 ||
                    self.left_columns & 0b111 << (c * 3) == 0
                {
                    let mut unmarked_sum = 0;

                    // Compute the sum of unmarked numbers
                    for j in 0..25 {
                        if self.numbers[j] < 128 {
                            unmarked_sum += self.numbers[j] as u32;
                        }
                    }

                    return Some(unmarked_sum * (num as u32))
                }

                // Boards are guaranteed to only have one instance of each
                // number, so there's no need to check the rest.
                break;
            }
        }

        None
    }
}

fn main() {
    let filename = args().nth(1).unwrap_or_else(|| "../input.txt".to_owned());
    let input_txt = read_to_string(filename).unwrap();

    let mut lines = input_txt.lines();
    let draws = lines.next().expect("missing first line")
        .split(',')
        .map(|s| s.parse::<u8>().expect("invalid draw"));

    let mut boards: Vec<Board> = Vec::new();

    'board_parse_loop: loop {
        // Skip the empty line
        lines.next();

        let mut numbers = [0u8; 25];

        for i in 0..5 {
            let line = match lines.next() {
                Some(line) => line,
                None => break 'board_parse_loop,
            };

            for j in 0..5 {
                let num_str = &line[(j*3)..(j*3+2)];
                numbers[i * 5 + j] = num_str.trim_start().parse::<u8>().expect("not a number");
            }
        }

        boards.push(Board::new(numbers))
    }

    let mut done_boards = Vec::new();

    for draw in draws {
        let boards_len = boards.len();

        for (i, board) in boards.iter_mut().enumerate() {
            if let Some(solution) = board.mark(draw) {
                if boards_len - done_boards.len() == 1 {
                    println!("Solution: {}", solution);
                    return;
                }

                done_boards.push(i);
            }
        }

        for i in done_boards.iter().rev() {
            boards.swap_remove(*i);
        }

        done_boards.clear();
    }

    println!("No solution!!! (ya messed up...)");
}
