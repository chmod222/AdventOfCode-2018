use std::collections::HashMap;
use std::ops::{Add, Sub};

type Node = u32;
type FieldSizes = HashMap<Node, usize>;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Coord(isize, isize);

impl Coord {
    fn manhattan(&self, other: &Coord) -> isize {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

impl Add<isize> for Coord {
    type Output = Coord;

    fn add(self, offset: isize) -> Self::Output {
        Coord(self.0 + offset, self.1 + offset)
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Self::Output {
        Coord(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub<isize> for Coord {
    type Output = Coord;

    fn sub(self, offset: isize) -> Self::Output {
        Coord(self.0 - offset, self.1 - offset)
    }
}

impl Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, other: Coord) -> Self::Output {
        Coord(self.0 - other.0, self.1 - other.1)
    }
}

#[derive(Debug)]
struct Field(HashMap<Coord, Option<Node>>);

impl Field {
    fn new() -> Field {
        Field(HashMap::new())
    }

    fn insert_node(&mut self, node: Node, pos: Coord) {
        self.0.insert(pos, Some(node));
    }

    fn bounding_box(&self, padding: isize) -> (Coord, Coord) {
        let mut min = Coord(isize::max_value(), isize::max_value());
        let mut max = Coord(isize::min_value(), isize::min_value());

        for (Coord(x, y), _) in self.0.iter() {
            if *x > max.0 { max.0 = *x; }
            if *x < min.0 { min.0 = *x; }
            if *y > max.1 { max.1 = *y; }
            if *y < min.1 { min.1 = *y; }
        }

        (min - padding, max + padding)
    }

    fn distances(&self, coord: Coord) -> impl Iterator<Item = (Node, isize)> + '_ {
        self.0.iter().filter_map(move |(pos, &node)| {
            Some((node?, coord.manhattan(pos)))
        })
    }

    fn summed_distance(&self, pos: Coord) -> isize {
        self.distances(pos).map(|(_, d)| d).sum()
    }

    fn closest(&self, coord: Coord) -> Option<(Node, isize)> {
        let mut distances = self.distances(coord).collect::<Vec<_>>();

        distances.sort_by(|(_, a), (_, b)| a.cmp(b));
        
        let mut distances = distances.iter();

        let (first_node, first_dist) = distances.next()?;
        
        if let Some((_, second_dist)) = distances.next() {
            if second_dist == first_dist {
                return None;
            }
        }

        Some((*first_node, *first_dist))
    }

    fn calculate_voronoi(&self, from: Coord, to: Coord) -> FieldSizes {
        let Coord(min_x, min_y) = from;
        let Coord(max_x, max_y) = to;

        let mut field_sizes = FieldSizes::new();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if let Some((closest, _)) = self.closest(Coord(x, y)) {
                    *field_sizes.entry(closest).or_insert(0_usize) += 1;
                }
            }
        }

        field_sizes
    }
}

fn main() {
    let input = shared::input::read_stdin_lines().expect("could not lock stdin");
    let input = (0..).zip(input.iter().filter_map(|coords| {
        let mut parts = coords.split(", ");

        Some(Coord(
            parts.next()?.parse().ok()?,
            parts.next()?.parse().ok()?
        ))
    }));

    let mut field = Field::new();
    
    for (node, pos) in input {
        field.insert_node(node, pos);
    }

    let (min, max) = field.bounding_box(0);

    let field_sizes = field.calculate_voronoi(min, max);
    let mut field_sizes_growing = field.calculate_voronoi(min - 100, max + 100);
        
    field_sizes_growing.retain(|k, v| field_sizes[&k] == *v);

    let smallest = field_sizes_growing.iter().max_by(|(_k1, v1), (_k2, v2)| v1.cmp(v2)).unwrap();

    println!("Part 1: {}", smallest.1);

    const DISTANCE_LIMIT: isize = 10_000;
    let mut near_field = 0;

    for y in min.1..=max.1 {
        for x in min.0..=max.0 {
            if field.summed_distance(Coord(x, y)) < DISTANCE_LIMIT {
                near_field += 1;
            }
        }
    }

    println!("Part 2: {}", near_field);
}
