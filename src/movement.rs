use crate::game::Game;
use crate::map::MapComponent;

use crate::ai::DEBUG_AI;
use crate::ai::find_astar_path;

use crate::util::{Bound, Point};
use crate::util::Contains::*;

use rand::Rng;
use tcod::colors::Color;
use tcod::input::KeyCode;

pub trait MovementComponent {
    fn update(&mut self, position: Point, map_component: &mut Box<MapComponent>) -> Option<Point>;
}

pub struct RandomMovementComponent {
    window_bounds: Bound
}

pub struct TcodUserMovementComponent;

pub struct AggroMovementComponent {
    path: Vec<Point>,
}

impl AggroMovementComponent {
    pub fn new() -> AggroMovementComponent {
        AggroMovementComponent { path: vec![] }
    }

    pub fn show_ai(&mut self, map: &mut Box<MapComponent>) {
        if DEBUG_AI {
            if !self.path.is_empty() {
                for point in &self.path {
                    map.get_map_mut()[point.x as usize][point.y as usize].color_override = Some(Color { r: 255, g: 0, b: 0 });
                }
            }
        }
    }
}

impl MovementComponent for AggroMovementComponent {
    fn update(&mut self, position: Point, map_component: &mut Box<MapComponent>) -> Option<Point> {
        let char_point = Game::get_character_point();
        let last_char_point = Game::get_last_character_point();

        if DEBUG_AI { self.show_ai(map_component); }

        if char_point != last_char_point || self.path.is_empty() {
            let path_opt = find_astar_path(map_component, position, char_point);

            if let Some(path) = path_opt {
                self.path = path;

                // 0th element is the current position
                self.path.remove(0);
                self.path.reverse();

                return if !self.path.is_empty() { Some(self.path.pop().unwrap()) } else { None }
            }
        } else if char_point == last_char_point && !self.path.is_empty() {
            return Some(self.path.pop().unwrap());
        }

        None
    }
}

impl TcodUserMovementComponent {
    pub fn new() -> TcodUserMovementComponent {
        TcodUserMovementComponent
    }
}

impl MovementComponent for TcodUserMovementComponent {
    fn update(&mut self, position: Point, map_component: &mut Box<MapComponent>) -> Option<Point> {
        let offset = match Game::get_last_keypress() {
            Some(keypress) => {
                match (keypress.code, keypress.printable) {
                    (KeyCode::Up, _) => {
                        Some(position.offset_y(-1))
                    }
                    (KeyCode::Down, _) => {
                        Some(position.offset_y(1))
                    }
                    (KeyCode::Left, _) => {
                        Some(position.offset_x(-1))
                    }
                    (KeyCode::Right, _) => {
                        Some(position.offset_x(1))
                    }
                    (KeyCode::Char, '.') => {
                        Some(position)
                    }
                    _ => None
                }
            }
            None => None
        };

        if let Some(movement) = offset {
            match map_component.is_blocked(movement.x, movement.y) {
                false => {
                    Game::set_last_character_point(position);
                    return offset;
                }
                true => return None
            }
        }
        None
    }
}

impl RandomMovementComponent {
    pub fn new(bound: Bound) -> RandomMovementComponent {
        RandomMovementComponent { window_bounds: bound }
    }
}

impl MovementComponent for RandomMovementComponent {
    fn update(&mut self, position: Point, map_component: &mut Box<MapComponent>) -> Option<Point> {
        let mut offset = Point { x: position.x, y: position.y };
        let offset_x = rand::thread_rng().gen_range(0, 3i32) - 1;
        match self.window_bounds.contains(offset.offset_x(offset_x)) {
            DoesContain => offset = offset.offset_x(offset_x),
            DoesNotContain => { return Some(position); }
        }

        let offset_y = rand::thread_rng().gen_range(0, 3i32) - 1;
        match self.window_bounds.contains(offset.offset_y(offset_y)) {
            DoesContain => offset = offset.offset_y(offset_y),
            DoesNotContain => { return Some(position); }
        }

        Some(offset)
    }
}
