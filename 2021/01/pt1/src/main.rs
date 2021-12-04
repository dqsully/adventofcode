use std::env::args;
use std::fs::read_to_string;

fn main() {
    let filename = args().nth(1).unwrap_or_else(|| "input.txt".to_owned());

    let input_txt = read_to_string(filename).unwrap();

    let increases = input_txt.split_terminator('\n')
        .map(|s| s.parse::<i32>().unwrap())
        .fold((0, None), |(increased, last), x| {
            let increased = match last {
                Some(last) if x > last => increased + 1,
                _ => increased,
            };

            (increased, Some(x))
        });

    println!("{}", increases.0);
}
