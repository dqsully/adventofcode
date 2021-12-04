use std::fs::read_to_string;

fn main() {
    let input_txt = read_to_string("list.txt").unwrap();
    let output = input_txt.split_terminator('\n')
        .map(|s| s.parse::<i32>().unwrap())
        .sum();

    println!("{}", output);
}
