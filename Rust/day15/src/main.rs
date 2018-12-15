use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty, Wall, Elf, Goblin
}

impl Tile {
    fn from_char(c: char) -> Option<Tile> {
        match c {
            '#' => Some(Tile::Wall),
            'E' => Some(Tile::Elf),
            'G' => Some(Tile::Goblin),
            _ => Some(Tile::Empty)
        }
    }

    fn is_passable(&self) -> bool {
        *self == Tile::Empty
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum UnitType {
    Elf, Goblin
}

#[derive(Debug, Clone, Copy)]
struct Unit {
    health: i32,
    unit_type: UnitType,
    pos: Coord,
    attack_power: i32
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Coord {
    x: usize,
    y: usize
}

const NEIGHBORS: &'static [(isize, isize)] = &[(0_isize, -1_isize), (-1, 0), (1, 0), (0, 1)];

impl Coord {
    fn dist(&self, other: &Coord) -> usize {
        let xd = (self.x as isize - other.x as isize).abs() as usize;
        let yd = (self.y as isize - other.y as isize).abs() as usize;

        xd + yd
    }

    fn neighbors(&self) -> impl Iterator<Item = Coord> + '_ {
        NEIGHBORS.iter().cloned().map(move |(x, y)| {
            Coord {
                x: (self.x as isize + x) as usize,
                y: (self.y as isize + y) as usize
            }
        })
    }
}

impl Ord for Coord {
    fn cmp(&self, other: &Coord) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.x.cmp(&other.x)
        }
    }
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Coord) -> Option<Ordering> {
        // Compare the wrong way around because BinaryHeap is a max-heap, but
        // we need a min-heap.
        Some(self.cmp(other))
    }
}

impl Unit {
    fn new(unit_type: UnitType, starting_pos: Coord) -> Self {
        Unit {
            health: 200,
            unit_type,
            pos: starting_pos,
            attack_power: 3
        }
    }

    fn is_enemy_of(&self, other: &Unit) -> bool {
        self.unit_type != other.unit_type
    }

    fn is_in_range_of(&self, other: &Unit) -> bool {
        self.pos.dist(&other.pos) == 1
    }

    fn hit(&mut self, damage: i32) {
        self.health -= damage;

        if self.health < 0 {
            self.health = 0;
        }
    }

    fn attack_power(&self) -> i32 {
        self.attack_power
    }

    fn hitpoints(&self) -> i32 {
        self.health
    }

    fn is_dead(&self) -> bool {
        self.hitpoints() == 0
    }
}

type Map = Vec<Vec<Tile>>;

#[derive(Clone)]
struct World {
    world: RefCell<Map>,
    units: Vec<RefCell<Unit>>
} 

fn read_units(map: &mut Map, units: &mut Vec<RefCell<Unit>>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let unit_type = if map[y][x] == Tile::Elf {
                UnitType::Elf
            } else if map[y][x] == Tile::Goblin {
                UnitType::Goblin
            } else {
                continue;
            };
            
            units.push(
                RefCell::new(
                    Unit::new(unit_type, Coord { x, y })));

            map[y][x] = Tile::Empty;
        }
    }
}

#[derive(Eq, PartialEq)]
enum TurnResult {
    NoQuarter,
    Victory
}

impl World {
    fn new(mut init: Map) -> Self {
        let mut units = Vec::new();

        // Parse out movable entities (elves and goblins) into a separate structure.
        // The order of the vec is already according to specifications
        read_units(&mut init, &mut units);

        World {
            world: RefCell::new(init),
            units: units
        }
    }

    fn round(&mut self) -> TurnResult {
        self.units.sort_unstable_by_key(|u| u.borrow().pos);

        for i in 0..self.units.len() {
            if !self.units[i].borrow().is_dead() {
                if self.turn(i, &self.units) == TurnResult::Victory {
                    return TurnResult::Victory;
                }
            }
        }

        TurnResult::NoQuarter
    }

    fn turn(&self, i: usize, units: &[RefCell<Unit>]) -> TurnResult {
        let targets = units
            .iter()
            .enumerate()
            .filter(|(j, u)| i != *j && !u.borrow().is_dead() && u.borrow().is_enemy_of(&units[i].borrow())).collect::<Vec<_>>();

        let unit_positions = units
            .iter()
            .enumerate()
            .filter(|(_j, u)| !u.borrow().is_dead())
            .map(|(_j, u)| u.borrow().pos)
            .collect::<HashSet<_>>();

        if targets.len() == 0 {
            return TurnResult::Victory;
        }

        // All reachable targets
        let mut target_squares = Vec::new();
        let mut adjacent = Vec::new();

        let world = self.world.borrow();

        for (_j, target) in targets.iter() {
            let unit = units[i].borrow();

            if unit.is_in_range_of(&target.borrow()) {
                adjacent.push(*target);
            }

            for neighbor in target.borrow().pos.neighbors() {
                if world[neighbor.y][neighbor.x].is_passable() && !unit_positions.contains(&neighbor) {
                    target_squares.push(neighbor);
                }
            }
        }

        if adjacent.len() == 0 {
            let mut unit = units[i].borrow_mut();

            // Have to move somewhere, figure out next move
            let mut next_moves = Vec::new();

            for tsquare in target_squares {
                if let Some(path) = self.bfs(&unit_positions, unit.pos, tsquare) {
                    next_moves.push((path.len(), path[1]));
                }
            }

            if let Some((_, next_move)) = next_moves.iter().min_by_key(|(l, _)| l) {
                unit.pos = *next_move;

                for (_j, target) in targets.iter() {
                    if unit.is_in_range_of(&target.borrow()) {
                        adjacent.push(*target);
                    }
                }
            }
        }

        if adjacent.len() != 0 {
            // If we have reached an enemy before, or using the last move, attack
            let unit = units[i].borrow();
            let weakest = adjacent.iter().min_by_key(|u| u.borrow().hitpoints()).unwrap();

            weakest.borrow_mut().hit(unit.attack_power());
        }

        TurnResult::NoQuarter
    }

    fn print(&self) {
        let world = self.world.borrow();
        let mut units_per_row = HashMap::new();

        for unit in &self.units {
            let p = units_per_row.entry(unit.borrow().pos.y).or_insert(Vec::new());

            p.push(unit.borrow());
            p.sort_by_key(|u| u.pos.x);
        }

        for y in 0..world.len() {
            for x in 0..world[y].len() {
                if let Some(u) = self.units.iter().find(|u| u.borrow().pos == Coord { x, y }) {
                    let u = u.borrow();
                    let c = if u.unit_type == UnitType::Elf {
                        'E'
                    } else {
                        'G'
                    };

                    if u.is_dead() {
                        print!("\x1b[1;31m");
                    } else {
                        print!("\x1b[1;32m");
                    }

                    print!("{}", if u.is_dead() {
                        c.to_ascii_lowercase()
                    } else {
                        c
                    });

                    print!("\x1b[0m");
                } else {
                    print!("{}", match world[y][x] {
                        Tile::Empty => ' ',
                        Tile::Wall => '#',
                        _ => '?'
                    });
                }
            }

            if units_per_row.contains_key(&y) {
                print!("   ");

                for unit in &units_per_row[&y] {
                    print!("{}({}), ", if unit.unit_type == UnitType::Elf { 'E' } else { 'G'}, unit.hitpoints());
                }
            }

            println!();
        }
    }

    fn bfs(&self,
        unit_positions: &HashSet<Coord>,
        source_pos: Coord,
        target_pos: Coord) -> Option<Vec<Coord>>
    {
        use std::collections::{HashSet, VecDeque};

        let world = self.world.borrow();

        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();

        queue.push_back(vec![source_pos]);
        seen.insert(source_pos);

        while let Some(path) = queue.pop_front() {
            let c =  path[path.len() - 1];

            if c == target_pos {
                return Some(path);
            }

            for neighbor in c.neighbors() {
                if world[neighbor.y][neighbor.x].is_passable() && !unit_positions.contains(&neighbor) && !seen.contains(&neighbor) {
                    let mut npath = path.clone();
                    npath.push(neighbor);

                    queue.push_back(npath);
                    seen.insert(neighbor);
                }
            }
        }

        None
    }

    fn hitpoints(&self) -> (i32, i32) {
        self.units.iter().fold((0, 0), |(s_elf, s_gob), u|
            if u.borrow().unit_type == UnitType::Elf {
                (s_elf + u.borrow().hitpoints(), s_gob)
            } else {
                (s_elf, s_gob + u.borrow().hitpoints())
            })
    }

    fn casualites(&self) -> (i32, i32) {
        self.units
            .iter()
            .filter(|u| u.borrow().is_dead())
            .fold((0, 0), |(s_elf, s_gob), u|
                if u.borrow().unit_type == UnitType::Elf {
                    (s_elf + 1, s_gob)
                } else {
                    (s_elf, s_gob + 1)
                })
    }

    fn toggle(&mut self, pelf: i32, pgob: i32) {
        for unit in &self.units {
            if unit.borrow().unit_type == UnitType::Elf {
                unit.borrow_mut().attack_power = pelf;
            } else {
                unit.borrow_mut().attack_power = pgob;
            }
        }
    }
}

fn run_battle(world: &mut World, elf_power: i32) -> (i32, (i32, i32), (i32, i32)) {
    let mut i = 0;

    world.toggle(elf_power, 3);

    loop {
        world.print();
        
        if world.round() == TurnResult::Victory {
            println!("Winner, Winner, Chicken Dinner after {} turns", i);

            break;
        }

        i += 1;

        if i == 1 {
            //break;
        }
    }
    
    world.print();

    let (score_elves, score_goblins) = world.hitpoints();
    let (celf, cgob) = world.casualites();

    (i, (celf, score_elves), (cgob, score_goblins))
}

fn main() {
    let field = shared::input::read_stdin_lines().expect("could not lock stdin");
    let field = field.iter().map(|row|
            row.chars().filter_map(|c| Tile::from_char(c)).collect::<Vec<_>>()
        ).collect::<Vec<_>>();

    let state = World::new(field);

    for p in 4.. {
        let mut scen = state.clone();

        let (i, (loss_elves, score_elves), (loss_gobs, score_goblins)) = run_battle(&mut scen, p);

        println!("Outcome: Elves: {}, Goblins: {}", score_elves, score_goblins);
        println!("Total outcome elves: {} ({} lost)", score_elves * i, loss_elves);
        println!("Total outcome goblins: {} ({} lost)", score_goblins * i, loss_gobs);

        if loss_elves == 0 {
            break;
        }
    }
}