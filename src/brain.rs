use crate::level::Level;

use crate::ai::{DEBUG_AI, find_astar_path};

use crate::util::{Point, Bound};

use crate::actor::{Actor, Entity};

use crate::map::MapComponent;

use tcod::colors::Color;
use rand::Rng;

use crate::action::{WalkAction, WaitAction, Action};

/// A trait for defining a method of movement
/// that may be applied to any living monster.
pub trait BrainComponent {
    /// The method that decides the next move according to the implementation.
    fn get_action(&mut self, entity: &mut Entity, level: &mut Level) -> Option<Box<dyn Action>>;
}

/// A movement component that uses A* to find the
/// fastest path to the player.
pub struct AggroBrainComponent {
    path: Vec<Point>,
}

pub struct RandomBrainComponent {
    bounds: Bound,
}

pub struct NoBrainComponent;

pub struct PlayerBrainComponent;

impl AggroBrainComponent {
    /// Convenience method for creating `AggroMovementComponent`s.
    pub fn new() -> AggroBrainComponent {
        AggroBrainComponent { path: vec![] }
    }

    fn show_ai(&mut self, map: &mut Box<dyn MapComponent>) {
        if DEBUG_AI {
            if !self.path.is_empty() {
                for point in &self.path {
                    map.get_map_mut()[point.x as usize][point.y as usize].color_override = Some(Color { r: 255, g: 0, b: 0 });
                }
            }
        }
    }
}

impl BrainComponent for AggroBrainComponent {
    fn get_action(&mut self, entity: &mut Entity, level: &mut Level) -> Option<Box<dyn Action>> {
        let player_pos = level.entities[0].as_ref().unwrap().get_position();
        let last_player_pos = level.entities[0].as_ref().unwrap().get_last_position();
        let target = level.current_actor;

        if player_pos != last_player_pos || self.path.is_empty() {
            let path_opt = find_astar_path(&level.map_component, entity.get_position(), player_pos);

            if let Some(path) = path_opt {
                self.path = path;

                // 0th element is the current position
                self.path.remove(0);

                return if !self.path.is_empty() {
                    self.show_ai(&mut level.map_component);
                    Some(box WalkAction::from_point(self.path.remove(0), target))
                } else {
                    Some(box WaitAction { target })
                }
            }
        } else if player_pos == last_player_pos && !self.path.is_empty() {
            self.show_ai(&mut level.map_component);
            return Some(box WalkAction::from_point(self.path.remove(0), target));
        }

        Some(box WaitAction { target })
    }
}

impl PlayerBrainComponent {
    /// Convenience method for creating `TcodUserMovementComponents`.
    pub fn new() -> PlayerBrainComponent {
        PlayerBrainComponent
    }
}

impl BrainComponent for PlayerBrainComponent {
    fn get_action(&mut self, _e: &mut Entity, level: &mut Level) -> Option<Box<dyn Action>> {
        use crate::action::Direction::*;
        use tcod::input::KeyCode::*;

        let target = level.current_actor;

        // TODO: Consider replacing this match with a method named something like "match_action"
        match level.input {
            Some(keypress) => {
                match (keypress.code, keypress.printable) {
                    (NumPad8, _) | (Up, _) => Some(box WalkAction::new(N, target)),
                    (NumPad2, _) | (Down, _) => Some(box WalkAction::new(S, target)),
                    (NumPad4, _) | (Left, _) => Some(box WalkAction::new(W, target)),
                    (NumPad6, _) | (Right, _) => Some(box WalkAction::new(E,target)),
                    (NumPad7, _) => Some(box WalkAction::new(NW, target)),
                    (NumPad9, _) => Some(box WalkAction::new(NE, target)),
                    (NumPad1, _) => Some(box WalkAction::new(SW, target)),
                    (NumPad3, _) => Some(box WalkAction::new(SE, target)),
                    (Char, '.') => Some(box WaitAction { target }),
                    _ => None
                }
            }
            None => None
        }
    }
}

impl RandomBrainComponent {
    /// Convenience method for creating `RandomMovementComponents`.
    pub fn new(bound: Bound) -> RandomBrainComponent {
        RandomBrainComponent { bounds: bound }
    }
}

impl BrainComponent for RandomBrainComponent {
    fn get_action(&mut self, _e: &mut Entity, level: &mut Level) -> Option<Box<dyn Action>> {
        let offset = Point { x: 0, y: 0 };

        let mut offset_x = rand::thread_rng().gen_range(-1, 2i32);
        while !self.bounds.contains(offset.offset_x(offset_x)) {
            offset_x = rand::thread_rng().gen_range(-1, 2i32);
        }

        let mut offset_y = rand::thread_rng().gen_range(-1, 2i32);
        while !self.bounds.contains(offset.offset_y(offset_y)) {
            offset_y = rand::thread_rng().gen_range(-1, 2i32);
        }

        offset.offset(offset_x, offset_y);

        Some(box WalkAction::from_offset(offset, level.current_actor))
    }
}

impl NoBrainComponent {
    pub fn new() -> NoBrainComponent { NoBrainComponent }
}

impl BrainComponent for NoBrainComponent {
    fn get_action(&mut self, _e: &mut Entity, level: &mut Level) -> Option<Box<dyn Action>> {
        Some(box WaitAction { target: level.current_actor })
    }
}
