use std::fs::read_to_string;
use std::collections::HashSet;

fn main() {
    let input_txt = read_to_string("input").unwrap();
    let numbers = input_txt.split_terminator('\n')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<HashSet<_>>();

    for number in &numbers {
        let compliment = 2020 - number;

        if numbers.contains(&compliment) {
            println!("{} * {} = {}", number, compliment, number * compliment);
            break;
        }
    }
}
