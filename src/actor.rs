use crate::util::Point;
use crate::rendering::RenderingComponent;
use crate::brain::{AggroBrainComponent, BrainComponent};
use crate::action::Action;
use crate::item::Item;
use crate::level::Level;
use crate::brain::PlayerBrainComponent;
use crate::brain::NoBrainComponent;


pub trait Actor {
    fn set_position(&mut self, new_pos: Point);
    fn get_last_position(&self) -> Point;
    fn get_position(&self) -> Point;
    fn get_health(&mut self) -> &mut i32;
    fn get_display_char(&self) -> char;
}

/// Struct representing both passive and aggressive mobs
pub struct Entity {
    /// The current position of the `Entity`
    pub position: Point,
    /// The last position of the `Entity`
    pub last_position: Point,
    /// The health of the `Entity`
    pub health: i32,
    /// The character to render the `Entity` as
    pub display_char: char,
    /// The movement component dictating the way the `Entity` moves
    pub brain_component: Box<dyn BrainComponent + 'static>,
    /// Whether the mob is aggro'd to the player
    pub is_hostile: bool,

    pub player: bool,
    pub inventory: Vec<Item>,
    pub wielded: Option<usize>,
}

impl Actor for Entity {
    fn set_position(&mut self, new_pos: Point) {
        self.last_position = self.position;
        self.position = new_pos;
    }
    fn get_last_position(&self) -> Point { self.last_position }
    fn get_position(&self) -> Point {
        self.position
    }
    fn get_health(&mut self) -> &mut i32 {
        &mut self.health
    }
    fn get_display_char(&self) -> char {
        self.display_char
    }
}

impl Entity {
    /// Creates a new actor, with all fields given as parameters
    pub fn new(x: i32, y: i32, health: i32, dc: char, bc: Box<dyn BrainComponent>, is_hostile: bool) -> Entity {
        Entity {
            health,
            is_hostile,
            position: Point { x, y },
            last_position: Point { x: -1, y: -1 },
            display_char: dc,
            brain_component: bc,
            inventory: vec![],
            wielded: None,
            player: false,
        }
    }

    /// The function called on every tick of the game loop to take care of movement and so on
    pub fn get_action(&mut self, level: &mut Level) -> Option<Box<dyn Action>> {
        let mut brain_component = std::mem::replace(&mut self.brain_component, box NoBrainComponent);
        let action = brain_component.get_action(self, level);
        self.brain_component = brain_component;
        action
    }

    /// Delegates rendering to the passed rendering component. See [render_object](../rendering/trait.RenderingComponent.html#tymethod.render_object)
    pub fn render(&self, rendering_component: &mut Box<dyn RenderingComponent>) {
        rendering_component.render_object(self.position, self.display_char);
    }

    /// Returns the `Entity`s health
    pub fn get_health(&self) -> i32 {
        self.health
    }

    /// Creates an `Entity` with traits of a kobold
    pub fn kobold(x: i32, y: i32) -> Entity {
        let mc: Box<AggroBrainComponent> = box AggroBrainComponent::new();
        Entity::new(x, y, 12, 'k', mc, true)
    }

    pub fn player(start_pos: Point) -> Entity {
        Entity {
            position: start_pos,
            is_hostile: false,
            last_position: Point { x: -1, y: -1 },
            health: 15,
            display_char: '@',
            inventory: vec![],
            wielded: None,
            brain_component: box PlayerBrainComponent::new(),
            player: true,
        }
    }
}
