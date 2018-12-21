use crate::game::Game;
use crate::util::Point;
use crate::actor::Actor;
use crate::player::Player;
use crate::rendering::RenderingComponent;
use crate::map::{DungeonMapComponent, MapComponent, Rect};

use hashbrown::HashMap;

type ItemsMap = HashMap<Point, Item>;

/// Struct containing all of the data necessary
/// for representing a single level of the game.
pub struct Level {
    /// A vector of the friendly and aggressive mobs on the level
    pub mobs: Vec<Actor>,
    /// A vector of all the items on the level
    pub items: ItemsMap,
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

    /// Calls the render method of the following things in order: Map, Mobs, Items
    pub fn render(&mut self, rendering_component: &mut Box<dyn RenderingComponent>) {
        self.map_component.render(rendering_component);
        for item in self.items.values() {
            item.render(rendering_component);
        }
        for i in self.mobs.iter() {
            i.render(rendering_component);
        }
    }

    /// Updates all the living things on the level.
    /// Propagates the turn boolean that indicates
    /// if the player pressed a valid key.
    pub fn update(&mut self, p: &mut Player) -> bool {
        let took_turn = p.update(self);
        if !took_turn {
            return took_turn;
        }
        Game::set_player_point(p.position);
        for i in self.mobs.iter_mut() {
            i.update(&mut self.map_component);
        }
        took_turn
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

    /// Basic render method. See [render_object](../rendering/trait.RenderingComponent.html#tymethod.render_object)
    pub fn render(&self, rendering_component: &mut Box<dyn RenderingComponent>) {
        rendering_component.render_object(self.position, self.display_char);
    }
}

/// Given a vector of rooms, return a hashmap of points containing items
fn place_items(rooms: &Vec<Rect>) -> ItemsMap {
    let mut items = ItemsMap::new();
    for room in rooms {
        let rand_point = room.rand_point();
        items.insert(rand_point, Item::new(rand_point, '?'));
    }
    items
}