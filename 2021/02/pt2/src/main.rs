use std::env::args;
use std::fs::read_to_string;


fn main() {
    let filename = args().nth(1).unwrap_or_else(|| "../input.txt".to_owned());

    let input_txt = read_to_string(filename).unwrap();

    let (_aim, x, y) = input_txt.split_terminator('\n')
        .fold((0, 0, 0), |(aim, x, y), s| {
            let (direction, magnitude) = s.split_once(' ').unwrap();
            let magnitude = magnitude.parse::<i32>().unwrap();

            match direction {
                "forward" => (aim, x + magnitude, y + magnitude * aim),
                "down" => (aim + magnitude, x, y),
                "up" => (aim - magnitude, x, y),
                _ => panic!("Unknown direction: {}", direction),
            }
        });

    println!("{}", x * y);
}
