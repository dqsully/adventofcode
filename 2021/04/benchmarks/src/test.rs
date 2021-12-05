extern crate test;

use test::Bencher;
use super::*;
use std::fs::read_to_string;

fn run_test<B: Board>(bencher: &mut Bencher) {
    let input_txt = read_to_string("../input.txt").unwrap();

    let mut lines = input_txt.lines();
    let draws: Vec<u8> = lines.next().expect("missing first line")
        .split(',')
        .map(|s| s.parse::<u8>().expect("invalid draw"))
        .collect();

    let mut boards: Vec<B> = Vec::new();

    'board_parse_loop: loop {
        // Skip the empty line
        lines.next();

        let mut numbers = [0u8; 25];

        for i in 0..5 {
            let line = match lines.next() {
                Some(line) => line,
                None => break 'board_parse_loop,
            };

            for j in 0..5 {
                let num_str = &line[(j*3)..(j*3+2)];
                numbers[i * 5 + j] = num_str.trim_start().parse::<u8>().expect("not a number");
            }
        }

        boards.push(B::new(numbers))
    }

    bencher.iter(|| {
        let mut boards = boards.clone();
        let mut done_boards = Vec::new();

        for &draw in &draws {
            let boards_len = boards.len();

            for (i, board) in boards.iter_mut().enumerate() {
                if let Some(solution) = board.mark(draw) {
                    if boards_len - done_boards.len() == 1 {
                        println!("Solution: {}", solution);
                        return;
                    }

                    done_boards.push(i);
                }
            }

            for i in done_boards.iter().rev() {
                boards.swap_remove(*i);
            }

            done_boards.clear();
        }
    });
}

#[bench]
fn bitwise_smallest(b: &mut Bencher) {
    run_test::<bitwise_smallest::Board>(b);
}

#[bench]
fn bitwise_aligned(b: &mut Bencher) {
    run_test::<bitwise_aligned::Board>(b);
}

#[bench]
fn array_smallest(b: &mut Bencher) {
    run_test::<array_smallest::Board>(b);
}

#[bench]
fn array_16bit(b: &mut Bencher) {
    run_test::<array_16bit::Board>(b);
}

#[bench]
fn array_32bit(b: &mut Bencher) {
    run_test::<array_32bit::Board>(b);
}

#[bench]
fn array_aligned(b: &mut Bencher) {
    run_test::<array_aligned::Board>(b);
}

#[bench]
fn array_aligned_16bit(b: &mut Bencher) {
    run_test::<array_aligned_16bit::Board>(b);
}
