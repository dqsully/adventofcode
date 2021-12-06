use std::env::args;
use std::fs::read_to_string;

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn parse(s: &str) -> Point {
        let (x, y) = s.split_once(',').expect("invalid point");

        Point {
            x: x.parse().expect("invalid int for x"),
            y: y.parse().expect("invalid int for y"),
        }
    }
}

enum Alignment {
    None,
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy)]
struct Line(Point, Point);

impl Line {
    fn parse(s: &str) -> Line {
        let (start, end) = s.split_once(" -> ").expect("invalid line");

        Line(Point::parse(start), Point::parse(end))
    }

    fn alignment(self) -> Alignment {
        if self.0.x == self.1.x {
            Alignment::Vertical
        } else if self.0.y == self.1.y {
            Alignment::Horizontal
        } else {
            Alignment::None
        }
    }
}

fn main() {
    let filename = args().nth(1).unwrap_or_else(|| "../input.txt".to_owned());
    let input_txt = read_to_string(filename).unwrap();

    let lines = input_txt.lines()
        .map(Line::parse);

    let mut floor = Box::new([0u8; 1000000]);
    let mut overlaps = 0;

    for line in lines {
        match line.alignment() {
            Alignment::Vertical => {
                let x = line.0.x;
                let y_range = if line.0.y > line.1.y {
                    line.1.y ..= line.0.y
                } else {
                    line.0.y ..= line.1.y
                };

                for y in y_range {
                    let c = &mut floor[x + y * 1000];

                    if *c < 2 {
                        *c += 1;

                        if *c == 2 {
                            overlaps += 1;
                        }
                    }
                }
            }
            Alignment::Horizontal => {
                let x_range = if line.0.x > line.1.x {
                    line.1.x ..= line.0.x
                } else {
                    line.0.x ..= line.1.x
                };
                let y = line.0.y;

                for x in x_range {
                    let c = &mut floor[x + y * 1000];

                    if *c < 2 {
                        *c += 1;

                        if *c == 2 {
                            overlaps += 1;
                        }
                    }
                }
            }
            Alignment::None => {
                let (x_range, x_reversed) = if line.0.x > line.1.x {
                    (line.1.x ..= line.0.x, true)
                } else {
                    (line.0.x ..= line.1.x, false)
                };
                let (y_range, y_reversed) = if line.0.y > line.1.y {
                    (line.1.y ..= line.0.y, true)
                } else {
                    (line.0.y ..= line.1.y, false)
                };

                if x_reversed ^ y_reversed {
                    for (x, y) in x_range.zip(y_range.rev()) {
                        let c = &mut floor[x + y * 1000];

                        if *c < 2 {
                            *c += 1;

                            if *c == 2 {
                                overlaps += 1;
                            }
                        }
                    }
                } else {
                    for (x, y) in x_range.zip(y_range) {
                        let c = &mut floor[x + y * 1000];

                        if *c < 2 {
                            *c += 1;

                            if *c == 2 {
                                overlaps += 1;
                            }
                        }
                    }
                }

            }
        }
    }

    println!("Overlaps: {}", overlaps);
}
