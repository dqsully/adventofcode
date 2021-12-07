use std::env::args;
use std::fs::read_to_string;

fn main() {
    let filename = args().nth(1).unwrap_or_else(|| "../input.txt".to_owned());
    let input_txt = read_to_string(filename).unwrap();

    let mut nums: Vec<i32> = input_txt.split(',')
        .map(|s| s.trim().parse::<i32>().expect("invalid int"))
        .collect();

    nums.sort_unstable();

    let mut fuel = 0;

    for i in 0..nums.len()/2 {
        fuel += nums[nums.len() - i - 1] - nums[i];
    }

    println!("Total fuel: {}", fuel)
}
