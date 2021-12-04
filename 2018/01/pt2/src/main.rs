use std::fs::read_to_string;
use std::collections::HashSet;

fn main() {
    let input = read_to_string("list.txt").unwrap();

    let mut freqs = HashSet::new();
    freqs.insert(0);

    let output = input.split_terminator('\n')
        .map(|s| s.parse::<i32>().unwrap())
        .cycle()
        .scan(0, |a, c| {
            *a += c;
            Some(*a)
        })
        .find(|c| {
            if freqs.contains(c) {
                true
            } else {
                freqs.insert(*c);
                false
            }
        });

    println!("{}", output.unwrap());
}
