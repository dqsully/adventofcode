use std::env::args;
use std::fs::read_to_string;

fn main() {
    let filename = args().nth(1).unwrap_or_else(|| "input.txt".to_owned());

    let input_txt = read_to_string(filename).unwrap();

    let measurements: Vec<i32> = input_txt.split_terminator('\n')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut increases = 0;

    for i in 3..measurements.len() {
        if measurements[i - 3] < measurements[i] {
            increases += 1;
        }
    }

    println!("{}", increases);
}
