use std::env::args;
use std::fs::read_to_string;


fn main() {
    let filename = args().nth(1).unwrap_or_else(|| "../input.txt".to_owned());

    let input_txt = read_to_string(filename).unwrap();

    let final_position = input_txt.split_terminator('\n')
        .map(|s| {
            let (direction, magnitude) = s.split_once(' ').unwrap();
            let magnitude = magnitude.parse::<i32>().unwrap();

            match direction {
                "forward" => (magnitude, 0),
                "down" => (0, magnitude),
                "up" => (0, -magnitude),
                _ => panic!("Unknown direction: {}", direction),
            }
        })
        .reduce(|(ax, ay), (bx, by)| (ax + bx, ay + by))
        .unwrap();

    println!("{}", final_position.0 * final_position.1);
}
