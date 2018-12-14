use crate::map::MapComponent;

use pathfinding::prelude::absdiff;

pub enum XPointRelation {
    LeftOfPoint,
    RightOfPoint,
    OnPointX
}

pub enum YPointRelation {
    AbovePoint,
    BelowPoint,
    OnPointY
}

pub enum PointEquality {
    PointsEqual,
    PointsNotEqual
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn offset_x(&self, offset: i32) -> Point {
        Point { x: self.x + offset, y: self.y }
    }

    pub fn offset_y(&self, offset: i32) -> Point {
        Point { x: self.x, y: self.y + offset }
    }

    pub fn offset(&self, offset: Point) -> Point {
        Point { x: self.x + offset.x, y: self.y + offset.y }
    }

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

    pub fn compare(&self, point: Point) -> PointEquality {
        use self::PointEquality::*;

        if self.x == point.x && self.y == point.y {
            PointsEqual
        } else {
            PointsNotEqual
        }
    }

    pub fn distance(&self, other: &Point) -> u32 {
        (absdiff(self.x, other.x) + absdiff(self.y, other.y)) as u32
    }

    pub fn successors(&self, map: &Box<MapComponent>) -> Vec<(Point, u32)> {
        let (x, y) = (self.x, self.y);
        let mut successors = vec![];
        for i in &[-1, 1] {
            if !map.is_blocked(x, y + i) {
                successors.push((Point { x, y: y + i }, 1));
            }

            if !map.is_blocked(x + i, y) {
                successors.push((Point { x: x + i, y }, 1));
            }
        }
        successors
    }
}

pub enum Contains {
    DoesContain,
    DoesNotContain
}

#[derive(Copy, Clone)]
pub struct Bound {
    pub min: Point,
    pub max: Point
}

impl Bound {
    pub fn contains(&self, point: Point) -> Contains {
        use self::Contains::*;

        if 
            point.x >= self.min.x &&
            point.x <= self.max.x &&
            point.y >= self.min.y &&
            point.y <= self.max.y
        {
            DoesContain
        } else {
            DoesNotContain
        }
    }
}
