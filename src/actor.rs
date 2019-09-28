use crate::map::MapComponent;
use crate::util::{Point, Bound};
use crate::rendering::RenderingComponent;
use crate::movement::{AggroMovementComponent, RandomMovementComponent, MovementComponent};
use crate::movement::NoMovementComponent;

/// Struct representing both passive and aggressive mobs
pub struct Actor {
    /// The current position of the `Actor`
    pub position: Point,
    /// The health of the `Actor`
    pub health: i32,
    /// The character to render the `Actor` as
    pub display_char: char,
    /// The movement component dictating the way the `Actor` moves
    pub movement_component: Box<dyn MovementComponent + 'static>,
    /// Whether the mob is aggro'd to the player
    pub is_hostile: bool,
}

impl Actor {
    /// Creates a new actor, with all fields given as parameters
    pub fn new(x: i32, y: i32, health: i32, dc: char, mc: Box<dyn MovementComponent>, is_hostile: bool) -> Actor {
        Actor { position: Point { x, y }, health, display_char: dc, movement_component: mc, is_hostile }
    }

    /// The function called on every tick of the game loop to take care of movement and so on
    pub fn update(&mut self, map_component: &mut Box<dyn MapComponent>) {
        if let Some(position) = self.movement_component.update(self.position, map_component) {
            self.position = position;
        }
    }

    /// Delegates rendering to the passed rendering component. See [render_object](../rendering/trait.RenderingComponent.html#tymethod.render_object)
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
        Actor::new(x, y, 10, 'd', mc, false)
    }

    /// Creates an `Actor` with traits of a cat
    pub fn cat(x: i32, y: i32, bound: Bound) -> Actor {
        let mc: Box<RandomMovementComponent> = box RandomMovementComponent::new(bound);
        Actor::new(x, y, 5, 'c', mc, false)
    }

    /// Creates an `Actor` with traits of a kobold
    pub fn kobold(x: i32, y: i32) -> Actor {
        let mc: Box<NoMovementComponent> = box NoMovementComponent::new();
        Actor::new(x, y, 12, 'k', mc, true)
    }
}
