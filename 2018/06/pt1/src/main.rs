use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::read_to_string;

extern crate regex;
use regex::Regex;

enum GridLoc {
    Taken(usize, i32),
    Tied,
}

fn main() {
    let schema = Regex::new(r"(\d+), (\d+)").expect("invalid regex");
    let input = read_to_string("input").expect("could not read input file");

    let points: Vec<(i32, i32)> = input.split_terminator('\n')
        .map(|s| {
            let captures = schema.captures(s).expect("line did not match regex");

            (captures[2].parse().unwrap(), captures[1].parse().unwrap())
        })
        .collect();

    let mut x_range = (1000, -1000);
    let mut y_range = (1000, -1000);

    points.iter()
        .for_each(|&(x, y)| {
            if x < x_range.0 {
                x_range.0 = x;
            }
            if x > x_range.1 {
                x_range.1 = x;
            }
            if y < y_range.0 {
                y_range.0 = y;
            }
            if y > y_range.1 {
                y_range.1 = y;
            }
        });

    println!("({:?}, {:?})", x_range, y_range);

    let mut map: HashMap<(i32, i32), GridLoc> = HashMap::new();
    let mut endpoints: HashSet<(usize, i32, i32)> = points.iter().enumerate()
        .map(|(i, &(x, y))| (i, x, y))
        .collect();
    // let mut infinites: HashSet<usize> = points.iter().enumerate()
    //     .filter(|(_, &(x, y))| x == x_range.0 || x == x_range.1 || y == y_range.0 || y == y_range.1)
    //     .map(|(i, _)| i)
    //     .collect();

    let mut cycle = 0;

    while endpoints.len() > 0 {
        // println!("map.len = {}", map.len());
        // println!("endpoints.len = {}", endpoints.len());

        let endpoints_tmp: HashSet<_> = endpoints.iter()
            .map(|&(_, x, y)| (x, y))
            .collect();

        for x in x_range.0..=x_range.1 {
            for y in y_range.0..=y_range.1 {
                match map.get(&(x, y)) {
                    Some(gridloc) => match gridloc {
                        GridLoc::Taken(id, _) => {
                            // let dist_str = dist.to_string();
                            // print!("{}", dist_str.as_bytes()[dist_str.as_bytes().len() - 1] as char);
                            print!("{}", (*id as u8 + b'A') as char);
                        },
                        GridLoc::Tied => print!("."),
                    },
                    None => if endpoints_tmp.contains(&(x, y)) {
                        print!("*")
                    } else {
                        print!(" ")
                    },
                }
            }
            println!();
        }
        println!();

        // Calculate current endpoints
        endpoints.iter()
            .for_each(|&(i, x, y)| {
                map.entry((x, y))
                    .and_modify(|g| match *g {
                        GridLoc::Taken(id, _) if id == i => {}
                        GridLoc::Taken(_, t_cycle) if t_cycle == cycle => {
                            *g = GridLoc::Tied;
                        },
                        _ => {}
                    })
                    .or_insert(GridLoc::Taken(i, cycle));
            });

        // Get new endpoints to calculate
        endpoints = endpoints.into_iter()
            .map(|(i, x, y)| vec![(i, x + 1, y), (i, x - 1, y), (i, x, y + 1), (i, x, y - 1)])
            .flatten()
            .filter(|&(_, x, y)| x >= x_range.0 && x <= x_range.1 && y >= y_range.0 && y <= y_range.1)
            .filter(|&(_, x, y)| !map.contains_key(&(x, y)))
            .collect();

        cycle += 1;
    }

    let mut infinites: HashSet<usize> = HashSet::new();
    let mut counts: HashMap<usize, i32> = HashMap::new();

    map.iter()
        .for_each(|(&(x, y), gridloc)| {
            match *gridloc {
                GridLoc::Taken(id, _) => {
                    if x == x_range.0 || x == x_range.1 || y == y_range.0 || y == y_range.1 {
                        infinites.insert(id);
                    }

                    counts.entry(id)
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
                GridLoc::Tied => {}
            }
        });

    // println!();
    //
    // counts.iter()
    //     .for_each(|(id, count)| {
    //         if infinites.contains(id) {
    //             println!("{}: infinite", (*id as u8 + b'A') as char);
    //         } else {
    //             println!("{}: {}", (*id as u8 + b'A') as char , count);
    //         }
    //     });
    //
    // println!();

    let max_area = counts.iter()
        .filter(|&(id, _)| !infinites.contains(id))
        .map(|(_, &count)| count)
        .max()
        .expect("no max found");

    println!("max_area: {}", max_area);
}
