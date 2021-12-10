use std::env::args;
use std::fs::read_to_string;

fn main() {
    let filename = args().nth(1).unwrap_or_else(|| "../input.txt".to_owned());
    let input_txt = read_to_string(filename).unwrap();

    let mut total_scores = Vec::new();
    let mut stack = Vec::new();

    'line_loop: for line in input_txt.lines() {
        stack.clear();

        for byte in line.bytes() {
            match byte {
                b'(' => stack.push(b')'),
                b'[' => stack.push(b']'),
                b'{' => stack.push(b'}'),
                b'<' => stack.push(b'>'),
                _ if Some(&byte) == stack.last() => {
                    stack.pop();
                }
                _ => continue 'line_loop,
            }
        }

        let mut line_score: u64 = 0;

        for remaining_byte in stack.drain(..).rev() {
            line_score *= 5;
            line_score += match remaining_byte {
                b')' => 1,
                b']' => 2,
                b'}' => 3,
                b'>' => 4,
                _ => unreachable!(),
            };
        }

        total_scores.push(line_score);
    }

    total_scores.sort_unstable();

    println!("Middle score: {}", total_scores[total_scores.len()/2]);
}
