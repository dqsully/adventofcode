use std::env::args;
use std::fs::read_to_string;

fn simulate(fish: u8, days: usize) -> usize {
    let mut fish_states = [0usize; 9];

    fish_states[fish as usize] = 1;

    for _ in 0..days {
        let parents = fish_states[0];

        fish_states.copy_within(1..9, 0);

        fish_states[6] += parents;
        fish_states[8] = parents;
    }

    fish_states.into_iter().sum()
}

const DAYS: usize = 256;

fn main() {
    let filename = args().nth(1).unwrap_or_else(|| "../input.txt".to_owned());
    let input_txt = read_to_string(filename).unwrap();

    let mut simulated = [0usize; 5];

    for (i, sim) in simulated.iter_mut().enumerate() {
        *sim = simulate(i as u8 + 1, DAYS);
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
