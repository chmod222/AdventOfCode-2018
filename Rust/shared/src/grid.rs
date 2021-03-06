use std::ops::{Add, Mul, Sub};
use std::cmp::Ordering;

pub trait Numeric:
    Copy + PartialEq + Ord
        + Add<Self, Output=Self>
        + Mul<Self, Output=Self>
        + Sub<Self, Output=Self>
{
}

impl Numeric for i8 { }
impl Numeric for i16 { }
impl Numeric for i32 { }
impl Numeric for isize { }

impl Numeric for u8 { }
impl Numeric for u16 { }
impl Numeric for u32 { }
impl Numeric for usize { }

pub trait Coordinate: Copy + PartialEq + Ord {
    type Component: Numeric + std::iter::Step;
    type NeighborIterator: Iterator<Item = Self>;

    fn new(y: Self::Component, x: Self::Component) -> Self;

    fn distance(&self, other: &Self) -> Self::Component;
    fn neighbors(&self) -> Self::NeighborIterator;

    fn x(&self) -> Self::Component;
    fn y(&self) -> Self::Component;

    fn numeric_limits() -> (Self::Component, Self::Component);
}

#[derive(Copy, Clone)]
pub enum Color {
    Black = 0, Red, Green, Yellow, Blue, Magenta, Cyan, White
}

pub enum Attribute {
    None, Bold
}

pub type FormatSpec = (Color, Attribute);

pub enum TileColor {
    NoColor,
    Foreground(FormatSpec),
    Background(FormatSpec),
    Both(FormatSpec, FormatSpec)
}

fn csi(base: u8, col: Color, attr: Attribute) {
    print!("\x1b[{}", base + (col as u8));
        
    match attr {
        Attribute::None => (),
        Attribute::Bold => print!(";1")
    }

    print!("m");
}

fn apply(fg: Option<FormatSpec>, bg: Option<FormatSpec>) {
    if let Some(fg) = fg {
        csi(30, fg.0, fg.1);
    }

    if let Some(bg) = bg {
        csi(40, bg.0, bg.1);
    }
}

fn reset() {
    print!("\x1b[0m");
}

pub trait GridTile {
    fn to_char(&self) -> char;
    fn color(&self) -> TileColor;
}

pub trait Grid {
    type Coord: Coordinate;
    type Tile: GridTile;

    fn bounds(&self) -> (Self::Coord, Self::Coord);
    fn tile_at(&self, coord: &Self::Coord) -> &Self::Tile;
    
    fn draw(&self) {
        let (lower, upper) = self.bounds();

        for y in lower.y() .. upper.y() {
            for x in lower.x() .. upper.x() {
                let c = <Self::Coord as Coordinate>::new(y, x);
                let t = self.tile_at(&c);

                let (fg, bg) = match t.color() {
                    TileColor::NoColor => (None, None),
                    TileColor::Foreground(spec) => (Some(spec), None),
                    TileColor::Background(spec) => (None, Some(spec)),
                    TileColor::Both(f_spec, b_spec) => (Some(f_spec), Some(b_spec))
                };

                let has_format = fg.is_some() || bg.is_some();

                apply(fg, bg);

                print!("{}", t.to_char());

                if has_format {
                    reset();
                }
            }

            println!();
        }
    }
}

pub enum MovementCost {
    Impassable,
    Passable(isize)
}

pub trait NavigatableGrid: Grid {
    fn movement_cost(&self, c1: &Self::Coord, c2: &Self::Coord) -> MovementCost;
}

pub trait NavigatableTile: GridTile {
    fn movement_cost(&self, other: &Self) -> MovementCost;
}

impl<T> NavigatableGrid for T where T: Grid, T::Tile: NavigatableTile {
    fn movement_cost(&self, c1: &Self::Coord, c2: &Self::Coord) -> MovementCost {
        let t1 = self.tile_at(c1);
        let t2 = self.tile_at(c2);

        t1.movement_cost(t2)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Coord(pub isize, pub isize);

pub struct CoordNeighbors {
    base: Coord,
    n: usize
}

impl Iterator for CoordNeighbors {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.n {
            0 => Some(Coord(self.base.0 - 1, self.base.1)),
            1 => Some(Coord(self.base.0, self.base.1 - 1)),
            2 => Some(Coord(self.base.0, self.base.1 + 1)),
            3 => Some(Coord(self.base.0 + 1, self.base.1)),
            _ => None
        };

        self.n += 1;
        next
    }
}

impl Coordinate for Coord {
    type Component = isize;
    type NeighborIterator = CoordNeighbors;

    fn new(y: Self::Component, x: Self::Component) -> Self {
        Coord(y, x)
    }

    fn distance(&self, other: &Self) -> Self::Component {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs())
    }

    fn neighbors(&self) -> CoordNeighbors {
        CoordNeighbors {
            base: *self,
            n: 0
        }
    }

    fn x(&self) -> Self::Component { self.1 }
    fn y(&self) -> Self::Component { self.0 }

    fn numeric_limits() -> (Self::Component, Self::Component) {
        (isize::min_value(), isize::max_value())
    }

}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.0.cmp(&other.0) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            _ => self.1.cmp(&other.1)
        }
    }
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Grid for Vec<Vec<T>> where T: GridTile {
    type Coord = Coord;
    type Tile = T;

    fn bounds(&self) -> (Self::Coord, Self::Coord) {
        if self.len() > 0 {
            (Coord(0, 0), Coord(self.len() as _, self[0].len() as _))
        } else {
            (Coord(0, 0), Coord(0, 0))
        }
    }

    fn tile_at(&self, coord: &Self::Coord) -> &Self::Tile {
        &self[coord.0 as usize][coord.1 as usize]
    }
}