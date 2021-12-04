use std::collections::BTreeSet;

fn main() {
    // let input = "#..#.#..##......###...###";
    let input = "###.......##....#.#.#..###.##..##.....#....#.#.....##.###...###.#...###.###.#.###...#.####.##.#....#";
    let growth_patterns: BTreeSet<_> = vec![
        // "...##",
        // "..#..",
        // ".#...",
        // ".#.#.",
        // ".#.##",
        // ".##..",
        // ".####",
        // "#.#.#",
        // "#.###",
        // "##.#.",
        // "##.##",
        // "###..",
        // "###.#",
        // "####.",
        "..###",
        "..#.#",
        "##.##",
        ".###.",
        "#####",
        "#.#..",
        ".##..",
        "...#.",
        "#.##.",
        "..#..",
        "##...",
        "###.#",
        "#..#.",
        "#.###",
        "###..",
        ".#...",
        "#.#.#",
    ].into_iter()
        .map(|s| (
            s.as_bytes()[0] == b'#',
            s.as_bytes()[1] == b'#',
            s.as_bytes()[2] == b'#',
            s.as_bytes()[3] == b'#',
            s.as_bytes()[4] == b'#',
        ))
        .collect();
    const GENERATIONS: usize = 50000000000;

    let mut plants: Vec<bool> = input.bytes()
        .map(|c| c == b'#')
        .collect();
    let mut min: i32 = 0;
    let mut max: i32 = plants.len() as i32 - 1;

    // println!("0: ({}, {})", min, max);
    // plants.iter()
    //     .for_each(|has_plant| if *has_plant {
    //         print!("#")
    //     } else {
    //         print!(".")
    //     });
    // println!();
    // println!();

    let mut last_sum = (min ..= max).zip(plants.iter())
        .filter_map(|(i, &p)| if p {Some(i)} else {None})
        .sum();

    for n in 0..GENERATIONS {
        let mut new_min = None;
        let mut new_max = None;

        let mut new_plants: Vec<bool> = (min - 2 ..= max + 2)
            .map(|i| {
                let get_plant = |i: i32| {
                    if i < min || i > max {
                        false
                    } else {
                        plants[(i - min) as usize]
                    }
                };

                let near_plants = (
                    get_plant(i - 2),
                    get_plant(i - 1),
                    get_plant(i),
                    get_plant(i + 1),
                    get_plant(i + 2),
                );

                let is_plant = growth_patterns.contains(&near_plants);

                // println!("  {}: {}{}{}{}{} -> {}",
                //     i,
                //     if near_plants.0 {"#"} else {"."},
                //     if near_plants.1 {"#"} else {"."},
                //     if near_plants.2 {"#"} else {"."},
                //     if near_plants.3 {"#"} else {"."},
                //     if near_plants.4 {"#"} else {"."},
                //     if is_plant {"#"} else {"."}
                // );

                if is_plant {
                    if let None = new_min {
                        new_min = Some(i);
                    }
                    new_max = Some(i);
                }

                is_plant
            })
            .collect();

        plants = new_plants.split_off((new_min.unwrap() - (min - 2)) as usize);
        plants.split_off((new_max.unwrap() - new_min.unwrap() + 1) as usize);
        min = new_min.unwrap();
        max = new_max.unwrap();

        // println!("{}: ({}, {})", n + 1, min, max);
        // plants.iter()
        //     .for_each(|has_plant| if *has_plant {
        //         print!("#")
        //     } else {
        //         print!(".")
        //     });
        // println!();
        // println!();

        if (n + 1) % 10000 == 0 {
            let sum: i32 = (min ..= max).zip(plants.iter())
                .filter_map(|(i, &p)| if p {Some(i)} else {None})
                .sum();

            println!("{} -> {}\t(diff {})", n + 1, sum, sum - last_sum);

            last_sum = sum;
        }
    }

    let sum: i32 = (min ..= max).zip(plants.iter())
        .filter_map(|(i, &p)| if p {Some(i)} else {None})
        .sum();

    println!("sum: {}", sum);
}
