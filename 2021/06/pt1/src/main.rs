use std::env::args;
use std::fs::read_to_string;

struct Lanternfish(u8);

impl Lanternfish {
    fn next(&mut self) -> Option<Lanternfish> {
        if self.0 == 0 {
            self.0 = 6;
            Some(Lanternfish(8))
        } else {
            self.0 -= 1;
            None
        }
    }
}

fn simulate(fish: Lanternfish, days: usize) -> usize {
    let mut fishes = vec![fish];
    let mut new_fishes = vec![];

    for _ in 0..days {
        fishes.append(&mut new_fishes);

        for fish in &mut fishes {
            if let Some(new_fish) = fish.next() {
                new_fishes.push(new_fish);
            }
        }
    }

    fishes.len() + new_fishes.len()
}

const DAYS: usize = 80;

fn main() {
    let filename = args().nth(1).unwrap_or_else(|| "../input.txt".to_owned());
    let input_txt = read_to_string(filename).unwrap();

    let mut simulated = [0usize; 5];

    for (i, sim) in simulated.iter_mut().enumerate() {
        *sim = simulate(Lanternfish(i as u8 + 1), DAYS);
    }

    let fishes = input_txt.lines().next().expect("no first line")
        .split(',')
        .map(|s| s.parse::<u8>().expect("invalid u8"));

    let mut total = 0;

    for fish in fishes {
        total += simulated[fish as usize - 1];
    }

    println!("Total after {} days: {}", DAYS, total);
}
