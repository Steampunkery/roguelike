use crate::game::Game;
use crate::util::Point;
use crate::item::Item;
use crate::level::Level;
use crate::rendering::RenderingComponent;
use crate::movement::{TcodUserMovementComponent, MovementComponent};

use tcod::input::KeyCode;

/// Struct representing the player.
pub struct Player<'a> {
    /// The current position of the player.
    pub position: Point,
    /// The health of the player.
    pub health: i32,
    /// The character to use to render the player.
    pub display_char: char,
    /// The inventory of the player.
    pub inventory: Vec<Item>,
    /// The movement component dictating the movement of the player
    pub movement_component: Box<dyn MovementComponent + 'static>,
    /// The currently wielded item of the player
    pub wielded: Option<&'a Item>
}

impl<'a> Player<'a> {
    /// Creates a new player with some default values.
    /// Health at 15, display character of '@'.
    pub fn new(start_pos: Point) -> Player<'a> {
        let mc: Box<TcodUserMovementComponent> = box TcodUserMovementComponent::new();
        Player {
            position: start_pos,
            health: 15,
            display_char: '@',
            inventory: vec![],
            movement_component: mc,
            wielded: None
        }
    }

    /// Basic update method with a twist.
    /// If there is not a keypress, return false
    /// so the game loop gets player input again.
    /// Also controls the non-movement turns of the player.
    pub fn update(&mut self, level: &mut Level) -> bool {
        // TODO: Consider replacing this match with a method named something like "match_action"
        match Game::get_last_keypress() {
            Some(keypress) => {
                match (keypress.code, keypress.printable) {
                    (KeyCode::Char, ',') => {
                        // Important: Only return if the action is successful.
                        // Else, we want to let the player try to make a move.
                        if self.pickup(level) {
                            return true
                        }
                    }
                    _ => ()
                }
            }
            None => ()

        };
        if let Some(position) = self.movement_component.update(self.position, &mut level.map_component) {
            self.position = position;
            return true;
        }
        false
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