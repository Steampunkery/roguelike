use crate::player::Player;
use crate::map::MapComponent;
use crate::util::Point;
use crate::rendering::RenderingComponent;
use crate::movement::{AggroMovementComponent, RandomMovementComponent, MovementComponent};
use crate::movement::NoMovementComponent;

pub trait Actor {
    fn set_position(&mut self, new_pos: Point);
    fn get_last_position(&self) -> &Point;
    fn get_position(&self) -> &Point;
    fn get_health(&mut self) -> &mut i32;
    fn get_display_char(&self) -> char;
}

/// Struct representing both passive and aggressive mobs
pub struct Mob {
    /// The current position of the `Mob`
    pub position: Point,
    /// The last position of the `Mob`
    pub last_position: Point,
    /// The health of the `Mob`
    pub health: i32,
    /// The character to render the `Mob` as
    pub display_char: char,
    /// The movement component dictating the way the `Mob` moves
    pub movement_component: Box<dyn MovementComponent + 'static>,
    /// Whether the mob is aggro'd to the player
    pub is_hostile: bool,
}

impl Actor for Mob {
    fn set_position(&mut self, new_pos: Point) {
        self.last_position = self.position;
        self.position = new_pos;
    }
    fn get_last_position(&self) -> &Point { &self.last_position }
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

impl Mob {
    /// Creates a new actor, with all fields given as parameters
    pub fn new(x: i32, y: i32, health: i32, dc: char, mc: Box<dyn MovementComponent>, is_hostile: bool) -> Mob {
        Mob {
            health,
            is_hostile,
            position: Point { x, y },
            last_position: Point { x: -1, y: -1 },
            display_char: dc,
            movement_component: mc,
        }
    }

    /// The function called on every tick of the game loop to take care of movement and so on
    pub fn update(&mut self, map_component: &mut Box<dyn MapComponent>, player: &Player) {
        if let Some(position) = self.movement_component.update(self.position, map_component, player) {
            self.position = position;
        }
    }

    /// Delegates rendering to the passed rendering component. See [render_object](../rendering/trait.RenderingComponent.html#tymethod.render_object)
    pub fn render(&self, rendering_component: &mut Box<dyn RenderingComponent>) {
        rendering_component.render_object(self.position, self.display_char);
    }

    /// Returns the `Mob`s health
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
