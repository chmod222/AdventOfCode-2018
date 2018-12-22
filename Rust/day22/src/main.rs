use shared::grid::{self as g, GridTile, Grid, Coordinate};
use std::collections::{VecDeque, HashSet, HashMap};

#[derive(Debug, PartialEq)]
enum CaveTile {
    Rocky,
    Wet,
    Narrow
}

impl g::GridTile for CaveTile {
    fn to_char(&self) -> char {
        match self {
            CaveTile::Rocky => '.',
            CaveTile::Wet => '=',
            CaveTile::Narrow => '|'
        }
    }

    fn color(&self) -> g::TileColor {
        match self {
            CaveTile::Rocky => g::TileColor::Foreground((g::Color::White, g::Attribute::Bold)),
            CaveTile::Wet => g::TileColor::Foreground((g::Color::Blue, g::Attribute::Bold)),
            CaveTile::Narrow => g::TileColor::Foreground((g::Color::Red, g::Attribute::Bold))
        }
    }
}


fn generate_cave_system(depth: isize, (ty, tx): (isize, isize)) -> Vec<Vec<CaveTile>> {
    let mut cave = Vec::new();

    // Due to the cascading nature of the erosion levels, calculating on-the-fly
    // is extremely slow, so we use a hash map to remember the previous erosion levels
    let mut memo = HashMap::<(isize, isize), isize>::new();

    // Generate a map beyond the target coordinates because the shortest path down may meander
    // around it. 4x should do.

    for y in 0..=ty*4 {
        let mut r = Vec::new();

        for x in 0..=tx*4 {
            let geologic_index = match (y, x) {
                (0, 0) => 0,
                (y, x) if (y, x) == (ty, tx) => 0,
                (0, x) => x * 16807,
                (y, 0) => y * 48271,
                (y, x) => {
                    let r1 = memo[&(y, x - 1)];
                    let r2 = memo[&(y - 1, x)];

                    r1 * r2
                }
            };
            
            let erosion_level = (geologic_index + depth) % 20183;

            memo.insert((y, x), erosion_level);

            r.push(match erosion_level % 3 {
                0 => CaveTile::Rocky,
                1 => CaveTile::Wet,
                2 => CaveTile::Narrow,

                _ => unreachable!()
            });
        }

        cave.push(r);
    }

    cave
}

fn risk_level(cave: &Vec<Vec<CaveTile>>, (sy, sx): (isize, isize), (ty, tx): (isize, isize)) -> usize {
    let mut rl = 0;

    for y in sy..=ty {
        for x in sx..=tx {
            match cave[y as usize][x as usize] {
                CaveTile::Rocky => rl += 0,
                CaveTile::Wet => rl += 1,
                CaveTile::Narrow => rl += 2
            }
        }
    }

    rl
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Ord, PartialOrd, Hash)]
enum Tool {
    Neither,
    ClimbingGear,
    Torch
}

impl Tool {
    fn can_be_used_in(&self, t: &CaveTile) -> bool {
        match t {
            CaveTile::Rocky => match self {
                Tool::ClimbingGear | Tool::Torch => true,
                _ => false
            },

            CaveTile::Wet => match self {
                Tool::ClimbingGear | Tool::Neither => true,
                _ => false
            },

            CaveTile::Narrow => match self {
                Tool::Torch | Tool::Neither => true,
                _ => false
            }
        }
    }
}

struct Cave {
    cave: Vec<Vec<CaveTile>>,
}

impl Cave {
    fn new(inner: Vec<Vec<CaveTile>>) -> Cave {
        Cave {
            cave: inner,
        }
    }
}

impl g::Grid for Cave {
    type Coord = g::Coord;
    type Tile = CaveTile;

    fn bounds(&self) -> (Self::Coord, Self::Coord) {
        if self.cave.len() > 0 {
            (g::Coord(0, 0),
             g::Coord(self.cave.len() as _, self.cave[0].len() as _))
        } else {
            (g::Coord(0, 0),
             g::Coord(0, 0))
        }
    }

    fn tile_at(&self, coord: &Self::Coord) -> &Self::Tile {
        &self.cave[coord.y() as usize][coord.x() as usize]
    }
}


fn modified_bfs(cave: &Cave, start: g::Coord, target: g::Coord) -> Option<usize>
{
    let mut q = VecDeque::new();
    let mut s = HashSet::new();

    q.push_back((start, Tool::Torch, None, 0));
    s.insert((start, Tool::Torch));

    while let Some((coord, tool, switching, minutes)) = q.pop_front() {
        if let Some(switching) = switching {
            // This node is still switching to a new tool, decrease and go on
            if switching != 0 || s.insert((coord, tool)) {
                q.push_back((coord, tool, if switching == 0 { None } else { Some(switching - 1) }, minutes + 1));
            }

            continue;
        } else if coord == target && tool == Tool::Torch {
            // If we reached the end with the right tool, we just stop
            return Some(minutes);
        }

        // Try all neighbors with the given tool
        for neighbor in coord.neighbors() {
            if neighbor.y() < 0 || neighbor.x() < 0 
                || neighbor.y() >= cave.cave.len() as _
                || neighbor.x() >= cave.cave[neighbor.y() as usize].len() as _ {

                continue;
            }

            if tool.can_be_used_in(cave.tile_at(&neighbor)) && s.insert((neighbor, tool)) {
                q.push_back((neighbor, tool, None, minutes + 1));
            }
        }

        // Try same node again with all the other tools
        let tile = cave.tile_at(&coord);

        if *tile != CaveTile::Narrow { q.push_back((coord, Tool::ClimbingGear, Some(6), minutes)); }
        if *tile != CaveTile::Wet { q.push_back((coord, Tool::Torch, Some(6), minutes)); }
        if *tile != CaveTile::Rocky { q.push_back((coord, Tool::Neither, Some(6), minutes)); }
    }

    None
}

fn main() {
    const START: (isize, isize) = (0, 0);
    const TARGET: (isize, isize) = (751, 9);
    const DEPTH: isize = 11817;

    // const TARGET: (isize, isize) = (10, 10);
    // const DEPTH: isize = 510;

    let cave = Cave::new(generate_cave_system(DEPTH, TARGET));

    println!("Cave has a risk level of {}", risk_level(&cave.cave, (0, 0), TARGET));

    let shortest_path_time = modified_bfs(
        &cave,
        g::Coord(START.0, START.1),
        g::Coord(TARGET.0, TARGET.1));

    cave.draw();

    if let Some(path_len) = shortest_path_time {
        println!("Shortest path down is {} minutes", path_len);
    }
}
