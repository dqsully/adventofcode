use std::env::args;
use std::fs::read_to_string;

// Unique based on number of bits per segment:
//
// #bits  bits             digit
//        g f e d c b a
//
// 2      0 0 0 0 0 1 1    d1
// 4      0 1 1 0 0 1 1    d4
// 3      0 0 0 1 0 1 1    d7
// 7      1 1 1 1 1 1 1    d8

// Remaining digits:
//
// #bits  bits             digit
//
// 6      1 0 1 1 1 1 1    d0
// 6      1 1 1 1 1 1 0    d6
// 6      0 1 1 1 1 1 1    d9
// 5      1 1 0 1 1 0 1    d2
// 5      0 1 1 1 1 1 0    d5
// 5      0 1 0 1 1 1 1    d3

// d1 & 6-count digits -> d6, (d0 or d9)
// d1 & 5-count digits -> d3, (d5 or d2)

// a = !d6
// (d5 or d2) & a -> d5, d2
// g = !(d4 | d3)
// (d0 or d9) & g -> d0, d9

fn solve_line(digits: &[u8; 10], output_reading: &[u8; 4]) -> u64 {
    // Use arrays (and an explicit index) to avoid heap allocations
    let mut known_digits = [0u8; 10];
    let mut unknown_digits = [0u8; 6];
    let mut unknown_digits_len=0;

    for &digit in digits {
        match digit.count_ones() {
            2 => known_digits[1] = digit,
            4 => known_digits[4] = digit,
            3 => known_digits[7] = digit,
            7 => known_digits[8] = digit, // Always going to be the same, but ¯\_(ツ)_/¯
            _ => {
                unknown_digits[unknown_digits_len] = digit;
                unknown_digits_len += 1;
            },
        }
    }

    let mut d2_or_d5 = [0u8; 2];
    let mut d0_or_d9 = [0u8; 2];

    for digit in unknown_digits {
        if digit.count_ones() == 6 {
            if (digit & known_digits[1]).count_ones() == 1 {
                known_digits[6] = digit
            } else if d0_or_d9[0] == 0 {
                d0_or_d9[0] = digit;
            } else {
                d0_or_d9[1] = digit;
            }
        } else if (digit & known_digits[1]).count_ones() == 2 {
            known_digits[3] = digit;
        } else if d2_or_d5[0] == 0 {
            d2_or_d5[0] = digit;
        } else {
            d2_or_d5[1] = digit;
        }
    }

    let a = !known_digits[6];

    if d2_or_d5[0] & a != 0 {
        known_digits[2] = d2_or_d5[0];
        known_digits[5] = d2_or_d5[1];
    } else {
        known_digits[5] = d2_or_d5[0];
        known_digits[2] = d2_or_d5[1];
    }

    let g = !(known_digits[4] | known_digits[3]);

    if d0_or_d9[0] & g != 0 {
        known_digits[0] = d0_or_d9[0];
        known_digits[9] = d0_or_d9[1];
    } else {
        known_digits[9] = d0_or_d9[0];
        known_digits[0] = d0_or_d9[1];
    }

    let mut output = 0;

    'output_digit_loop: for &digit in output_reading.iter() {
        for (j, &known_digit) in known_digits.iter().enumerate() {
            if digit == known_digit {
                output = output * 10 + j as u64;
                continue 'output_digit_loop
            }
        }

        panic!("missing digit!: {}, {:?}", digit, known_digits);
    }

    output
}

fn parse_digit_str(digit_str: &str) -> u8 {
    let mut digit = 0;

    for letter in digit_str.bytes() {
        digit |= 1 << (letter - b'a');
    }

    digit
}

fn main() {
    let filename = args().nth(1).unwrap_or_else(|| "../input.txt".to_owned());
    let input_txt = read_to_string(filename).unwrap();

    let mut sum = 0;

    for line in input_txt.lines() {
        let digit_strs = line.split_once('|').unwrap();

        let mut digits = [0u8; 10];

        for (i, digit_str) in digit_strs.0.trim().split_whitespace().enumerate() {
            digits[i] = parse_digit_str(digit_str);
        }

        let mut output_reading = [0u8; 4];

        for (i, digit_str) in digit_strs.1.trim().split_whitespace().enumerate() {
            output_reading[i] = parse_digit_str(digit_str);
        }

        sum += solve_line(&digits, &output_reading);
    }

    println!("Sum of readings: {}", sum);
}
