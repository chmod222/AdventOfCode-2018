#[derive(Debug)]
enum Direction {
    North, East, South, West
}

impl Direction {
    fn step(&self, (y, x): (isize, isize)) -> (isize, isize) {
        match self {
            Direction::North => (y - 1, x),
            Direction::East => (y, x + 1),
            Direction::South => (y + 1, x),
            Direction::West => (y, x - 1)
        }
    }
}

type Path = Vec<DoorEx>;

#[derive(Debug)]
enum DoorEx {
    Door(Direction),
    Branch(Vec<Path>)
}

use std::collections::VecDeque;

fn parse_path(inp: &[u8]) -> Option<Path> {
    let mut stack: VecDeque<VecDeque<Vec<DoorEx>>> = VecDeque::new();

    stack.push_back({
        let mut tmp = VecDeque::new();
        tmp.push_back(Vec::new());

        tmp
    });

    for i in 0..inp.len() {
        match inp[i] {
            b'N' => stack.back_mut()?.back_mut()?.push(DoorEx::Door(Direction::North)),
            b'E' => stack.back_mut()?.back_mut()?.push(DoorEx::Door(Direction::East)),
            b'S' => stack.back_mut()?.back_mut()?.push(DoorEx::Door(Direction::South)),
            b'W' => stack.back_mut()?.back_mut()?.push(DoorEx::Door(Direction::West)),

            b'(' => {
                stack.push_back({
                    let mut tmp = VecDeque::new();
                    tmp.push_back(Vec::new());

                    tmp
                });
            },

            b'|' => {
                stack.back_mut()?.push_back(Vec::new());
            },
            
            b')' => {
                if let Some(mut fr) = stack.pop_back() {
                    stack.back_mut()?.back_mut()?.push(DoorEx::Branch(fr.drain(..).collect()));
                }
            }

            _ => (),
        }
    }

    let mut inner = stack.pop_back()?;
    
    Some(inner.pop_back()?)
}

fn parse_doorex(inp: &[u8]) -> Option<Path> {
    if inp[0] != b'^' {
        return None;
    }

    parse_path(&inp[1..inp.len() - 1])
}

use std::collections::{HashMap};

fn count_steps(
    mut pos: (isize, isize),
    mut steps_so_far: usize,
    path: &Path,
    rooms: &mut HashMap<(isize, isize), usize>)
{
    for p in path {
        match p {
            DoorEx::Door(d) => {
                pos = d.step(pos);
                steps_so_far += 1;

                rooms.entry(pos).or_insert(steps_so_far);
            },

            DoorEx::Branch(choices) => {
                for choice in choices {
                    count_steps(pos, steps_so_far, &choice, rooms);
                }
            }
        }
    }
}

fn main() {
    const INPUT: &[u8] = include_bytes!("../input");

    let mut rooms: HashMap<(isize, isize), usize> = HashMap::new();
    let parsed_input = parse_doorex(INPUT).unwrap();
    
    count_steps((0, 0), 0, &parsed_input, &mut rooms);

    let mut rooms = rooms.iter().collect::<Vec<_>>();
    
    rooms.sort_by_key(|(_k, v)| *v);

    println!("Part 1: {}", rooms[rooms.len() - 1].1);
    println!("Part 2: {}", rooms.iter().filter(|(_k, v)| **v >= 1000).count());
}
