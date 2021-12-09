use std::collections::VecDeque;
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

    let mut top_basin_sizes: Vec<usize> = Vec::with_capacity(3);

    let mut compute_basin = |map: &mut Vec<Vec<u8>>, x: usize, y: usize| {
        let mut leafs: VecDeque<(usize, usize)> = VecDeque::new();
        let mut basin_size = 0;

        leafs.push_back((x, y));
        map[y][x] |= 0x80;

        while !leafs.is_empty() {
            let (x, y) = leafs.pop_front().unwrap();
            basin_size += 1;

            if x != 0 && map[y][x - 1] < 9 {
                map[y][x - 1] |= 0x80;
                leafs.push_back((x - 1, y));
            }
            if x != width - 1 && map[y][x + 1] < 9 {
                map[y][x + 1] |= 0x80;
                leafs.push_back((x + 1, y));
            }
            if y != 0 && map[y - 1][x] < 9 {
                map[y - 1][x] |= 0x80;
                leafs.push_back((x, y - 1));
            }
            if y != height - 1 && map[y + 1][x] < 9 {
                map[y + 1][x] |= 0x80;
                leafs.push_back((x, y + 1));
            }
        }

        if top_basin_sizes.is_empty() {
            top_basin_sizes.push(basin_size);
        } else {
            for (i, &size) in top_basin_sizes.iter().enumerate() {
                if basin_size > size {
                    top_basin_sizes.insert(i, basin_size);

                    if top_basin_sizes.len() > 3 {
                        top_basin_sizes.pop();
                    }
                    break;
                }
            }
        }
    };

    for y in 0..height {
        for x in 0..width {
            if map[y][x] < 9 {
                compute_basin(&mut map, x, y);
            }
        }
    }

    println!("Output: {}", top_basin_sizes[0] * top_basin_sizes[1] * top_basin_sizes[2]);
}
