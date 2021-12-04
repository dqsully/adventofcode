use std::fs::read_to_string;
use std::collections::{HashMap, HashSet, BTreeMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opp(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

// Only two kinds of turns, even though they are only used in one way at a time
// NeSw means East turns to North and vice-versa, and West turns to South and vice-versa
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Turn {
    NeSw,
    NwSe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TurnChoice {
    Left,
    Straight,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cart {
    dir: Direction,
    choice: TurnChoice,
}

impl Cart {
    fn new(dir: Direction) -> Cart {
        Cart {
            dir,
            choice: TurnChoice::Left,
        }
    }
}

fn main() {
    let input = read_to_string("input").expect("could not read input file");

    let mut carts = BTreeMap::new();
    let mut turns = HashMap::new();
    let mut intersections = HashSet::new();

    let mut width = 0;
    let mut height = 0;

    input.split_terminator('\n').enumerate()
        .for_each(|(y, line)| {
            line.bytes().enumerate()
                .for_each(|(x, c)| match c {
                    b'/' => {
                        turns.insert((x, y), Turn::NeSw);
                    }
                    b'\\' => {
                        turns.insert((x, y), Turn::NwSe);
                    }
                    b'^' => {
                        carts.insert((x, y), Cart::new(Direction::North));
                    }
                    b'>' => {
                        carts.insert((x, y), Cart::new(Direction::East));
                    }
                    b'v' => {
                        carts.insert((x, y), Cart::new(Direction::South));
                    }
                    b'<' => {
                        carts.insert((x, y), Cart::new(Direction::West));
                    }
                    b'+' => {
                        intersections.insert((x, y));
                    }
                    _ => {}
                });

            height = y + 1;
            if line.len() > width {
                width = line.len();
            }
        });

    println!("{:?}", (width, height));
    let mut cart_count = carts.len();
    let mut cycles = 0;

    while carts.len() > 1 {
        // carts.iter()
        //     .for_each(|cart| println!("{:?}", cart));
        // println!();

        // let mut out = String::with_capacity((width + 1) * height);
        //
        // (0..height).for_each(|y| {
        //     (0..width).for_each(|x| {
        //         if x == 143 && y == 43 {
        //             out += "\x1b[100m";
        //         }
        //
        //         if let Some(cart) = carts.get(&(x, y)) {
        //             out += "\x1b[1m\x1b[91m";
        //
        //             match cart.dir {
        //                 Direction::North => out += "^",
        //                 Direction::East => out += ">",
        //                 Direction::South => out += "v",
        //                 Direction::West => out += "<",
        //             }
        //
        //             out += "\x1b[0m";
        //         } else if let Some(turn) = turns.get(&(x, y)) {
        //             match turn {
        //                 Turn::NeSw => out += "/",
        //                 Turn::NwSe => out += "\\",
        //             }
        //         } else if intersections.contains(&(x, y)) {
        //             out += "+";
        //         } else {
        //             out += " ";
        //         }
        //
        //         if x == 143 && y == 43 {
        //             out += "\x1b[0m";
        //         }
        //     });
        //
        //     out += "\n";
        // });
        // println!("{}", out);

        let mut new_carts: BTreeMap<(usize, usize), Cart> = BTreeMap::new();

        for ((mut x, mut y), cart) in carts {
            let mut cart = cart;

            match cart.dir {
                Direction::North => y -= 1,
                Direction::East => x += 1,
                Direction::South => y += 1,
                Direction::West => x -= 1,
            }

            if let Some(turn) = turns.get(&(x, y)) {
                match turn {
                    Turn::NeSw => cart.dir = match cart.dir {
                        Direction::North => Direction::East,
                        Direction::East => Direction::North,
                        Direction::South => Direction::West,
                        Direction::West => Direction::South,
                    },
                    Turn::NwSe => cart.dir = match cart.dir {
                        Direction::North => Direction::West,
                        Direction::East => Direction::South,
                        Direction::South => Direction::East,
                        Direction::West => Direction::North,
                    },
                }
            } else if intersections.contains(&(x, y)) {
                match cart.choice {
                    TurnChoice::Left => {
                        cart.dir = match cart.dir {
                            Direction::North => Direction::West,
                            Direction::East => Direction::North,
                            Direction::South => Direction::East,
                            Direction::West => Direction::South,
                        };
                        cart.choice = TurnChoice::Straight;
                    }
                    TurnChoice::Right => {
                        cart.dir = match cart.dir {
                            Direction::North => Direction::East,
                            Direction::East => Direction::South,
                            Direction::South => Direction::West,
                            Direction::West => Direction::North,
                        };
                        cart.choice = TurnChoice::Left;
                    }
                    TurnChoice::Straight => cart.choice = TurnChoice::Right,
                }
            }

            // let carts_at_new_location = new_carts.entry((x, y))
            //     .and_modify(|carts: &mut Vec<Cart>| carts.push(cart.clone()))
            //     .or_insert_with(|| vec![cart]);
            let hit_cart = new_carts.insert((x, y), cart);

            if hit_cart.is_some() {
                new_carts.remove(&(x, y));
                cart_count -= 1;
                println!("{:?} ({} left)", (x, y), cart_count);
            }
        }

        let mut will_intersect = HashSet::new();

        new_carts.iter()
            .for_each(|(&(x, y), cart)| {
                match cart.dir {
                    Direction::North => if let Some(Cart {
                        dir: Direction::South,
                        ..
                    }) = new_carts.get(&(x, y - 1)) {
                        will_intersect.insert((x, y));
                        will_intersect.insert((x, y - 1));
                    }
                    Direction::East => if let Some(Cart {
                        dir: Direction::West,
                        ..
                    }) = new_carts.get(&(x + 1, y)) {
                        will_intersect.insert((x, y));
                        will_intersect.insert((x + 1, y));
                    }
                    Direction::South => if let Some(Cart {
                        dir: Direction::North,
                        ..
                    }) = new_carts.get(&(x, y + 1)) {
                        will_intersect.insert((x, y));
                        will_intersect.insert((x, y + 1));
                    }
                    Direction::West => if let Some(Cart {
                        dir: Direction::East,
                        ..
                    }) = new_carts.get(&(x - 1, y)) {
                        will_intersect.insert((x, y));
                        will_intersect.insert((x - 1, y));
                    }
                }
            });

        will_intersect.into_iter()
            .for_each(|(x, y)| {
                new_carts.remove(&(x, y));
                cart_count -= 1;
                println!("{:?} ({} left)", (x, y), cart_count);
            });

        carts = new_carts;
        cycles += 1;

        // ::std::thread::sleep(::std::time::Duration::from_millis(10));
    };

    println!("{:?}", carts.iter().next().unwrap());
    println!("{} cycles", cycles);
}
