use std::collections::VecDeque;

struct Marbles {
    ring: VecDeque<usize>,
    next_marble: usize
}

impl Marbles {
    fn new(size: usize) -> Self {
        let mut ring = VecDeque::with_capacity(size);
        ring.push_back(0);

        Marbles {
            ring: ring,
            next_marble: 1
        }
    }

    fn place(&mut self) -> usize {
        let score = if self.next_marble == 0 || self.next_marble % 23 != 0 {
            for _ in 0..2 {
                let first = self.ring.pop_front().unwrap();
                self.ring.push_back(first);
            }

            self.ring.push_front(self.next_marble);

            0
        } else {
            let mut score = self.next_marble;

            for _ in 0..7 {
                let last = self.ring.pop_back().unwrap();
                self.ring.push_front(last);
            }

            score += self.ring.pop_front().unwrap();
            score
        };

        self.next_marble += 1;

        score
    }
}

const PLAYERS: usize = 459;
const HIGH: usize = 71320;

fn play_to_win(high: usize) -> (usize, usize) {
    let mut ring = Marbles::new(high);
    let mut scores = [0; PLAYERS];

    for i in 0..=high {
        scores[i % scores.len()] += ring.place();
        //ring.print();
    }

    scores
        .iter()
        .cloned()
        .enumerate()
        .max_by(|(_, s1), (_, s2)| s1.cmp(&s2))
        .unwrap()
}

fn main() {
    let (p1, s1) = play_to_win(HIGH);
    println!("Part 1: Winning score: Player {}: {}", p1, s1);

    let (p2, s2) = play_to_win(HIGH * 100);
    println!("Part 2: Winning score: Player {}: {}", p2, s2);
}
