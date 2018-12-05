use std::collections::{HashSet, HashMap};

fn contained_units(input: &str) -> HashSet<char> {
    input
        .as_bytes()
        .iter()
        .map(|c| (*c as char).to_ascii_lowercase())
        .collect()
}

fn react(input: &str) -> String {
    let mut out = input.to_string();

    loop {
        let mut removals = false;
        let mut cpy = out.clone();

        for (i, pair) in out.as_bytes().windows(2).enumerate() {
            if pair.len() != 2 {
                break;
            }

            let a = pair[0] as char;
            let b = pair[1] as char;

            if a.eq_ignore_ascii_case(&b)
                && ((a.is_ascii_uppercase() && !b.is_ascii_uppercase())
                || (!a.is_ascii_uppercase() && b.is_ascii_uppercase())) {

                cpy.remove(i);
                cpy.remove(i);

                removals = true;
                break;
            }
        }

        if !removals {
            break cpy;
        }

        out = cpy;
    }
}

fn main() {
    let raw_polymer = shared::input::read_stdin_lines().expect("could not lock stdin")[0].clone();
    let raw_result = react(&raw_polymer);

    println!("Part 1: {}", raw_result.len());

    let mut lens = HashMap::new();

    for unit in contained_units(&raw_polymer) {
        let mut filtered = raw_polymer.clone();

        filtered.retain(|c| c.to_ascii_lowercase() != unit);
    
        let reacted = react(&filtered);

        lens.insert(unit, reacted.len());

        println!("Part 2.{}: {}", unit, reacted.len());
    }

    let (km, vm) = lens.iter().min_by(|(_, v1), (_, v2)| v1.cmp(v2)).unwrap();

    println!("Part 2: {} when removing {}", vm, km);
}

#[cfg(test)]
mod test {
    use super::react;

    #[test]
    fn test_reaction() {
        assert_eq!(react("aA"), "");
        assert_eq!(react("abBA"), "");
        assert_eq!(react("abAB"), "abAB");
        assert_eq!(react("aabAAB"), "aabAAB");

        assert_eq!(react("dabAcCaCBAcCcaDA"), "dabCBAcaDA");
    }
}