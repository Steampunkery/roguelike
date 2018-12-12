extern crate tcod;
extern crate rand;

use self::tcod::input::KeyCode;
use game::Game;
use util::{Bound, Point};

use util::Contains::*;
use util::PointEquality::*;
use util::XPointRelation::*;
use util::YPointRelation::*;

use self::rand::Rng;

pub trait MovementComponent {
    fn update(&self, point: Point) -> Point;
}

pub struct RandomMovementComponent {
    window_bounds: Bound
}

pub struct TcodUserMovementComponent {
    window_bounds: Bound
}

pub struct AggroMovementComponent {
    window_bounds: Bound
}

impl AggroMovementComponent {
    pub fn new(bound: Bound) -> AggroMovementComponent {
        AggroMovementComponent { window_bounds: bound }
    }
}

impl MovementComponent for AggroMovementComponent {
    fn update(&self, point: Point) -> Point {
        let char_point = Game::get_character_point();
        let mut offset = Point { x: 0, y: 0 };
        match point.compare_x(char_point) {
            RightOfPoint => offset = offset.offset_x(-1),
            LeftOfPoint => offset = offset.offset_x(1),
            OnPointX => {}
        }

        match point.compare_y(char_point) {
            BelowPoint => offset = offset.offset_y(-1),
            AbovePoint => offset = offset.offset_y(1),
            OnPointY => {}
        }

        match point.offset(offset).compare(char_point) {
            PointsEqual => { return point; }
            PointsNotEqual => {
                match self.window_bounds.contains(point.offset(offset)) {
                    DoesContain => { return point.offset(offset); }
                    DoesNotContain => { return point; }
                }
            }
        }
    }
}

impl TcodUserMovementComponent {
    pub fn new(bound: Bound) -> TcodUserMovementComponent {
        TcodUserMovementComponent { window_bounds: bound }
    }
}

impl MovementComponent for TcodUserMovementComponent {
    fn update(&self, point: Point) -> Point {
        let mut offset = Point { x: point.x, y: point.y };
        offset = match Game::get_last_keypress() {
            Some(keypress) => {
                match keypress.code {
                    KeyCode::Up => {
                        offset.offset_y(-1)
                    }
                    KeyCode::Down => {
                        offset.offset_y(1)
                    }
                    KeyCode::Left => {
                        offset.offset_x(-1)
                    }
                    KeyCode::Right => {
                        offset.offset_x(1)
                    }
                    _ => { offset }
                }
            }
            None => { offset }
        };

        match self.window_bounds.contains(offset) {
            DoesContain => { offset }
            DoesNotContain => { point }
        }
    }
}

impl RandomMovementComponent {
    pub fn new(bound: Bound) -> RandomMovementComponent {
        RandomMovementComponent { window_bounds: bound }
    }
}

impl MovementComponent for RandomMovementComponent {

    fn update(&self, point: Point) -> Point {
        let mut offset = Point { x: point.x, y: point.y };
        let offset_x = rand::thread_rng().gen_range(0, 3i32) - 1;
        match self.window_bounds.contains(offset.offset_x(offset_x)) {
            DoesContain => offset = offset.offset_x(offset_x),
            DoesNotContain => { return point; }
        }

        let offset_y = rand::thread_rng().gen_range(0, 3i32) - 1;
        match self.window_bounds.contains(offset.offset_y(offset_y)) {
            DoesContain => offset = offset.offset_y(offset_y),
            DoesNotContain => { return point; }
        }

        offset
    }
}
