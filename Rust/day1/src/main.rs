use std::collections::HashSet;

fn solve_part1(freqs: &[i32]) -> i32 {
    freqs.iter().sum()
}

fn solve_part2(freqs: &[i32]) -> i32 {
    let mut encountered = HashSet::new();

    let steps = freqs
        .iter()
        .cycle()
        .scan(0, |state, next| {
            *state += next;

            Some(*state)
        });

    for step in steps {
        if encountered.contains(&step) {
            return step;
        } else {
            encountered.insert(step);
        }
    }

    unreachable!();
}

fn main() {
    let changes = shared::input::read_stdin_lines()
        .expect("could not lock stdin")
        .iter()
        .map(|n|
            n.parse::<i32>().expect("bad numeral"))
        .collect::<Vec<_>>();
    
    println!("Part 1: {}", solve_part1(&changes));
    println!("Part 2: {}", solve_part2(&changes));
}
