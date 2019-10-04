use crate::actor::{Actor, Entity};
use crate::item::ItemsMap;
use crate::rendering::RenderingComponent;
use crate::map::{DungeonMapComponent, MapComponent};

use rand_isaac::IsaacRng;
use rand::Rng;

use tcod::input::Key;

/// Struct containing all of the data necessary
/// for representing a single level of the game.
pub struct Level {
    /// A vector of the friendly and aggressive entities on the level
    pub entities: Vec<Option<Entity>>,
    /// A vector of all the items on the level
    pub items: ItemsMap,
    /// The actual `MapComponent` that hold the meat of the level data
    pub map_component: Box<dyn MapComponent + 'static>,
    /// A vector of messages to show to the player
    pub message_queue: Vec<String>,
    /// Where we are in the massages vector
    pub message_cache: Vec<String>,
    /// Input handed down from the Game
    pub input: Option<Key>,
    /// The entity who can act at a given moment
    pub current_actor: usize
}

impl Level {
    /// Creates a basic level with a default dungeon map and some random items
    pub fn new(width: i32, height: i32, random: &mut IsaacRng, p: Option<Entity>) -> Level {
        let mut mc: Box<dyn MapComponent> = box DungeonMapComponent::new_empty(width, height, random);
        let items = crate::item::place_items(mc.get_rooms(), random);
        let player_pos = mc.get_player_start();

        let mut entities = vec![];
        entities.push(if p.is_some() {
            let mut player = p.unwrap();
            player.set_position(player_pos);
            Some(player)
        } else {
            Some(Entity::player(player_pos))
        });

        mc.get_map_mut()[player_pos.x as usize][player_pos.y as usize].occupied = true;

        for _ in 0..3 {
            // Get a random room
            let room_num = random.gen_range(0, mc.get_rooms().len());
            let room = mc.get_rooms()[room_num];

            // Pick random coordinates in that room
            let rand_point = room.rand_point(random);

            // Spawn a monster there
            entities.push(Some(Entity::kobold(rand_point.x, rand_point.y)));
        }

        Level {
            items,
            entities,
            map_component: mc,
            message_queue: vec!["Welcome to MR: TOM".to_string()],
            message_cache: vec![],
            input: None,
            current_actor: 0
        }
    }

    /// Updates all the living things on the level.
    pub fn update(&mut self) {
        'outer: while self.current_actor < self.entities.len() {
            let mut entity = self.entities[self.current_actor].take().unwrap();
            let mut action = entity.get_action(self);
            self.entities[self.current_actor] = Some(entity);

            if action.is_some() {
                'inner: loop {
                    let act = action.unwrap();
                    let result = act.perform(self);
                    if !result.success { return }
                    if result.alternate.is_none() { break 'inner }
                    action = result.alternate;
                }
                self.current_actor += 1;
            } else {
                return
            }

        }

        self.current_actor = 0;
    }

    /// Calls the render method of the following things in order: Map, Mobs, Items
    pub fn render(&mut self, rendering_component: &mut Box<dyn RenderingComponent>) {
        self.map_component.render(rendering_component, &self.entities[0].as_ref().unwrap());

        for item in self.items.values() {
            item.render(rendering_component);
        }

        // reverse to render the player last because it's always 0
        for i in self.entities.iter().rev() {
            i.as_ref().unwrap().render(rendering_component);
        }
    }

    pub fn game_log(&mut self, message: String) {
        self.message_queue.push(message);
    }
}