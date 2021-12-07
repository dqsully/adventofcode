use std::env::args;
use std::fs::read_to_string;

fn main() {
    let filename = args().nth(1).unwrap_or_else(|| "../input.txt".to_owned());
    let input_txt = read_to_string(filename).unwrap();

    let nums: Vec<i32> = input_txt.split(',')
        .map(|s| s.trim().parse::<i32>().expect("invalid int"))
        .collect();

    let mean = (nums.iter().sum::<i32>() as f64) / (nums.len() as f64);

    let calculate_fuel = |mean: i32| {
        let mut fuel = 0;

        for &num in &nums {
            let x = (mean - num).abs();
            fuel += x * (x + 1) / 2;
        }

        fuel
    };

    let lower_int_bound = (mean - 0.5).floor() as i32;

    let lower_guess = calculate_fuel(lower_int_bound);
    let middle_guess = calculate_fuel(lower_int_bound + 1);
    let upper_guess = calculate_fuel(lower_int_bound + 2);

    println!("Total fuel: {}", lower_guess.min(middle_guess).min(upper_guess));
}
