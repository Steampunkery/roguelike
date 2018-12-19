use crate::game::Game;
use crate::map::MapComponent;
use crate::util::{Point, Bound};
use crate::rendering::RenderingComponent;
use crate::movement::{AggroMovementComponent, RandomMovementComponent, TcodUserMovementComponent, MovementComponent};

/// Struct representing both passive and aggressive mobs, as well as the player
pub struct Actor {
    /// The current position of the `Actor`
    pub position: Point,
    /// The health of the `Actor`
    pub health: i32,
    /// Whether the `Actor` is the player or not
    pub is_player: bool,
    /// The character to render the `Actor` as
    pub display_char: char,
    /// The movement component dictating the way the `Actor` moves
    pub movement_component: Box<dyn MovementComponent + 'static>,
}

impl Actor {
    /// Creates a new actor, with all fields given as parameters
    pub fn new(x: i32, y: i32, health: i32, is_player: bool, dc: char, mc: Box<dyn MovementComponent>) -> Actor {
        Actor { position: Point { x, y }, health, is_player, display_char: dc, movement_component: mc }
    }

    /// The function called on every tick of the game loop to take care of movement and so on
    pub fn update(&mut self, map_component: &mut Box<dyn MapComponent>) {
        if let Some(position) = self.movement_component.update(self.position, map_component) {
            self.position = position;
        }
    }

    /// Delegates rendering to the passed rendering component
    pub fn render(&self, rendering_component: &mut Box<dyn RenderingComponent>) {
        rendering_component.render_object(self.position, self.display_char);
    }

    /// Returns the `Actor`s health
    pub fn get_health(&self) -> i32 {
        self.health
    }

    /// Creates an `Actor` with traits of a dog
    pub fn dog(x: i32, y: i32, bound: Bound) -> Actor {
        let mc: Box<RandomMovementComponent> = box RandomMovementComponent::new(bound);
        Actor::new(x, y, 10, false, 'd', mc)
    }

    /// Creates an `Actor` with traits of a cat
    pub fn cat(x: i32, y: i32, bound: Bound) -> Actor {
        let mc: Box<RandomMovementComponent> = box RandomMovementComponent::new(bound);
        Actor::new(x, y, 5, false, 'c', mc)
    }

    /// Creates an the character to be played as
    pub fn player() -> Actor {
        let point = Game::get_character_point();
        let mc: Box<TcodUserMovementComponent> = box TcodUserMovementComponent::new();
        Actor::new(point.x, point.y, 15, true, '@', mc)
    }

    /// Creates an `Actor` with traits of a kobold
    pub fn kobold(x: i32, y: i32) -> Actor {
        let mc: Box<AggroMovementComponent> = box AggroMovementComponent::new();
        Actor::new(x, y, 12, false, 'k', mc)
    }
}
