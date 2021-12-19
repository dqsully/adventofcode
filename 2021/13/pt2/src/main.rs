use std::collections::HashSet;
use std::env::args;
use std::fs::read_to_string;

fn main() {
    let filename = args().nth(1).unwrap_or_else(|| "../input.txt".to_owned());
    let input_txt = read_to_string(filename).unwrap();

    let mut points: HashSet<(i32, i32)> = HashSet::new();

    let mut is_folds = false;

    for line in input_txt.lines() {
        if line.is_empty() {
            is_folds = true;
        } else if !is_folds {
            let (x, y) = line.split_once(',').unwrap();
            points.insert((x.parse().unwrap(), y.parse().unwrap()));
        } else {
            let (fold, coord) = line.split_once('=').unwrap();

            let coord = coord.parse::<i32>().unwrap();
            let is_x = *fold.as_bytes().last().unwrap() == b'x';

            points = points.into_iter()
                .map(|(x, y)| {
                    if is_x && x > coord {
                        (2 * coord - x, y)
                    } else if !is_x && y > coord {
                        (x, 2 * coord - y)
                    } else {
                        (x, y)
                    }
                })
                .collect();
        }
    }

    let lim_x = points.iter()
        .map(|&(x, _)| x)
        .max().unwrap() as usize + 1;
    let lim_y = points.iter()
        .map(|&(_, y)| y)
        .max().unwrap() as usize + 1;

    let mut map: Vec<Vec<bool>> = Vec::with_capacity(lim_y);

    for _ in 0..lim_y {
        map.push(vec![false; lim_x]);
    }

    for (x, y) in points {
        map[y as usize][x as usize] = true;
    }

    for row in map {
        for point in row {
            if point {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
