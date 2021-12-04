use std::env::args;
use std::fs::read_to_string;

const NUM_LEN: usize = 12;

fn main() {
    let filename = args().nth(1).unwrap_or_else(|| "../input.txt".to_owned());

    let input_txt = read_to_string(filename).unwrap();

    let mut zeroes = [0u32; NUM_LEN];
    let mut ones = [0u32; NUM_LEN];

    for line in input_txt.split_terminator('\n') {
        let line = line.as_bytes();

        for i in 0..NUM_LEN {
            if line[i] == b'1' {
                ones[i] += 1;
            } else {
                zeroes[i] += 1;
            }
        }
    }

    let mut gamma = 0u32;
    let mut epsilon = 0u32;

    for (z, o) in zeroes.into_iter().zip(ones.into_iter()) {
        gamma <<= 1;
        epsilon <<= 1;

        if z > o {
            epsilon |= 1;
        } else {
            gamma |= 1;
        }
    }

    println!("{}", gamma * epsilon)
}
