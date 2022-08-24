use std::ops::{Add, Div, Mul, Sub};

pub type Coord = i32;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: Coord,
    pub y: Coord,
}

impl Point {
    pub fn new(x: Coord, y: Coord) -> Self {
        Self { x, y }
    }

    pub fn from_usize(x: usize, y: usize) -> Self {
        Self {
            x: x as Coord,
            y: y as Coord,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for Point {
    type Output = Point;

    fn mul(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Div for Point {
    type Output = Point;

    fn div(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
