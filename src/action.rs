use crate::actor::Actor;
use crate::map::MapComponent;

pub trait Action {
    fn perform(&self, map_component: &mut Box<dyn MapComponent>) -> bool;
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
}

pub struct WalkAction<'b> {
    pub direction: Direction,
    pub actor: &'b mut Actor,
}

impl<'b> WalkAction<'b> {
    pub fn new(direction: Direction, actor: &'b mut Actor) -> WalkAction { WalkAction { direction, actor } }
}

impl<'b> Action for WalkAction<'b> {
    fn perform(&self, map_component: &mut Box<dyn MapComponent>) -> bool {
        use crate::action::Direction::*;

        let position = self.actor.get_position().clone();
        let new_position = match self.direction {
            N => position.offset_y(-1),
            S => position.offset_y(1),
            W => position.offset_x(-1),
            E => position.offset_x(1),
            NW => position.offset(-1, -1),
            NE => position.offset(1, -1),
            SW => position.offset(-1, 1),
            SE => position.offset(1, 1),
        };


        if !map_component.is_blocked(position.x, position.y) {
            self.actor.set_position(new_position);
            return true;
        }

        false
    }
}

pub struct WaitAction;

impl Action for WaitAction {
    fn perform(&self, _m: &mut Box<dyn MapComponent>) -> bool {
        true
    }
}