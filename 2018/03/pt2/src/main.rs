use std::fs::read_to_string;
use std::cmp::{max, min};
use std::collections::HashSet;

extern crate regex;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cut {
    id: String,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl Cut {
    fn intersection(&self, other: &Cut) -> Option<Cut> {
        let x1 = max(self.x, other.x);
        let y1 = max(self.y, other.y);
        let x2 = min(self.w + self.x, other.w + other.x);
        let y2 = min(self.h + self.y, other.h + other.y);

        if x2 > x1 && y2 > y1 {
            Some(Cut{
                id: "".to_owned(),
                x: x1,
                y: y1,
                w: x2 - x1,
                h: y2 - y1,
            })
        } else {
            None
        }
    }
}

fn main() {
    let schema = Regex::new(r"(#\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let input = read_to_string("input").unwrap();

    let cuts: Vec<Cut> = input.split_terminator('\n')
        .filter_map(|l| schema.captures(l))
        .map(|c| Cut {
            id: c[1].to_owned(),
            x: c[2].parse().unwrap(),
            y: c[3].parse().unwrap(),
            w: c[4].parse().unwrap(),
            h: c[5].parse().unwrap(),
        }).collect();

    let mut intersections = HashSet::new();

    cuts.iter()
        .for_each(|c1| {
            cuts.iter()
                .filter(|c2| c1.intersection(c2).is_some() && *c2 != c1)
                .for_each(|c2| {
                    intersections.insert(c2.id.clone());
                });
        });

    let no_intersections: Vec<_> = cuts.iter()
        .filter(|c| !intersections.contains(&c.id))
        .collect();

    println!("{:?}", no_intersections);
}
