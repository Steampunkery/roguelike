use crate::actor::Actor;
use crate::level::Level;
use crate::util::Point;
use crate::action::Direction::NoDir;

pub trait Action {
    fn perform(&self, level: &mut Level) -> bool;
}

pub enum Direction {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW,
    NoDir,
}

pub struct WalkAction {
    pub direction: Direction,
    pub target: usize,
    pub offset: Option<Point>,
    pub point: Option<Point>,
}

impl WalkAction {
    pub fn new(direction: Direction, target: usize) -> WalkAction { WalkAction { direction, target, offset: None, point: None } }
    pub fn from_offset(offset: Point, target: usize) -> WalkAction { WalkAction { direction: NoDir, target, offset: Some(offset), point: None } }
    pub fn from_point(point: Point, target: usize) -> WalkAction { WalkAction { direction: NoDir, target, offset: None, point: Some(point) } }
}

impl Action for WalkAction {
    fn perform(&self, level: &mut Level) -> bool {
        use crate::action::Direction::*;
        let actor = &mut level.entities[self.target];

        let position = actor.as_ref().unwrap().get_position();

        let new_position = if let Some(offset) = self.offset {
            position.offset(offset.x, offset.y)
        } else if let Some(point) = self.point {
            point
        } else {
            match self.direction {
                N => position.offset_y(-1),
                S => position.offset_y(1),
                W => position.offset_x(-1),
                E => position.offset_x(1),
                NW => position.offset(-1, -1),
                NE => position.offset(1, -1),
                SW => position.offset(-1, 1),
                SE => position.offset(1, 1),
                NoDir => position,
            }
        };


        if !level.map_component.is_blocked(new_position.x, new_position.y) {
            actor.as_mut().unwrap().set_position(new_position);
            return true;
        }

        false
    }
}

pub struct WaitAction {
    pub target: usize
}

impl Action for WaitAction {
    fn perform(&self, l: &mut Level) -> bool {
        let me = &mut l.entities[self.target];

        let position = me.as_mut().unwrap().get_position();
        me.as_mut().unwrap().set_position(position);
        true
    }
}