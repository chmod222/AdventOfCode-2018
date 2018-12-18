use shared::grid::{self as sg, Grid};
use shared::input as si;

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Debug, Hash)]
enum Acre {
    Ground, Tree, Lumberyard
}

impl sg::GridTile for Acre {
    fn to_char(&self) -> char {
        match self {
            Acre::Ground => '.',
            Acre::Tree => '|',
            Acre::Lumberyard => '#'
        }
    }

    fn color(&self) -> sg::TileColor {
        match self {
            Acre::Ground => sg::TileColor::Foreground((sg::Color::Yellow, sg::Attribute::None)),
            Acre::Tree => sg::TileColor::Foreground((sg::Color::Green, sg::Attribute::Bold)),
            Acre::Lumberyard => sg::TileColor::Foreground((sg::Color::Magenta, sg::Attribute::None))
        }
    }
}

fn count_adjacent(acre: &Vec<Vec<Acre>>, y: usize, x: usize) -> (usize, usize) {
    let y = y as isize;
    let x = x as isize;

    let (mut c_tree, mut c_lumberyard) = (0, 0);

    for yo in -1..=1 {
        for xo in -1..=1 {
            let yy = y + yo;
            let xx = x + xo;

            if yy == y && xx == x
                || yy < 0 || yy >= acre.len() as isize
                || xx < 0 || xx >= acre[0].len() as isize {
                continue;
            }

            match acre[yy as usize][xx as usize] {
                Acre::Lumberyard => c_lumberyard += 1,
                Acre::Tree => c_tree += 1,
                _ => ()
            }
        }
    }

    (c_lumberyard, c_tree)
}

fn run_cellular_automaton(acre: &mut Vec<Vec<Acre>>) {
    let mut new = acre.clone();

    for y in 0..acre.len() {
        for x in 0..acre[y].len() {
            let (adj_lumberyard, adj_tree) = count_adjacent(acre, y, x);

            new[y][x] = match acre[y][x] {
                Acre::Ground => {
                    // -> Tree if three surrounding are tree
                    if adj_tree >= 3 {
                        Acre::Tree
                    } else {
                        Acre::Ground
                    }
                },

                Acre::Tree => {
                    // -> Lumberyard if three surrounding are lumberyard
                    if adj_lumberyard >= 3 {
                        Acre::Lumberyard
                    } else {
                        Acre::Tree
                    }
                },

                Acre::Lumberyard => {
                    // -> Lumberyard if one surrounding lumberyard, else ground
                    if adj_lumberyard >= 1 && adj_tree >= 1 {
                        Acre::Lumberyard
                    } else {
                        Acre::Ground
                    }
                }
            }
        }
    }

    std::mem::swap(&mut new, acre);
}

fn read_tile(c: char) -> Option<Acre> {
    match c {
        '.' => Some(Acre::Ground),
        '|' => Some(Acre::Tree),
        '#' => Some(Acre::Lumberyard),
        _ => None
    }
}

fn total_resource(map: &Vec<Vec<Acre>>) -> usize {
    let (mut c_tree, mut c_lumber) = (0, 0);

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == Acre::Tree {
                c_tree += 1;
            } else if map[y][x] == Acre::Lumberyard {
                c_lumber += 1;
            }
        }
    }

    c_tree * c_lumber
}

use std::collections::HashMap;

fn main() {
    let input = si::read_stdin_lines().expect("could not lock stdin");
    
    let mut m: Vec<Vec<Acre>> =
        input
            .iter().map(|l|
                l.chars().filter_map(read_tile).collect())
            .collect();

    m.draw();

    let mut part1 = 0;
    let mut known = HashMap::new();
    let mut gen = 0;

    const GENERATIONS: i32 = 1_000_000_000;

    let (period, start) = loop {
        // Jump up the cursor to get a nice update
        print!("\x1b[{}A", m.len());
        
        run_cellular_automaton(&mut m);
        m.draw();

        // Enjoy the colors
        std::thread::sleep(std::time::Duration::from_millis(15));

        let s = total_resource(&m);

        if gen == 9 {
            part1 = s;
        }

        if known.contains_key(&m) {
            let gens_last_repeat = known[&m];

            break (gen - gens_last_repeat, gens_last_repeat);
        }
        
        // Just stuff the entire map into the hashmap and call it a day, this way we
        // avoid having to run a few generations to let it settle in order to avoid
        // false positives due to accidental score matches on different patterns.
        known.insert(m.clone(), gen);

        gen += 1;
    };

    println!("Pattern at {} repeats every {} cycles", gen, period);

    let gen = ((GENERATIONS - start) / period * period) + start + 1;

    for _gen in gen..GENERATIONS {
        run_cellular_automaton(&mut m);
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", total_resource(&m));
}
