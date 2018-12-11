use rayon::prelude::*;

fn get_power_level(x: usize, y: usize, serial: i32) -> i32 {
    let rack_id = x + 10;
    let pwr = rack_id * y;
    let pwr = pwr + serial as usize;
    let pwr = pwr * rack_id;
    let pwr = pwr % 1000 / 100;
    let pwr = pwr as i32 - 5;

    pwr
}

fn get_grid_sum(x: usize, y: usize, serial: i32, grid_size: usize) -> i32 {
    let mut sum = 0;

    for y in y..y+grid_size {
        for x in x..x+grid_size {
            sum += get_power_level(x, y, serial)
        }
    }

    sum
}

fn main() {
    const SERIAL: i32 = 7672;
    
    const MAX: usize = 300;

    let mut max = i32::min_value();
    let mut maxc = (usize::default(), usize::default());

    // Part 1
    for y in 1..=MAX - 3 {
        for x in 1..=MAX - 3 {
            let pwr = get_grid_sum(x, y, SERIAL, 3);

            if pwr > max {
                max = pwr;
                maxc = (x, y);
            }
        }
    }

    println!("Part 1: x:{} y:{} = {}", maxc.0, maxc.1, max);

    // Part 2 - Could be doing this the smart way using a summed area table, but
    // I decided to rub 16 threads against the problem.
    let results: Vec<(usize, (usize, usize), i32)> = (1..301_usize).into_par_iter().map(|s| {
        let mut maxc = (0, 0);
        let mut maxs = usize::min_value();
        let mut max = i32::min_value();

        for y in 1..=MAX - maxs {
            for x in 1..=MAX - maxs {
                let pwr = get_grid_sum(x, y, SERIAL, s);

                if pwr > max {
                    max = pwr;
                    maxs = s;
                    maxc = (x, y);
                }
            }
        }

        (s, maxc, max)
    }).collect();

    let (bs, (bx, by), bp) = results.iter().max_by(|(_, _, n1), (_, _, n2)| n1.cmp(&n2)).unwrap();

    println!("Part 2: x:{} y:{} s:{} = {}", bx, by, bs, bp);
}
