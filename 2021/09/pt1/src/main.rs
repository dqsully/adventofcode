use std::env::args;
use std::fs::read_to_string;

fn main() {
    let filename = args().nth(1).unwrap_or_else(|| "../input.txt".to_owned());
    let input_txt = read_to_string(filename).unwrap();

    let mut map: Vec<Vec<u8>> = Vec::new();

    for line in input_txt.lines() {
        map.push(line.bytes().map(|b| b - b'0').collect());
    }

    let width = map[0].len();
    let height = map.len();

    let mut total_risk: u64 = 0;

    for y in 0..height {
        for x in 0..width {
            let depth = map[y][x];

            if (x == 0 || map[y][x - 1] > depth) &&
               (x == width - 1 || map[y][x + 1] > depth) &&
               (y == 0 || map[y - 1][x] > depth) &&
               (y == height - 1 || map[y + 1][x] > depth)
            {
                total_risk += depth as u64 + 1;
            }
        }
    }

    println!("Total risk: {}", total_risk);
}
