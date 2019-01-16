use crate::game::Game;
use crate::actor::Actor;
use crate::item::ItemsMap;
use crate::player::Player;
use crate::rendering::RenderingComponent;
use crate::map::{DungeonMapComponent, MapComponent};

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
    pub fn new(width: i32, height: i32) -> Level {
        let mc: Box<dyn MapComponent> = box DungeonMapComponent::new(width, height);
        let items = crate::item::place_items(mc.get_rooms());

        Level {
            mobs: vec![],
            items,
            map_component: mc
        }
    }

    /// Calls the render method of the following things in order: Map, Mobs, Items
    pub fn render(&mut self, rendering_component: &mut Box<dyn RenderingComponent>, player: &Player) {
        self.map_component.render(rendering_component);
        for item in self.items.values() {
            item.render(rendering_component);
        }
        for i in self.mobs.iter() {
            i.render(rendering_component);
        }
        player.render(rendering_component);
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