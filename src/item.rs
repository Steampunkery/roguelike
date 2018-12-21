use crate::map::Rect;
use crate::util::Point;
use crate::rendering::RenderingComponent;

use hashbrown::HashMap;
use downcast_rs::{Downcast, impl_downcast};

pub type ItemsMap = HashMap<Point, Box<Item>>;

/// Struct representing a single item on the map
pub trait Item: Downcast {
    /// Convenience method for creating new items
    fn new(position: Point, display_char: char) -> Self where Self: Sized;
    /// Use the item
    fn apply(&mut self) -> bool { true }
    /// Return the position of the item
    fn position(&self) -> Point;
    /// Return the display character of the item
    fn display_char(&self) -> char;
    /// Basic render method. See [render_object](../rendering/trait.RenderingComponent.html#tymethod.render_object)
    fn render(&self, rendering_component: &mut Box<dyn RenderingComponent>) {
        rendering_component.render_object(self.position(), self.display_char());
    }
}
impl_downcast!(Item);

pub struct Weapon {
    pub position: Point,
    pub display_char: char,
}

impl Item for Weapon {
    /// Convenience method for creating items
    fn new(position: Point, display_char: char) -> Weapon {
        Weapon {
            position,
            display_char
        }
    }

    fn position(&self) -> Point {
        self.position
    }

    fn display_char(&self) -> char {
        self.display_char
    }
}

/// Given a vector of rooms, return a hashmap of points containing items
pub fn place_items(rooms: &Vec<Rect>) -> ItemsMap {
    let mut items = ItemsMap::new();
    let room = rooms[0];
    let rand_point = room.rand_point();
    items.insert(rand_point, box Weapon::new(rand_point, ')'));

    items
}