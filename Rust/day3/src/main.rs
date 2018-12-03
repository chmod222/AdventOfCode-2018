const N: usize = 1000;

#[derive(Debug, PartialEq, Eq)]
struct Rect {
    x: usize,
    y: usize
}

#[derive(Debug)]
struct Claim {
    id: u32,

    start: Rect,
    size: Rect
}

impl Claim {
    fn from_string<T>(input: &T) -> Option<Self>
        where T: std::borrow::Borrow<str>
    {
        let mut parts = input.borrow().split(' ');

        let id = parts.next()?[1..].parse::<u32>().ok()?;

        let _ = parts.next()?;

        let start = {
            let raw = parts.next()?;
            let mut raw = raw[.. raw.len() - 1].split(',');

            Rect {
                x: raw.next()?.parse().ok()?,
                y: raw.next()?.parse().ok()?
            }
        };

        let size = {
            let mut raw = parts.next()?.split('x');

            Rect {
                // inclusive range, thus -1
                x: raw.next()?.parse::<usize>().ok()? - 1,
                y: raw.next()?.parse::<usize>().ok()? - 1
            }
        };
        
        Some(Claim {
            id,
            start,
            size
        })
    }

    fn apply_to<F>(&self, board: &mut Board, mut func: F)
        where F: FnMut(&mut u16)
    {
        for y in self.start.y .. self.start.y + self.size.y {
            for x in self.start.x .. self.start.x + self.size.x {
                func(&mut board[y][x]);
            }
        }
    }

    fn overlaps(&self, claim: &Claim) -> bool {
        let in_range = |x, min, max| (x >= min) && (x <= max);

        let yover = in_range(self.start.x, claim.start.x, claim.start.x + claim.size.x)
            || in_range(claim.start.x, self.start.x, self.start.x + self.size.x);
        let xover = in_range(self.start.y, claim.start.y, claim.start.y + claim.size.y)
            || in_range(claim.start.y, self.start.y, self.start.y + self.size.y);

        yover && xover
    }
}

fn count_shared(board: &Board) -> usize {
    board.iter().map(|row| row.iter().filter(|&&n| n > 1).count()).sum()
}

type Board = [[u16; N]; N];

fn main() {
    let board = Box::new([[0; N]; N]);

    let input = shared::input::read_stdin_lines()
        .expect("could not lock stdin")
        .iter()
        .filter_map(Claim::from_string)
        .collect::<Vec<_>>();

    let overlapping_claims = input.iter().fold(board.clone(), |mut acc, nxt| {
        nxt.apply_to(&mut acc, |cell| *cell += 1);

        acc
    });

    println!("Part 1:");
    println!("Shared claims: {}", count_shared(&overlapping_claims));

    println!();
    println!("Part 2:");

    let mut non_overlapping = Vec::new();

    'outer: for i in 0..input.len() {
        for j in 0..input.len() {
            if i == j {
                continue;
            }

            if input[i].overlaps(&input[j]) {
                continue 'outer;
            }
        }

        non_overlapping.push(&input[i]);
    }

    println!("Non-overlapping: {} ({} total)", non_overlapping[0].id, non_overlapping.len());
}
