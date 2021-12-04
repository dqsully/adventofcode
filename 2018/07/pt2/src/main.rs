use std::collections::{HashSet, HashMap, BTreeMap, BTreeSet};
use std::fs::read_to_string;

extern crate regex;
use regex::Regex;

fn main() {
    let schema = Regex::new(r"Step ([A-Z]+) must be finished before step ([A-Z]+) can begin.").expect("invalid regex");
    let input = read_to_string("input").expect("could not read input file");

    let deps: Vec<_> = input.split_terminator('\n')
        .map(|line| {
            let captures = schema.captures(line).expect("invalid line");

            (captures.get(1).unwrap().as_str(), captures.get(2).unwrap().as_str())
        })
        .collect();

    let mut dag: HashMap<&str, BTreeSet<&str>> = HashMap::new();
    let mut roots: BTreeMap<&str, bool> = BTreeMap::new();
    let mut dependents_left: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut total_nodes: HashSet<&str> = HashSet::new();

    deps.into_iter()
        .for_each(|(parent, child)| {
            total_nodes.insert(parent);
            total_nodes.insert(child);

            dag.entry(parent)
                .and_modify(|deps| {
                    deps.insert(child);
                })
                .or_insert_with(|| {
                    let mut out = BTreeSet::new();
                    out.insert(child);
                    out
                });

            dependents_left.entry(child)
                .and_modify(|dependents| {
                    dependents.insert(parent);
                })
                .or_insert_with(|| {
                    let mut out = HashSet::new();
                    out.insert(parent);
                    out
                });

            roots.entry(parent)
                .or_insert(true);
            roots.insert(child, false);
        });

    let mut output: Vec<&str> = Vec::new();
    let mut new_nodes: BTreeSet<_> = roots.into_iter()
        .filter_map(|(parent, root)| if root {Some(parent)} else {None})
        .collect();
    let mut working_nodes: HashMap<&str, u32> = HashMap::new();
    let mut ever_worked: HashSet<&str> = HashSet::new();

    let mut cycle = 0;
    const WORKERS: usize = 5;
    const TIME: u32 = 60;

    loop {
        while working_nodes.len() < WORKERS && new_nodes.len() > 0 {
            let next_letter = new_nodes.iter()
                .find(|&parent|
                    match dependents_left.get_mut(parent) {
                        Some(deps) => deps.len() == 0,
                        None => true,
                    }
                );

            let next_letter = match next_letter {
                Some(l) => *l,
                None => break,
            };

            working_nodes.insert(next_letter, (next_letter.as_bytes()[0] - b'A') as u32 + 1 + TIME);
            ever_worked.insert(next_letter);
            new_nodes.remove(next_letter);
        }

        println!("Finished adding new nodes (cycle = {})", cycle);
        println!("new: {:?}", new_nodes);
        println!("working: {:?}", working_nodes);
        println!("done: {:?}", output);
        println!("ever: {}, all: {}", ever_worked.len(), total_nodes.len());

        loop {
            let (more, done) = working_nodes.into_iter()
                .map(|(node, time)| (node, time - 1))
                .partition(|&(_, time)| time != 0);

            working_nodes = more;

            let mut done: Vec<_> = done.into_iter()
                .inspect(|(next_letter, _)| {
                    if let Some(children) = dag.get(next_letter) {
                        children.iter()
                            .filter(|child| !ever_worked.contains(*child))
                            .for_each(|child| {
                                dependents_left.entry(child)
                                    .and_modify(|deps| {
                                        deps.remove(next_letter);
                                    });
                                new_nodes.insert(child);
                            });
                    }
                })
                .map(|(node, _)| node).collect();
            done.sort();
            let done_len = done.len();
            output.append(&mut done);

            cycle += 1;

            if (ever_worked.len() < total_nodes.len() && done_len > 0) || working_nodes.len() == 0 {
                break;
            }
        }

        println!("Finished calculating work (cycle = {})", cycle);
        println!("new: {:?}", new_nodes);
        println!("working: {:?}", working_nodes);
        println!("done: {:?}", output);
        println!();

        if new_nodes.len() == 0 {
            break;
        }
    }

    // let mut working_nodes: Vec<(&str, u32)> = Vec::new();
    //
    // while cur_nodes.len() > 0 {
    //     println!("{:?}", cur_nodes);
    //     println!("{:?}", working_nodes);
    //
    //     let next_letter = *cur_nodes.iter()
    //         .find(|&parent| {
    //             match dependents_left.get_mut(parent) {
    //                 Some(deps) => deps.len() == 0,
    //                 None => true,
    //             }
    //         })
    //         .expect("No solution");
    //
    //     if let Some(children) = dag.get(next_letter) {
    //         children.iter()
    //             .inspect(|child| {
    //                 cur_nodes.insert(child);
    //             })
    //             .for_each(|child| {
    //                 dependents_left.entry(child)
    //                     .and_modify(|deps| {
    //                         deps.remove(next_letter);
    //                     });
    //             })
    //     }
    //
    //     loop {
    //         let (more, done) = working_nodes.into_iter()
    //             .map(|(node, time)| (node, time - 1))
    //             .partition(|&(_, time)| time != 0);
    //
    //         println!("more: {:?}, done: {:?}", more, done);
    //
    //         working_nodes = more;
    //         done.iter()
    //             .for_each(|&(next_letter, _)| {
    //                 if let Some(children) = dag.get(next_letter) {
    //                     children.iter()
    //                         .inspect(|child| {
    //                             cur_nodes.insert(child);
    //                         })
    //                         .for_each(|child| {
    //                             dependents_left.entry(child)
    //                                 .and_modify(|deps| {
    //                                     deps.remove(next_letter);
    //                                 });
    //                         })
    //                 }
    //
    //                 output.push(next_letter);
    //             });
    //
    //         if working_nodes.len() < 4 {
    //             break;
    //         }
    //     }
    //     cur_nodes.remove(next_letter);
    //     // output.push(next_letter);
    //
    //     working_nodes.push((next_letter, (next_letter.as_bytes()[0] - b'A') as u32 + 61));
    //
    //     println!();
    // }
    //
    // loop {
    //     let (more, done) = working_nodes.into_iter()
    //         .map(|(node, time)| (node, time - 1))
    //         .partition(|&(_, time)| time != 0);
    //
    //     working_nodes = more;
    //     done.iter()
    //         .for_each(|&(next_letter, _)| {
    //             if let Some(children) = dag.get(next_letter) {
    //                 children.iter()
    //                     .inspect(|child| {
    //                         cur_nodes.insert(child);
    //                     })
    //                     .for_each(|child| {
    //                         dependents_left.entry(child)
    //                             .and_modify(|deps| {
    //                                 deps.remove(next_letter);
    //                             });
    //                     })
    //             }
    //
    //             output.push(next_letter);
    //         });
    //
    //     if working_nodes.len() == 0 {
    //         break;
    //     }
    // }

    println!("{}", output.iter().fold(String::new(), |s, p| s + p));
}
