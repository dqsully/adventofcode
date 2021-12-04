use std::env::args;
use std::fs::read_to_string;

#[derive(Default)]
struct Bit {
    zero_population: usize,
    zeros: Option<Box<Bit>>,

    one_population: usize,
    ones: Option<Box<Bit>>,
}

fn main() {
    let filename = args().nth(1).unwrap_or_else(|| "../input.txt".to_owned());
    let input_txt = read_to_string(filename).unwrap();

    let mut first_bit: Bit = Default::default();

    for line in input_txt.split_terminator('\n') {
        let mut bit = &mut first_bit;

        for b in line.bytes() {
            if b == b'1' {
                bit.one_population += 1;
                bit = bit.ones.get_or_insert_with(Default::default);
            } else {
                bit.zero_population += 1;
                bit = bit.zeros.get_or_insert_with(Default::default);
            }
        }
    }

    let mut oxy_gen_rating = 0u32;
    let mut co2_scrub_rating = 0u32;

    let mut oxy_bit = &first_bit;
    let mut co2_bit = &first_bit;

    loop {
        if oxy_bit.zero_population + oxy_bit.one_population == 0 {
            break
        }
        if co2_bit.zero_population + co2_bit.one_population == 0 {
            break
        }

        oxy_gen_rating <<= 1;
        co2_scrub_rating <<= 1;

        let next_oxy_bit = if oxy_bit.one_population >= oxy_bit.zero_population {
            oxy_gen_rating |= 1;
            &oxy_bit.ones
        } else {
            &oxy_bit.zeros
        };

        let next_co2_bit = if co2_bit.one_population < co2_bit.zero_population && co2_bit.one_population > 0 {
            co2_scrub_rating |= 1;
            &co2_bit.ones
        } else {
            &co2_bit.zeros
        };

        match (next_oxy_bit, next_co2_bit) {
            (Some(next_oxy_bit), Some(next_co2_bit)) => {
                oxy_bit = next_oxy_bit;
                co2_bit = next_co2_bit;
            },
            _ => panic!("Inconsistent bit lengths")
        }
    }

    println!("{:b} * {:b} = {}", oxy_gen_rating, co2_scrub_rating, oxy_gen_rating * co2_scrub_rating)
}
