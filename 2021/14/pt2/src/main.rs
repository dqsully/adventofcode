use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;

struct PolymerTemplate {
    start: u8,
    end: u8,
    insert: u8,
}

const ITERATIONS: usize = 40;

fn main() {
    let filename = args().nth(1).unwrap_or_else(|| "../input.txt".to_owned());
    let input_txt = read_to_string(filename).unwrap();
    let mut lines = input_txt.lines();

    let mut pairs: HashMap<(u8, u8), usize> = HashMap::new();
    let mut counts: HashMap<u8, usize> = HashMap::new();

    let mut last_byte = 0;

    for b in lines.next().unwrap().bytes() {
        if last_byte > 0 {
            pairs.entry((last_byte, b))
                .and_modify(|n| *n += 1)
                .or_insert(1);
        }
        last_byte = b;

        counts.entry(b)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    lines.next();

    let mut polymer_templates: Vec<PolymerTemplate> = Vec::new();

    for line in lines {
        let (pair, insert) = line.split_once(" -> ").unwrap();

        polymer_templates.push(PolymerTemplate {
            start: pair.as_bytes()[0],
            end: pair.as_bytes()[1],
            insert: insert.as_bytes()[0],
        });
    }

    for _ in 0..ITERATIONS {
        let mut next_pairs = HashMap::new();

        for template in &polymer_templates {
            if let Some(&count) = pairs.get(&(template.start, template.end)) {
                counts.entry(template.insert)
                    .and_modify(|c| *c += count)
                    .or_insert(count);

                next_pairs.entry((template.start, template.insert))
                    .and_modify(|c| *c += count)
                    .or_insert(count);
                next_pairs.entry((template.insert, template.end))
                    .and_modify(|c| *c += count)
                    .or_insert(count);
            }
        }

        pairs = next_pairs;
    }

    let mut counts_sorted = counts.into_values().collect::<Vec<_>>();

    counts_sorted.sort_unstable();

    let least_common = counts_sorted.first().unwrap();
    let most_common = counts_sorted.last().unwrap();

    println!("Diff of counts: {}", most_common - least_common)
}
