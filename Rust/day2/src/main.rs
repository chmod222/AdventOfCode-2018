fn test_repeats<T: std::borrow::Borrow<str>>(id: T) -> (bool, bool) {
    let mut has_two = false;
    let mut has_three = false;

    for c in b'a' ..= b'z' {
        let count = id.borrow().bytes().filter(|c2| *c2 == c).count();

        if count == 2 {
            has_two = true;
        } else if count == 3 {
            has_three = true;
        }
    }

    (has_two, has_three)
}

fn calculate_checksum<T>(ids: &[T]) -> i32
    where T: std::borrow::Borrow<str>
{
    let (s2, s3) = ids
        .iter()
        .map(|s| test_repeats(s.borrow()))
        .fold((0, 0), |(p2, p3), (n2, n3)| {
            (
                p2 + if n2 { 1 } else { 0 },
                p3 + if n3 { 1 } else { 0 }
            )
        });

    s2 * s3
}

fn id_match<T: std::borrow::Borrow<str>>(a: T, b: T) -> Option<usize> {
    let ab = a.borrow().as_bytes();
    let bb = b.borrow().as_bytes();

    let mut diffs = ab
        .iter()
        .zip(bb)
        .enumerate()
        .filter_map(|(i, (ac, bc))|
            if ac != bc {
                Some(i)
            } else {
                None
            });

    let head = diffs.next();

    if head.is_some() && diffs.count() == 0 {
        head
    } else {
        None
    }
}

fn find_target_packet<T>(ids: &[T]) -> String
    where T: std::borrow::Borrow<str>
{
    for i in 0 .. ids.len() {
        for j in 0 .. ids.len() {
            if i == j {
                continue;
            }

            if let Some(offset) = id_match(ids[i].borrow(), ids[j].borrow()) {
                let mut r = ids[i].borrow().to_string();

                r.remove(offset);

                return r;
            }
        }
    }

    unreachable!();
}

fn main() {
    let packets = shared::input::read_stdin_lines().expect("could not lock stdin");

    println!("Part 1: {:?}", calculate_checksum(&packets));
    println!("Part 2: {:?}", find_target_packet(&packets));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_checksum() {
        let ids1 = vec![
            ("abcdef", (false, false)),
            ("bababc", (true, true)),
            ("abbcde", (true, false)),
            ("abcccd", (false, true)),
            ("aabcdd", (true, false)),
            ("abcdee", (true, false)),
            ("ababab", (false, true))
        ];

        for (input, expected) in &ids1 {
            assert_eq!(super::test_repeats(*input), *expected);
        }

        assert_eq!(super::calculate_checksum(&ids1.iter().map(|(input, _)| input.clone()).collect::<Vec<_>>()), 12);

        // Part 2
        assert_eq!(super::id_match("fghij", "fguij"), Some(2));

        let ids2 = vec![
            "abcde", "fghij", "klmno", 
            "pqrst", "fguij", "axcye",
            "wvxyz"];

        assert_eq!(super::find_target_packet(&ids2), "fgij")
    }
}
