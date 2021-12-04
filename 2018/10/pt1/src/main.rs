use std::collections::HashSet;
use std::fs::read_to_string;
use std::io;

extern crate regex;
use regex::Regex;

struct Point {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

fn main() {
    let schema = Regex::new(r"position=<\s*([0-9-]+),\s*([0-9-]+)> velocity=<\s*([0-9-]+),\s*([0-9-]+)>").expect("invalid regex");
    let input = read_to_string("input").expect("couldn't read input file");
    let stdin = io::stdin();

    let range_reset = ((1000, -1000), (1000, -1000));
    let mut range = range_reset;
    let mut points: Vec<Point> = input.split_terminator('\n')
        .map(|l| {
            let captures = schema.captures(l).expect("invalid line");

            Point {
                x: captures[1].parse().expect("invalid x number"),
                y: captures[2].parse().expect("invalid y number"),
                dx: captures[3].parse().expect("invalid dx number"),
                dy: captures[4].parse().expect("invalid dy number"),
            }
        })
        .inspect(|&Point {x, y, ..}| {
            if x < (range.0).0 {
                (range.0).0 = x;
            }
            if x > (range.0).1 {
                (range.0).1 = x;
            }
            if y < (range.1).0 {
                (range.1).0 = y;
            }
            if y > (range.1).1 {
                (range.1).1 = y;
            }
        })
        .collect();

    for i in 0..11000 {
        if (range.0).1 - (range.0).0 < 300 && (range.1).1 - (range.1).0 < 300 {
            println!("i: {}", i);
            let visible_points: HashSet<(i32, i32)> = points.iter()
                .map(|&Point {x, y, ..}| (x, y))
                .collect();

            for y in (range.1).0 ..= (range.1).1 {
                for x in (range.0).0 ..= (range.0).1 {
                    if visible_points.contains(&(x, y)) {
                        print!("*");
                    } else {
                        print!(" ");
                    }
                }
                println!();
            }

            println!();
        }

        range = range_reset;
        points = points.into_iter()
            .map(|mut point| {
                point.x += point.dx;
                point.y += point.dy;
                point
            })
            .inspect(|&Point {x, y, ..}| {
                if x < (range.0).0 {
                    (range.0).0 = x;
                }
                if x > (range.0).1 {
                    (range.0).1 = x;
                }
                if y < (range.1).0 {
                    (range.1).0 = y;
                }
                if y > (range.1).1 {
                    (range.1).1 = y;
                }
            })
            .collect();
    }
}
