use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let input: String = read_to_string("input").unwrap();

    let mut duplicate_count = 0;
    let mut triplicate_count = 0;

    input.split_terminator('\n')
        .for_each(|id| {
            let mut letter_count = HashMap::new();

            id.chars()
                .for_each(|c| {
                    letter_count.entry(c)
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                });

            let iter = letter_count.values();

            if iter.clone().any(|c| *c == 2) {
                duplicate_count += 1;
            }
            if iter.clone().any(|c| *c == 3) {
                triplicate_count += 1;
            }
        });

    println!("{}", duplicate_count * triplicate_count);
}
