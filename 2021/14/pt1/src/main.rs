#![feature(map_first_last)]

use std::cell::Cell;
use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;

#[derive(Debug)]
struct Element {
    prev: Cell<Option<usize>>,
    next: Cell<Option<usize>>,
    element: u8,
}

struct PolymerTemplate {
    start: u8,
    end: u8,
    insert: u8,
}

const ITERATIONS: usize = 10;

fn main() {
    let filename = args().nth(1).unwrap_or_else(|| "../input.txt".to_owned());
    let input_txt = read_to_string(filename).unwrap();
    let mut lines = input_txt.lines();

    let mut polymer_pieces: Vec<Element> = Vec::new();
    let mut counts: HashMap<u8, usize> = HashMap::new();

    for b in lines.next().unwrap().bytes() {
        let len = polymer_pieces.len();

        let prev = if let Some(last) = polymer_pieces.last() {
            last.next.set(Some(len));
            Some(len - 1)
        } else {
            None
        };

        polymer_pieces.push(Element {
            prev: Cell::new(prev),
            next: Cell::new(None),
            element: b,
        });

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

    let mut to_append = Vec::new();

    for _ in 0..ITERATIONS {
        let mut element = &polymer_pieces[0];
        let len = polymer_pieces.len();

        while let Some(next_idx) = element.next.get() {
            let next_element = &polymer_pieces[next_idx];

            for template in &polymer_templates {
                if element.element == template.start && next_element.element == template.end {
                    to_append.push(Element {
                        prev: Cell::new(next_element.prev.get()),
                        next: Cell::new(element.next.get()),
                        element: template.insert,
                    });

                    next_element.prev.set(Some(len + to_append.len() - 1));
                    element.next.set(Some(len + to_append.len() - 1));


                    counts.entry(template.insert)
                        .and_modify(|c| *c += 1)
                        .or_insert(1);

                    break;
                }
            }

            element = next_element;
        }

        polymer_pieces.append(&mut to_append);
    }

    let mut counts_sorted = counts.into_values().collect::<Vec<_>>();

    counts_sorted.sort_unstable();

    let least_common = counts_sorted.first().unwrap();
    let most_common = counts_sorted.last().unwrap();

    println!("Diff of counts: {}", most_common - least_common)
}
