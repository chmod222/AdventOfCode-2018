use shared::{
    grid::{self as sg, Grid, GridTile, Coordinate},
    input::read_stdin_lines
};

use lazy_static::*;
use regex::Regex;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Tile {
    Clay, Sand, Spring, Water, WaterAtRest
}

struct Map {
    data: Vec<Vec<Tile>>,
    xstart: usize
}


impl Map {
    fn new(mut data: Vec<Vec<Tile>>, xstart: usize) -> Self {
        data[0][500 - xstart + 2] = Tile::Spring;

        Map {
            data,
            xstart
        }
    }

    fn flow_down(&mut self, y: usize, x: usize) -> (usize, usize) {
        for yf in y+1 .. self.data.len() {
            if self.data[yf][x] == Tile::Sand {
                self.data[yf][x] = Tile::Water;
            } else if self.data[yf][x] != Tile::Water {
                return (yf - 1, x);
            }
        }

        (self.data.len(), x)
    }

    fn check_enclosed(&self, y: usize, x: usize) -> (Option<usize>, Option<usize>) {
        // Check left
        let (mut enclosed_left, mut enclosed_right) = (None, None);

        for xf in (0..x).rev() {
            if self.data[y][xf] == Tile::Clay
                || self.data[y + 1][xf] == Tile::Sand
            {
                enclosed_left = Some(xf + 1);
                break;
            }
        }

        for xf in x..self.data[y].len() {
            if self.data[y][xf] == Tile::Clay 
                || self.data[y + 1][xf] == Tile::Sand
            {
                enclosed_right = Some(xf);
                break;
            }
        }

        (enclosed_left, enclosed_right)
    }

    fn update(&mut self) -> bool {
        // Find next flowable water tile
        let mut last = None;

        /* Lots of trial and error'd code below, lots of redundancies probably */
        'outer: for y in (0..self.data.len() - 1).rev() {
            for x in 0..self.data[y].len() {
                match self.data[y][x] {
                    Tile::Spring | Tile::Water => {
                        let below = self.data[y + 1][x];

                        // Do not try to flow water that's already flowing
                        if x > 0 && x < self.data[y].len() - 1
                            && self.data[y][x - 1] == Tile::Water
                            && self.data[y][x + 1] == Tile::Water {
                            continue;
                        }

                        // Do not try to flow flowing water that's stopped at a left wall
                        if x > 0 && self.data[y][x - 1] == Tile::Clay 
                            && self.data[y][x + 1] == Tile::Water {
                            continue;
                        }

                        // Do not try to flow flowing water that's stopped at a right wall
                        if x < self.data[y].len() - 1 && self.data[y][x + 1] == Tile::Clay
                            && x > 0 && self.data[y][x - 1] == Tile::Water {
                            continue;
                        }

                        // Try to flow water that is either hanging over sand or over resting water
                        if below == Tile::Sand || below == Tile::WaterAtRest {
                            last = Some(self.flow_down(y, x));

                            if last.unwrap().0 < self.data.len() {
                                break 'outer;
                            }
                        }
                    }
                    _ => ()
                }
            }
        }

        // No flowable water found, we are done here
        if let None = last {
            return false;
        }
        
        let last = last.unwrap();

        if last.0 == self.data.len() {
            return false; // Dead flow
        }

        let (mut ec_left, mut ec_right) = self.check_enclosed(last.0, last.1);

        // Normalize enclosed spaces and resolve drops in the flow_right and flow_left helpers
        if let Some(l) = ec_left {
            if self.data[last.0 + 1][l - 1] == Tile::Sand {
                ec_left = None;
            }
        }  
        
        if let Some(r) = ec_right {
            if self.data[last.0 + 1][r] == Tile::Sand {
                ec_right = None;
            }
        }

        // Keep flowing right until stopped or dropping
        let flow_right = |data: &mut Vec<Vec<Tile>>, tile| {
            for x in last.1.. {
                data[last.0][x] = tile;

                if data[last.0 + 1][x] == Tile::Sand || data[last.0][x + 1] == Tile::Water
                 || x == data[last.0].len() - 1 || data[last.0][x + 1] == Tile::Clay {
                    break;
                }
            }
        };

        // Keep flowing left until stopped or dropping
        let flow_left = |data: &mut Vec<Vec<Tile>>, tile| {
            for x in (0..last.1 + 1).rev() {
                data[last.0][x] = tile;

                if data[last.0 + 1][x] == Tile::Sand
                    || x == 0 || data[last.0][x - 1] == Tile::Clay || data[last.0][x - 1] == Tile::Water {
                    break;
                }
            }
        };

        // Try to pool or keep flowing sideways
        match (ec_left, ec_right) {
            (Some(l), Some(r)) => {
                // Enclosed on both sides, pool up
                for x in l..r {
                    self.data[last.0][x] = Tile::WaterAtRest;
                }
            },

            _ => {
                // Flow both directions until drop or stop
                flow_left(&mut self.data, Tile::Water);
                flow_right(&mut self.data, Tile::Water);
            }
        }

        // Extend spouts left and right that can be dropped in the next iteration if not fully enclosed
        if let Some(l) = ec_left {
            if self.data[last.0 + 1][l - 1] == Tile::Sand {
                self.data[last.0][l - 1] = Tile::Water;
            }
        }

        if let Some(r) = ec_right {
            if self.data[last.0 + 1][r] == Tile::Sand {
                self.data[last.0][r] = Tile::Water;
            }
        }

        true
    }

    fn count_water(&self, ystart: usize, yend: usize) -> (usize, usize) {
        let (mut flowing, mut at_rest) = (0, 0);

        for y in ystart..=yend {
            for x in 0..self.data[0].len() {
                match self.data[y][x] {
                    Tile::Water => flowing += 1,
                    Tile::WaterAtRest => at_rest += 1,

                    _ => ()
                }
            }
        }

        (flowing, at_rest)
    }
}

impl sg::Grid for Map {
    type Coord = sg::Coord;
    type Tile = Tile;

    fn bounds(&self) -> (Self::Coord, Self::Coord) {
        (sg::Coord::new(0, 0),
         sg::Coord::new(self.data.len(), self.data[0].len()))
    }

    fn tile_at(&self, c: &Self::Coord) -> &Self::Tile {
        &self.data[c.y()][c.x()]
    }
}

impl sg::GridTile for Tile {
    fn to_char(&self) -> char {
        match self {
            Tile::Clay => '#',
            Tile::Sand => '.',
            Tile::Spring => '+',
            Tile::Water => '|',
            Tile::WaterAtRest => '~'
        }
    }

    fn color(&self) -> sg::TileColor {
        match self {
            Tile::Clay =>  sg::TileColor::Foreground((sg::Color::Yellow, sg::Attribute::Bold)),
            Tile::Sand =>  sg::TileColor::Foreground((sg::Color::Yellow, sg::Attribute::None)),
            Tile::Spring =>  sg::TileColor::Foreground((sg::Color::Blue, sg::Attribute::Bold)),
            Tile::Water | Tile::WaterAtRest => sg::TileColor::Foreground((sg::Color::Blue, sg::Attribute::None))
        }
    }
}

use std::ops::RangeInclusive;

#[derive(Debug)]
enum ScanEntry {
    RangeY(RangeInclusive<usize>, usize),
    RangeX(usize, RangeInclusive<usize>)
}

impl ScanEntry {
    fn parse(e: &String) -> Option<ScanEntry> {
        lazy_static! {
            static ref PAT: Regex = Regex::new(r"(x|y)=(\d+)(?:..(\d+))?").unwrap();
        }

        let mut captures = PAT.captures_iter(e);

        let first_coord = captures.next()?;
        let second_coord = captures.next()?;

        if first_coord.get(1)?.as_str() == "x" {
            let x: usize = first_coord.get(2)?.as_str().parse().ok()?;
            let ys = second_coord.get(2)?.as_str().parse().ok()?;
            let ye = second_coord.get(3)?.as_str().parse().ok()?;

            Some(ScanEntry::RangeY(ys..=ye, x + 1))
        } else if first_coord.get(1)?.as_str() == "y" {
            let y = first_coord.get(2)?.as_str().parse().ok()?;
            let xs: usize = second_coord.get(2)?.as_str().parse().ok()?;
            let xe: usize = second_coord.get(3)?.as_str().parse().ok()?;

            Some(ScanEntry::RangeX(y, (xs + 1)..=(xe + 1 )))
        } else {
            None
        }
    }
}

use std::collections::HashSet;

fn main() {
    let input = read_stdin_lines().expect("could not lock stdin");
    let mut results = input.iter().filter_map(ScanEntry::parse).collect::<Vec<_>>();

    let (cmin, cmax) = sg::Coord::numeric_limits();

    let (mut min, mut max) = (sg::Coord::new(cmax, cmax), sg::Coord::new(cmin, cmin));

    let clay = results.iter_mut().fold(HashSet::new(), |mut hs, res| {
        let mut update = |c: sg::Coord| {
            hs.insert(c);

            if c.0 > max.0 { max.0 = c.0; }
            if c.1 > max.1 { max.1 = c.1; }
            if c.0 < min.0 { min.0 = c.0; }
            if c.1 < min.1 { min.1 = c.1; }
        };

        match res {
            ScanEntry::RangeX(y, xr) => {
                for x in xr {
                    update(sg::Coord::new(*y, x));
                }
            },
            ScanEntry::RangeY(yr, x) => {
                for y in yr {
                    update(sg::Coord::new(y, *x));
                }
            }
        }

        hs
    });

    let mut grid = Vec::with_capacity(max.y());

    for y in 0..=max.y() {
        let mut r = Vec::with_capacity(max.x() - min.x());

        for x in min.x() - 1 ..= max.x() + 1 {
            if clay.contains(&sg::Coord::new(y, x)) {
                r.push(Tile::Clay);
            } else {
                r.push(Tile::Sand);
            }
        }

        grid.push(r);
    }

    let mut grid = Map::new(grid, min.x());

    for y in 0.. {
        if !grid.update() {
            break;
        }
    }

    grid.draw();

    let (f, r) = grid.count_water(min.y(), max.y());

    println!("Part 1: Total water tiles: {}", f + r);
    println!("Part 2: Remaining tiles: {}", r);
}
