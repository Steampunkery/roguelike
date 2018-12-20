use crate::game::Game;
use crate::util::Point;
use crate::map::MapComponent;
use crate::rendering::RenderingComponent;
use crate::movement::{TcodUserMovementComponent, MovementComponent};

/// Struct representing the player
pub struct Player {
    /// The current position of the player
    pub position: Point,
    /// The health of the player
    pub health: i32,
    /// The character to use to render the player
    pub display_char: char,
    /// The movement component dictating the movement of the player
    pub movement_component: Box<dyn MovementComponent + 'static>,
}

impl Player {
    /// Creates a new player with some default values.
    /// Health at 15, display character of '@'.
    pub fn new() -> Player {
        let mc: Box<TcodUserMovementComponent> = box TcodUserMovementComponent::new();
        Player { position: Game::get_player_point(), health: 15, display_char: '@', movement_component: mc }
    }

    /// Basic update method with a twist.
    /// If there is not a keypress, return false
    /// so the game loop gets player input again
    pub fn update(&mut self, map_component: &mut Box<dyn MapComponent>) -> bool {
        if let Some(position) = self.movement_component.update(self.position, map_component) {
            self.position = position;
            return true;
        }
        false
    }

    /// Delegates rendering to the given rendering component. See [render_object](../rendering/trait.RenderingComponent.html#tymethod.render_object)
    pub fn render(&self, rendering_component: &mut Box<dyn RenderingComponent>) {
        rendering_component.render_object(self.position, self.display_char);
    }
}