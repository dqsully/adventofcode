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

    deps.into_iter()
        .for_each(|(parent, child)| {
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
    let mut cur_nodes: BTreeSet<_> = roots.into_iter()
        .filter_map(|(parent, root)| if root {Some(parent)} else {None})
        .collect();

    while cur_nodes.len() > 0 {
        // println!("{:?}", cur_nodes);

        let next_letter = *cur_nodes.iter()
            .find(|&parent| {
                match dependents_left.get_mut(parent) {
                    Some(deps) => deps.len() == 0,
                    None => true,
                }
            })
            .expect("No solution");

        if let Some(children) = dag.get(next_letter) {
            children.iter()
                .inspect(|child| {
                    cur_nodes.insert(child);
                })
                .for_each(|child| {
                    dependents_left.entry(child)
                        .and_modify(|deps| {
                            deps.remove(next_letter);
                        });
                })
        }

        output.push(next_letter);
        cur_nodes.remove(next_letter);
    }

    println!("{}", output.iter().fold(String::new(), |s, p| s + p));
}
