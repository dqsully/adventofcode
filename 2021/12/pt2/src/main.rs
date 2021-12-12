use std::cell::Cell;
use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;

#[derive(Debug)]
struct Cave {
    big: bool,
    visited: Cell<usize>,
    connections: Vec<usize>,
}

fn main() {
    let filename = args().nth(1).unwrap_or_else(|| "../input.txt".to_owned());
    let input_txt = read_to_string(filename).unwrap();

    let mut string_to_index_map: HashMap<&str, usize> = HashMap::new();

    string_to_index_map.insert("start", 0);
    string_to_index_map.insert("end", 1);

    let mut caves = vec![
        Cave { // start
            big: false,
            visited: Cell::new(2),
            connections: vec![],
        },
        Cave { // end
            big: false,
            visited: Cell::new(0),
            connections: vec![],
        },
    ];

    for line in input_txt.lines() {
        let (from, to) = line.split_once('-').unwrap();

        let from_idx = *string_to_index_map.entry(from)
            .or_insert_with(|| {
                let idx = caves.len();

                caves.push(Cave {
                    big: from.to_ascii_uppercase() == from,
                    visited: Cell::new(0),
                    connections: vec![],
                });

                idx
            });

        let to_idx = *string_to_index_map.entry(to)
            .or_insert_with(|| {
                let idx = caves.len();

                caves.push(Cave {
                    big: to.to_ascii_uppercase() == to,
                    visited: Cell::new(0),
                    connections: vec![],
                });

                idx
            });

        caves[from_idx].connections.push(to_idx);
        caves[to_idx].connections.push(from_idx);
    }

    let mut paths = 0;

    #[derive(Debug)]
    struct Decision {
        idx: usize,
        direction: usize,
    }
    let mut stack = vec![
        Decision {
            idx: 0,
            direction: 0,
        }
    ];
    let mut doubled = false;

    while !stack.is_empty() {
        let decision = stack.last_mut().unwrap();
        let cave = &caves[decision.idx];

        let next_idx = match cave.connections.get(decision.direction) {
            Some(&idx) => idx,
            None => {
                stack.pop();
                let old_visited = cave.visited.replace(cave.visited.get() - 1);

                if old_visited == 2 && !cave.big {
                    doubled = false;
                }

                if let Some(prev_decision) = stack.last_mut() {
                    prev_decision.direction += 1;
                }

                continue;
            }
        };

        if next_idx == 1 {
            paths += 1;
            decision.direction += 1;
            continue;
        }

        let next_cave = &caves[next_idx];

        if next_cave.visited.get() >= 2 - (doubled as usize) && !next_cave.big {
            decision.direction += 1;
            continue;
        }

        stack.push(Decision {
            idx: next_idx,
            direction: 0,
        });
        let old_visited = next_cave.visited.replace(next_cave.visited.get() + 1);

        if old_visited == 1 && !next_cave.big {
            doubled = true;
        }
    }

    println!("Total paths: {}", paths);
}
