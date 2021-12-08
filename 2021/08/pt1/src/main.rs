use std::env::args;
use std::fs::read_to_string;

fn main() {
    let filename = args().nth(1).unwrap_or_else(|| "../input.txt".to_owned());
    let input_txt = read_to_string(filename).unwrap();

    let mut unique_segment_count = 0;

    for line in input_txt.lines() {
        let output_value = line.split_once('|').unwrap().1.trim();

        for digit in output_value.split_whitespace() {
            match digit.len() {
                2 | 3 | 4 | 7 => unique_segment_count += 1,
                _ => {}
            }
        }
    }

    println!("Count of 1, 4, 7, and 8: {}", unique_segment_count);
}
