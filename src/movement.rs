use crate::map::MapComponent;

use crate::ai::DEBUG_AI;
use crate::ai::find_astar_path;

use crate::util::Point;

use crate::actor::Actor;
use crate::player::Player;

use tcod::colors::Color;

/// A trait for defining a method of movement
/// that may be applied to any living monster.
pub trait MovementComponent {
    /// The method that decides the next move according to the implementation.
    fn update(&mut self, position: Point, map_component: &mut Box<dyn MapComponent>, player: &Player) -> Option<Point>;
}

/// A movement component that uses A* to find the
/// fastest path to the player.
pub struct AggroMovementComponent {
    path: Vec<Point>,
}

pub struct NoMovementComponent;

impl AggroMovementComponent {
    /// Convenience method for creating `AggroMovementComponent`s.
    pub fn new() -> AggroMovementComponent {
        AggroMovementComponent { path: vec![] }
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

impl MovementComponent for AggroMovementComponent {
    fn update(&mut self, position: Point, map_component: &mut Box<dyn MapComponent>, player: &Player) -> Option<Point> {
        let char_point = player.get_position();
        let last_char_point = player.get_last_position();

        if DEBUG_AI { self.show_ai(map_component); }

        if char_point != last_char_point || self.path.is_empty() {
            let path_opt = find_astar_path(map_component, position, *char_point);

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
    /// Convenience method for creating `TcodUserMovementComponents`.
    pub fn new() -> TcodUserMovementComponent {
        TcodUserMovementComponent
    }
}

impl MovementComponent for TcodUserMovementComponent {
    fn update(&mut self, position: Point, map_component: &mut Box<dyn MapComponent>) -> Option<Point> {
        use tcod::input::KeyCode::*;
        let offset = match Game::get_last_keypress() {
            Some(keypress) => {
                match (keypress.code, keypress.printable) {
                    (NumPad8, _) | (Up, _) => Some(position.offset_y(-1)),
                    (NumPad2, _) | (Down, _) => Some(position.offset_y(1)),
                    (NumPad4, _) | (Left, _) => Some(position.offset_x(-1)),
                    (NumPad6, _) | (Right, _) => Some(position.offset_x(1)),
                    (NumPad7, _) => Some(position.offset(-1, -1)),
                    (NumPad9, _) => Some(position.offset(1, -1)),
                    (NumPad1, _) => Some(position.offset(-1, 1)),
                    (NumPad3, _) => Some(position.offset(1, 1)),
                    (Char, '.') => Some(position),
                    _ => None
                }
            }
            None => None
        };

        if let Some(movement) = offset {
            match map_component.is_blocked(movement.x, movement.y) {
                false => {
                    Game::set_last_player_point(position);
                    return offset;
                }
                true => return None
            }
        }

        None
    }
}

impl RandomMovementComponent {
    /// Convenience method for creating `RandomMovementComponents`.
    pub fn new(bound: Bound) -> RandomMovementComponent {
        RandomMovementComponent { window_bounds: bound }
    }
}

impl MovementComponent for RandomMovementComponent {
    fn update(&mut self, position: Point, _map_component: &mut Box<dyn MapComponent>) -> Option<Point> {
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

impl NoMovementComponent {
    pub fn new() -> NoMovementComponent { NoMovementComponent }
}

impl MovementComponent for NoMovementComponent {
    fn update(&mut self, _position: Point, _map_component: &mut Box<dyn MapComponent>) -> Option<Point> {
        None
    }
}
