use crate::map::MapComponent;

use pathfinding::prelude::absdiff;

/// Deprecated. An enum for expressing the
/// relationship of two X coordinates.
pub enum XPointRelation {
    LeftOfPoint,
    RightOfPoint,
    OnPointX
}

/// Deprecated. An enum for expressing the
/// relationship of two Y coordinates.
pub enum YPointRelation {
    AbovePoint,
    BelowPoint,
    OnPointY
}

/// Deprecated. An enum for expressing the
/// equality of two coordinates.
pub enum PointEquality {
    PointsEqual,
    PointsNotEqual
}

/// The foundational struct for representing a coordinate.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    /// Create a new point with the given X offset.
    pub fn offset_x(&self, offset: i32) -> Point {
        Point { x: self.x + offset, y: self.y }
    }

    /// Create a new point with the given Y offset.
    pub fn offset_y(&self, offset: i32) -> Point {
        Point { x: self.x, y: self.y + offset }
    }

    /// Create a new point with an offset of the given point.
    pub fn offset(&self, x: i32, y: i32) -> Point {
        Point { x: self.x + x, y: self.y + y }
    }

    /// Compare the x value of the current point against another
    pub fn compare_x(&self, point: Point) -> XPointRelation {
        use self::XPointRelation::*;

        if self.x > point.x {
            RightOfPoint
        } else if self.x < point.x {
            LeftOfPoint
        } else {
            OnPointX
        }
    }

    /// Compare the y value of the current point against another
    pub fn compare_y(&self, point: Point) -> YPointRelation {
        use self::YPointRelation::*;

        if self.y > point.y {
            BelowPoint
        } else if self.y < point.y {
            AbovePoint
        } else {
            OnPointY
        }
    }

    /// Compare both axes of the current point against another
    pub fn compare(&self, point: Point) -> PointEquality {
        use self::PointEquality::*;

        if self.x == point.x && self.y == point.y {
            PointsEqual
        } else {
            PointsNotEqual
        }
    }

    /// Get the absolute distance between the current point and another
    pub fn distance(&self, other: &Point) -> u32 {
        (absdiff(self.x, other.x) + absdiff(self.y, other.y)) as u32
    }

    /// Determine which points adjoining the current point can be used
    /// as a successor for the current point in an A* algorithm.
    pub fn successors(&self, map: &Box<dyn MapComponent>, goal: &Point) -> Vec<(Point, u32)> {
        let (x, y) = (self.x, self.y);
        let mut successors = vec![];
        for (idx, i) in [(0, 1), (0, -1), (1, 0), (-1, 0), (-1, -1), (1, 1), (1, -1), (-1, 1)].iter().enumerate() {
            let p = Point { x: x + i.0, y: y + i.1 };
            if (!map.is_blocked(x + i.0, y + i.1) && !map.is_occupied(x + i.0, y + i.1)) || p == *goal {
                if idx > 3 {
                    successors.push((p, 14));
                } else {
                    successors.push((p, 10));
                }
            }
        }
        successors
    }
}

/// A rectangle representing a boundary.
#[derive(Copy, Clone)]
pub struct Bound {
    pub min: Point,
    pub max: Point
}

impl Bound {
    /// Check whether the current bound contains a given point.
    pub fn contains(&self, point: Point) -> bool {
        if 
            point.x >= self.min.x &&
            point.x <= self.max.x &&
            point.y >= self.min.y &&
            point.y <= self.max.y
        {
            true
        } else {
            false
        }
    }
}

pub fn add_punctuation(mut s: String) -> String {
    if s.ends_with(".") || s.ends_with("!") || s.ends_with("?") {
        return s;
    }

    s.push('.');
    s
}
