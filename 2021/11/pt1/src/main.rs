use std::env::args;
use std::fs::read_to_string;

fn main() {
    let filename = args().nth(1).unwrap_or_else(|| "../input.txt".to_owned());
    let input_txt = read_to_string(filename).unwrap();

    let mut map = input_txt.lines()
        .map(|s| s.bytes().map(|b| b - b'0').collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    let mut flashes = 0u64;

    for _ in 0..100 {
        let mut next_flashes = Vec::new();

        for y in 0..10 {
            for x in 0..10 {
                map[y][x] += 1;

                if map[y][x] > 9 {
                    next_flashes.push((y, x));
                    map[y][x] = 0;
                }
            }
        }

        while !next_flashes.is_empty() {
            flashes += 1;

            let (y, x) = next_flashes.pop().unwrap();

            if y > 0 && map[y - 1][x] > 0 {
                map[y - 1][x] += 1;

                if map[y - 1][x] > 9 {
                    next_flashes.push((y - 1, x));
                    map[y - 1][x] = 0;
                }
            }
            if y < 9 && map[y + 1][x] > 0 {
                map[y + 1][x] += 1;

                if map[y + 1][x] > 9 {
                    next_flashes.push((y + 1, x));
                    map[y + 1][x] = 0;
                }
            }
            if x > 0 && map[y][x - 1] > 0 {
                map[y][x - 1] += 1;

                if map[y][x - 1] > 9 {
                    next_flashes.push((y, x - 1));
                    map[y][x - 1] = 0;
                }
            }
            if x < 9 && map[y][x + 1] > 0 {
                map[y][x + 1] += 1;

                if map[y][x + 1] > 9 {
                    next_flashes.push((y, x + 1));
                    map[y][x + 1] = 0;
                }
            }
            if x > 0 && y > 0 && map[y - 1][x - 1] > 0 {
                map[y - 1][x - 1] += 1;

                if map[y - 1][x - 1] > 9 {
                    next_flashes.push((y - 1, x - 1));
                    map[y - 1][x - 1] = 0;
                }
            }
            if x > 0 && y < 9 && map[y + 1][x - 1] > 0 {
                map[y + 1][x - 1] += 1;

                if map[y + 1][x - 1] > 9 {
                    next_flashes.push((y + 1, x - 1));
                    map[y + 1][x - 1] = 0;
                }
            }
            if x < 9 && y > 0 && map[y - 1][x + 1] > 0 {
                map[y - 1][x + 1] += 1;

                if map[y - 1][x + 1] > 9 {
                    next_flashes.push((y - 1, x + 1));
                    map[y - 1][x + 1] = 0;
                }
            }
            if x < 9 && y < 9 && map[y + 1][x + 1] > 0 {
                map[y + 1][x + 1] += 1;

                if map[y + 1][x + 1] > 9 {
                    next_flashes.push((y + 1, x + 1));
                    map[y + 1][x + 1] = 0;
                }
            }
        }
    }

    println!("Total flashes: {}", flashes);
}
