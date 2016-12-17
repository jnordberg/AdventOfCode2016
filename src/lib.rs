use std::fmt;
use std::ops::Add;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Point {
    pub fn manhattan_distance(&self, to: Point) -> i32 {
        (self.x - to.x).abs() + (self.y - to.y).abs()
    }
}


#[derive(Debug)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}
