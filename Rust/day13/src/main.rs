#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum CartDirection {
    North, // ^
    East,  // >
    South, // v
    West   // <
}

impl CartDirection {
    fn right(self) -> Self {
        match self {
            CartDirection::North => CartDirection::East,
            CartDirection::East => CartDirection::South,
            CartDirection::South => CartDirection::West,
            CartDirection::West => CartDirection::North
        }
    }

    fn left(self) -> Self {
        // Two wrongs don't make a right but three rights make a left
        self.right().right().right()
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum TrackDirection {
    NorthSouth,
    EastWest,
    Crossing
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum CornerConnection {
    SouthWest, // / from v and <
    SouthEast, // \ from v and >
    NorthWest, // \ from ^ and <
    NorthEast, // / from ^ and >
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Tile {
    Minecart(CartDirection),
    Track(TrackDirection),
    Corner(CornerConnection),
    Empty
}

impl Tile {
    fn graph(&self) -> char {
        use crate::Tile::*;
        use crate::CartDirection::*;
        use crate::TrackDirection::*;
        use crate::CornerConnection::*;

        match *self {
            Minecart(North) => '^',
            Minecart(East) => '>',
            Minecart(South) => 'v',
            Minecart(West) => '<',

            Track(EastWest) => '-',
            Track(NorthSouth) => '|',
            Track(Crossing) => '+',

            Corner(SouthWest) | Corner(NorthEast) => '/',
            Corner(SouthEast) | Corner(NorthWest) => '\\',

            Empty => ' '
        }
    }
}

use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct CartEntity {
    x: usize,
    y: usize,

    dir: CartDirection,
    next_intersect: i32, // 0 = left, 1 = straight, 2 = right, ...
    crashed: bool
}

impl CartEntity {
    fn tick(&mut self, world: &World) {
        use crate::CornerConnection::*;
        use crate::CartDirection::*;

        let (dx, dy): (isize, isize) = match self.dir {
            CartDirection::North => (0, -1),
            CartDirection::East => (1, 0),
            CartDirection::South => (0, 1),
            CartDirection::West => (-1, 0)
        };

        let nx = (self.x as isize + dx) as usize;
        let ny = (self.y as isize + dy) as usize;

        let next_tile = world.tracks[ny][nx];

        let new_dir = match next_tile {
            Tile::Corner(NorthEast) if self.dir == North => East,
            Tile::Corner(NorthEast) if self.dir == West => South,

            Tile::Corner(NorthWest) if self.dir == North => West,
            Tile::Corner(NorthWest) if self.dir == East => South,

            Tile::Corner(SouthWest) if self.dir == South => West,
            Tile::Corner(SouthWest) if self.dir == East => North,

            Tile::Corner(SouthEast) if self.dir == South => East,
            Tile::Corner(SouthEast) if self.dir == West => North,

            Tile::Track(TrackDirection::Crossing) => {
                let action = match self.next_intersect % 3 {
                    0 => self.dir.left(),
                    1 => self.dir,
                    2 => self.dir.right(),
                    _ => panic!("more cosmic rays!")
                };

                self.next_intersect += 1;

                action
            }

            _ => self.dir
        };

        self.x = nx;
        self.y = ny;
        self.dir = new_dir;
    }
}

use std::cell::RefCell;

struct World {
    tracks: Vec<Vec<Tile>>,
    carts: RefCell<Vec<CartEntity>>
}

fn parse_tiles(raw: &Vec<Vec<char>>) -> World {
    let mut world = Vec::new();
    let mut carts = Vec::new();

    for y in 0..raw.len() {
        let mut row  =Vec::new();

        fn is_east_west(c: char) -> bool {
            c == '-' || c == '+' || c == '<' || c == '>'
        }
        
        fn is_north_south(c: char) -> bool {
            c == '|' || c == '+' || c == '^' || c == 'v'
        }

        for x in 0..raw[y].len() {
            row.push(match raw[y][x] {
                ' ' => Tile::Empty,
                '^' | '>' | 'v' | '<' => {
                    let dir = match raw[y][x] {
                        '^' => CartDirection::North,
                        '>' => CartDirection::East,
                        'v' => CartDirection::South,
                        '<' => CartDirection::West,
                        _ => panic!("cosmic ray imapct")
                    };

                    carts.push(CartEntity {
                        x,
                        y,
                        dir,
                        next_intersect: 0,
                        crashed: false
                    });

                    if dir == CartDirection::North || dir == CartDirection::South {
                        Tile::Track(TrackDirection::NorthSouth)
                    } else {
                        Tile::Track(TrackDirection::EastWest)
                    }
                },
                
                '|' => Tile::Track(TrackDirection::NorthSouth),
                '-' => Tile::Track(TrackDirection::EastWest),
                '+' => Tile::Track(TrackDirection::Crossing),
                '/' => {
                    // Determine if NorthEast or SouthWest
                    if x > 0 && is_east_west(raw[y][x - 1])
                        && y > 0 && is_north_south(raw[y - 1][x]) {

                        Tile::Corner(CornerConnection::SouthWest)
                    } else if x < (raw[y].len() - 1) && is_east_west(raw[y][x + 1])
                        && y < (raw.len() - 1) && is_north_south(raw[y + 1][x]) {

                        Tile::Corner(CornerConnection::NorthEast)
                    } else {
                        panic!("invalid corner at x:{} y:{}!", x, y);
                    }
                },

                '\\' => {
                    // Determine if NorthWest or SouthEast
                    if x > 0 && is_east_west(raw[y][x - 1])
                        && y < (raw.len() - 1) && is_north_south(raw[y + 1][x]) {

                        Tile::Corner(CornerConnection::NorthWest)
                    } else if x < (raw[y].len() - 1) && is_east_west(raw[y][x + 1])
                        && y > 0 && is_north_south(raw[y - 1][x]) {

                        Tile::Corner(CornerConnection::SouthEast)
                    } else {
                        panic!("invalid corner at x:{} y:{}!", x, y);
                    }
                },

                _ => {
                    panic!("invalid tile at x:{} y:{}: {}", x, y, raw[y][x]);
                }
            })
        }

        world.push(row);
    }

    World {
        tracks: world,
        carts: RefCell::new(carts)
    }
}

impl World {
    fn tick(&mut self) -> Vec<(usize, usize)> {
        // First of all we have to sort by X,Y coords since even though the initial
        // list is in the correct order, the carts moving about causes their positions
        // to shift (duh)
        let mut collisions = Vec::new();

        let mut carts = self.carts.borrow_mut();

        carts.sort_by(|c1, c2| {
            if c1.y.cmp(&c2.y) == std::cmp::Ordering::Less {
                // Y cord lower => don't even look at X
                std::cmp::Ordering::Less
            } else {
                c1.x.cmp(&c2.x)
            }
        });

        let len = carts.len();

        for i in 0..len {
            if carts[i].crashed {
                continue;
            }

            carts[i].tick(&self);

            for j in 0..carts.len() {
                if j == i {
                    continue;
                }

                if carts[j].x == carts[i].x && carts[j].y == carts[i].y {
                    collisions.push((carts[j].x, carts[j].y));

                    carts[i].crashed = true;
                    carts[j].crashed = true;
                }
            }
        }

        collisions
    }

    fn print(&self) {
        let cartmap = self.carts.borrow().iter().map(|c| {
            ((c.x, c.y), Tile::Minecart(c.dir))
        }).collect::<HashMap<_, _>>();

        for y in 0..self.tracks.len() {
            for x in 0..self.tracks[y].len() {
                if cartmap.contains_key(&(x, y)) {
                    print!("{}", cartmap[&(x, y)].graph())
                } else {
                    print!("{}", self.tracks[y][x].graph())
                }
            }

            println!();
        }    
    }
}


fn main() {
    let raw = shared::input::read_stdin_lines().expect("could not lock stdin");
    let raw = raw.iter().map(|row| row.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut world = parse_tiles(&raw);

    'outer: for _generation in 0.. {
        // Check all collisions
        for (x, y) in world.tick() {
            let mut carts = world.carts.borrow_mut();

            println!("Carts crashed at x:{}, y:{}", x, y);

            // Throw out all broken carts
            carts.retain(|c| !c.crashed);

            if carts.len() == 1 {
                println!("Last cart at x:{}, y:{}", carts[0].x, carts[0].y);

                break 'outer;
            }
        }
    }
}
