//! `O(n*k)`

use std::fs::read_to_string;
use std::collections::HashSet;

fn main() {
    let input: String = read_to_string("input").unwrap();

    let mut set = HashSet::new();

    let output = input.split_terminator('\n')
        .find_map(|s| {
            for i in 0..s.len() {
                let string = (&s[..i], &s[i + 1..]);

                if set.contains(&string) {
                    return Some(format!("{}{}", string.0, string.1));
                }
                set.insert(string);
            }

            None
        });

    println!("{:?}", output);
}
