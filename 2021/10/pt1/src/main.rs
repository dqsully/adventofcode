use std::env::args;
use std::fs::read_to_string;

fn main() {
    let filename = args().nth(1).unwrap_or_else(|| "../input.txt".to_owned());
    let input_txt = read_to_string(filename).unwrap();

    let mut score = 0;
    let mut stack = Vec::new();

    for line in input_txt.lines() {
        for byte in line.bytes() {
            match byte {
                b'(' => stack.push(b')'),
                b'[' => stack.push(b']'),
                b'{' => stack.push(b'}'),
                b'<' => stack.push(b'>'),
                _ if Some(&byte) == stack.last() => {
                    stack.pop();
                }
                _ => {
                    score += match byte {
                        b')' => 3,
                        b']' => 57,
                        b'}' => 1197,
                        b'>' => 25137,
                        _ => panic!("unexpected byte {}", byte),
                    };
                    break;
                }
            }
        }

        stack.clear();
    }

    println!("Total score: {}", score);
}
