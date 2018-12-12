#[derive(Debug)]
struct Rule {
    test: [bool; 5],
    result: bool
}

use std::collections::HashSet;

use std::str::FromStr;

//  0: ................................#..#.#..##......###...###................................
//  => 145
//  1: ................................#...#....#.....#..#..#..#................................
//  => 91
//  2: ................................##..##...##....#..#..#..##...............................
//  => 132
//  3: ...............................#.#...#..#.#....#..#..#...#...............................
//  => 102
//  4: ................................#.#..#...#.#...#..#..##..##..............................
//  => 154
//  5: .................................#...##...#.#..#..#...#...#..............................
//  => 115
//  6: .................................##.#.#....#...#..##..##..##.............................
//  => 174
//  7: ................................#..###.#...##..#...#...#...#.............................
//  => 126

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let mut test = [false; 5];

        for i in 0..test.len() {
            test[i] = chars.next().ok_or(())? == '#';
        }

        let mut chars = chars.skip(4);

        let result = chars.next().ok_or(())? == '#';

        Ok(Rule {
            test,
            result
        })
    }
}

fn simulate(prev_gen: &HashSet<i32>, next_gen: &mut HashSet<i32>, rules: &[Rule]) {
    let min = *prev_gen.iter().min().unwrap();
    let max = *prev_gen.iter().max().unwrap();

    next_gen.clear();

    const N: i32 = 2;

    for rule in rules {
        for i in min - N .. max + N {
            let a = prev_gen.contains(&(i - 2));
            let b = prev_gen.contains(&(i - 1));
            let c = prev_gen.contains(&i);
            let d = prev_gen.contains(&(i + 1));
            let e = prev_gen.contains(&(i + 2));

            if rule.test == [a,b,c,d,e] && rule.result {
                next_gen.insert(i);
            }
        }
    }
}

macro_rules! generate {
    ($cur:expr, $next:expr, $rules:expr) => {
        simulate(&$cur, &mut $next, &$rules);

        std::mem::swap(&mut $cur, &mut $next);
    }
}

fn main() {
    let input = shared::input::read_stdin_lines().expect("could not lock stdin");
    let (init, r) = input.split_first().unwrap();

    let mut cur_gen: HashSet<i32> = init
        .split(" ")
        .skip(2)
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .map(|(i, _)| i as i32)
        .collect();

    let rules = r.iter().filter_map(|s| s.parse().ok()).collect::<Vec<Rule>>();

    let mut next_gen = HashSet::new();

    // Calculate part 1
    for _ in 0..20 {
        generate!(cur_gen, next_gen, rules);
    }

    let sum = cur_gen.iter().cloned().sum::<i32>();

    println!("Part 1: {}", sum);

    // Kick it a few more times until it has done close to 1000 generations
    for _ in 20..999 {
        generate!(cur_gen, next_gen, rules);
    }

    // 999
    let sum1 = cur_gen.iter().cloned().sum::<i32>();
    
    generate!(cur_gen, next_gen, rules);

    // 1000
    let sum2 = cur_gen.iter().cloned().sum::<i32>();

    let diff = (sum2 - sum1) as usize;

    // We know that at 1000 iterations, the sum is sum2 and it increases by (sum2 - sum1)
    // thus we should assume that sum2 + (50_000_000 - 1000) * diff should be the solution
    println!("Part 2: {}", sum2 as usize + (50_000_000_000_usize - 1000) * diff);
}
