use crate::game::Game;
use crate::util::Point;
use crate::item::Item;
use crate::level::Level;
use crate::actor::Actor;
use crate::rendering::RenderingComponent;
use crate::action::{WalkAction, WaitAction, Action};

/// Struct representing the player.
pub struct Player<'a> {
    /// The current position of the player.
    position: Point,
    /// The last position of the player
    last_position: Point,
    /// The health of the player.
    pub health: i32,
    /// The character to use to render the player.
    pub display_char: char,
    /// The inventory of the player.
    pub inventory: Vec<Item>,
    /// The currently wielded item of the player
    pub wielded: Option<&'a Item>,
    pub action: Option<Box<Action>>,
}

impl<'a> Actor for Player<'a> {
    fn set_position(&mut self, new_pos: Point) {
        self.last_position = self.position;
        self.position = new_pos;
    }
    fn get_last_position(&self) -> &Point {
        &self.last_position
    }
    fn get_position(&self) -> &Point {
        &self.position
    }
    fn get_health(&mut self) -> &mut i32 {
        &mut self.health
    }
    fn get_display_char(&self) -> char {
        self.display_char
    }
}

impl<'a> Player<'a> {
    /// Creates a new player with some default values.
    /// Health at 15, display character of '@'.
    pub fn new(start_pos: Point) -> Player<'a> {
        Player {
            position: start_pos,
            last_position: Point { x: -1, y: -1 },
            health: 15,
            display_char: '@',
            inventory: vec![],
            wielded: None,
            action: None,
        }
    }

    /// Basic update method with a twist.
    /// If there is not a keypress, return false
    /// so the game loop gets player input again.
    /// Also controls the non-movement turns of the player.
    pub fn update(&mut self, level: &mut Level) {
        use crate::action::Direction::*;
        use tcod::input::KeyCode::*;

        // TODO: Consider replacing this match with a method named something like "match_action"
        self.action = match Game::get_last_keypress() {
            Some(keypress) => {
                match (keypress.code, keypress.printable) {
                    (NumPad8, _) | (Up, _) => Some(box WalkAction::new(N, self)),
                    (NumPad2, _) | (Down, _) => Some(box WalkAction::new(S, self)),
                    (NumPad4, _) | (Left, _) => Some(box WalkAction::new(W, self)),
                    (NumPad6, _) | (Right, _) => Some(box WalkAction::new(E,self)),
                    (NumPad7, _) => Some(box WalkAction::new(NW, self)),
                    (NumPad9, _) => Some(box WalkAction::new(NE, self)),
                    (NumPad1, _) => Some(box WalkAction::new(SW, self)),
                    (NumPad3, _) => Some(box WalkAction::new(SE, self)),
                    (Char, '.') => Some(box WaitAction),
                    _ => None
                }
            }
            None => None
        };
    }

    /// Delegates rendering to the given rendering component. See [render_object](../rendering/trait.RenderingComponent.html#tymethod.render_object)
    pub fn render(&self, rendering_component: &mut Box<dyn RenderingComponent>) {
        rendering_component.render_object(self.position, self.display_char);
    }

    fn pickup(&mut self, level: &mut Level) -> bool {
        if let Some(item) = level.items.remove(&self.position) {
            self.inventory.push(item);
            return true;
        }
        false
    }

//    fn wield(&mut self, rendering_component: &mut Box<dyn RenderingComponent>) -> bool {
//        let mut keypress = rendering_component.wait_for_keypress();
//        loop {
//            if let Some(index) = keypress.printable.to_digit(10) {
//
//            }
//        }
//    }
}