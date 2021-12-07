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

    let lower = calculate_fuel(mean.floor() as i32);
    let upper = calculate_fuel(mean.ceil() as i32);

    println!("Total fuel: {}", lower.min(upper));
}
