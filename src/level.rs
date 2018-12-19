use crate::util::Point;
use crate::actor::Actor;
use crate::rendering::RenderingComponent;
use crate::map::{DungeonMapComponent, MapComponent, Rect};

/// Struct containing all of the data necessary
/// for representing a single level of the game.
pub struct Level {
    /// A vector of the friendly and aggressive mobs on the level
    pub mobs: Vec<Actor>,
    /// A vector of all the items on the level
    pub items: Vec<Item>,
    /// The actual `MapComponent` that hold the meat of the level data
    pub map_component: Box<dyn MapComponent + 'static>,
}

impl Level {
    /// Creates a basic level with a default dungeon map and some random items
    pub fn new() -> Level {
        let mc: Box<dyn MapComponent> = box DungeonMapComponent::new();
        let items = place_items(mc.get_rooms());

        Level {
            mobs: vec![],
            items,
            map_component: mc
        }
    }

    pub fn render(&mut self, rendering_component: &mut Box<dyn RenderingComponent>) {
        self.map_component.render(rendering_component);
        for item in &self.items {
            item.render(rendering_component);
        }
    }
}

/// Struct representing a single item on the map
pub struct Item {
    /// The position of the item
    pub position: Point,
    /// The char used to render the item
    pub display_char: char,
}

impl Item {
    /// Convenience method for creating items
    pub fn new(position: Point, display_char: char) -> Item {
        Item {
            position,
            display_char
        }
    }

    pub fn render(&self, rendering_component: &mut Box<dyn RenderingComponent>) {
        rendering_component.render_object(self.position, self.display_char);
    }
}

/// Given a vector of rooms, return a hashmap of points containing items
fn place_items(rooms: &Vec<Rect>) -> Vec<Item> {
    let mut items = vec![];
    for room in rooms {
        let rand_point = room.rand_point();
        items.push(Item::new(rand_point, '?'));
    }
    items
}