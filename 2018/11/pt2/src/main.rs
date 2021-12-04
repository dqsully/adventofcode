// const GRID_SN: i32 = 18;
const GRID_SN: i32 = 2187;

fn power_level(x: i32, y: i32) -> i32 {
    let rack_id = x + 10;

    (rack_id * y + GRID_SN) * rack_id / 100 % 10 - 5
}

fn best_window(win_size: usize, grid: &Vec<Vec<i32>>) -> (usize, usize, i32) {
    grid.windows(win_size).enumerate()
        .map(|(y, rows)| {
            let (max_x, max_total) = rows.into_iter()
                .map(|row| row.windows(win_size))
                .fold(None, |vec: Option<Vec<i32>>, row| {
                    if let Some(mut vec) = vec {
                        vec.iter_mut()
                            .zip(row)
                            .for_each(|(sum, window)| *sum += window.iter().sum::<i32>());

                        Some(vec)
                    } else {
                        Some(row.map(|window| window.iter().sum()).collect())
                    }
                }).unwrap()
                .into_iter().enumerate()
                .max_by_key(|(_, c)| *c)
                .expect("should have a max (rows)");

            (y, max_x, max_total)
        })
        .max_by_key(|(_, _, c)| *c)
        .expect("should have a max (columns)")
}

fn main() {
    let grid: Vec<Vec<i32>> = (1 ..= 300).map(
        |y| (1 ..= 300).map(
            |x| power_level(x, y)
        ).collect()
    ).collect();

    let (size, (y, x, _)) = (1 ..= 300)
        .map(|size| (size, best_window(size, &grid)))
        .inspect(|v| println!("{:?}", v))
        .max_by_key(|(_, (_, _, c))| *c)
        .unwrap();
    // let (y, x, _) = best_window(3, &grid);

    // println!("{},{}", x, y);

    println!("{},{},{}", x + 1, y + 1, size);

    // (33 .. 33 + 3)
    //     .for_each(|x|
    //         (45 .. 45 + 3)
    //             .for_each(|y| println!("({}, {}) = {}", x, y, power_level(x, y)))
    //     )
    // println!("33,45: {}", power_level(33, 45));
}
