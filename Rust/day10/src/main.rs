use std::str::FromStr;
use std::ops::Add;

use regex::Regex;
use lazy_static::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vec2<T>(T, T);

impl<T> Add<Vec2<T>> for Vec2<T> where T: Add<T, Output = T> {
    type Output = Vec2<T>;

    fn add(self, other: Vec2<T>) -> Self::Output {
        Vec2(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(Debug)]
struct Point {
    pos: Vec2<i32>,
    vel: Vec2<i32>
}

impl Point {
    fn tick(&self) -> Self {
        Point {
            pos: self.pos + self.vel,
            vel: self.vel
        }
    }

    fn untick(&self) -> Self {
        Point {
            pos: self.pos + Vec2(-self.vel.0, -self.vel.1),
            vel: self.vel
        }
    }
}

fn calculate_bounding_box(points: &[Point]) -> (Vec2<i32>, Vec2<i32>) {
    let mut min = Vec2(i32::max_value(), i32::max_value());
    let mut max = Vec2(i32::min_value(), i32::min_value());

    for Point { pos, .. } in points {
        if pos.0 > max.0 { max.0 = pos.0; }
        if pos.1 > max.1 { max.1 = pos.1; }

        if pos.0 < min.0 { min.0 = pos.0; }
        if pos.1 < min.1 { min.1 = pos.1; }
    }

    (min, max)
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PATTERN: Regex = Regex::new(r"(?x)
                ^
                position=<\s*(-?(?:\d+)),\s*(-?(?:\d+))>
                \s+
                velocity=<\s*(-?(?:\d+)),\s*(-?(?:\d+))>
                $").unwrap();
        }

        let matches = PATTERN.captures(s).ok_or(())?;

        Ok(Point {
            pos: Vec2(
                matches.get(1).ok_or(())?.as_str().parse().or(Err(()))?,
                matches.get(2).ok_or(())?.as_str().parse().or(Err(()))?
            ),
            
            vel: Vec2(
                matches.get(3).ok_or(())?.as_str().parse().or(Err(()))?,
                matches.get(4).ok_or(())?.as_str().parse().or(Err(()))?
            ),
        })
    }
}

fn draw_sky(points: &[Point]) {
    use std::collections::HashSet;

    let (min, max) = calculate_bounding_box(points);
    let sky = points.iter().fold(HashSet::new(), |mut acc, pt| {
        acc.insert(pt.pos);
        acc
    });
    
    for y in min.1 ..= max.1 {
        for x in min.0 ..= max.0 {
            print!("{}", if sky.contains(&Vec2(x, y)) {
                '#'
            } else {
                '.'
            });
        }

        println!();
    }
}

fn main() {
    let input = shared::input::read_stdin_lines().expect("could not lock stdin");
    let mut input: Vec<Point> = input.iter().filter_map(|line| line.parse().ok()).collect();

    let (mut min, mut max) = calculate_bounding_box(&input);
    
    for t in 0.. {
        input.iter_mut().for_each(|p| *p = p.tick());

        let (new_min, new_max) = calculate_bounding_box(&input);

        // Presumably, once all points are aligned, the bounding box is minimal, so any increase in
        // the BB is taken as the message being just past its most coherent form
        if new_min.0 < min.0 || new_min.1 < min.1
            || new_max.0 > max.0 || new_max.1 > max.1 {

            input.iter_mut().for_each(|p| *p = p.untick());

            draw_sky(&input);

            println!("Arrived at message after {} steps", t);

            break;
        } else {
            min = new_min;
            max = new_max;            
        }
    }
}
