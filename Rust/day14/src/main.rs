use std::iter::FromIterator;

const SCORE_CEIL: usize = 503761;
const SCORE_RECS: &[u8] = &[5, 0, 3, 7, 6, 1];

struct Recipes {
    recipes: Vec<u8>,
    positions: [usize; 2]
}

impl Recipes {
    fn new(init: &[u8]) -> Self {
        Recipes {
            recipes: Vec::from_iter(init.iter().cloned()),
            positions: [0, 1]
        }
    }

    fn print(&self) {
        for i in 0..self.recipes.len() {
            if i == self.positions[0] {
                print!("({}) ", self.recipes[i]);
            } else if i == self.positions[1] {
                print!("[{}] ", self.recipes[i]);
            } else {
                print!(" {}  ", self.recipes[i]);
            }
        }

        println!();
    }

    fn round(&mut self) {
        let (ca, cb) = (self.recipes[self.positions[0]], self.recipes[self.positions[1]]);
        let combined = ca + cb; 
        let (ra, rb) = (combined / 10, combined % 10);

        if ra > 0 {
            self.recipes.push(ra);
        }
        
        self.recipes.push(rb);

        self.positions[0] = (self.positions[0] + ca as usize + 1) % self.recipes.len();
        self.positions[1] = (self.positions[1] + cb as usize + 1) % self.recipes.len();
    }

    fn len(&self) -> usize {
        self.recipes.len()
    }

    fn score(&self, offset: usize) -> Option<&[u8]> {
        if self.len() < offset + 10 {
            None
        } else {
            Some(&self.recipes[offset..offset+10])
        }
    }

    fn locate(&self, needle: &[u8], last_n: usize) -> Option<usize> {
        //self.recipes.windows(needle.len()).position(|hay| hay == needle)
        if self.len() < needle.len() {
            return None
        }

        for i in self.recipes.len() - last_n..self.recipes.len() - needle.len() {
            if &self.recipes[i..i+needle.len()] == needle {
                return Some(i);
            }
        }

        None
    }
}

const RECIPE_INIT: &[u8] = &[3, 7];

fn part1() {
    let mut rec = Recipes::new(RECIPE_INIT);

    loop {
        match rec.score(SCORE_CEIL) {
            None => {
                rec.round();
                //rec.print();
            },

            Some(score) => {
                println!("Score: {:?}", score);

                break;
            }
        }
    }
}

fn part2() {
    let mut rec = Recipes::new(RECIPE_INIT);

    loop {
        // Since we know at most 2 new recipes can be added per round and everything until
        // now hasn't matched, there is no point in retrying everything all over. Always only
        // check the last N*2 recipes.
        match rec.locate(SCORE_RECS, SCORE_RECS.len() * 2) {
            None => {
                rec.round();
                //rec.print();
            },

            Some(idx) => {
                println!("Preceeding: {}", idx);

                break;
            }
        }
    }
}

fn main() {
    part1();
    part2();
}
