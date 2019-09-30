use crate::game::Game;
use crate::actor::Entity;
use crate::item::ItemsMap;
use crate::rendering::RenderingComponent;
use crate::map::{DungeonMapComponent, MapComponent};

use rand_isaac::IsaacRng;

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
    pub input: Option<Key>,
    pub current_actor: usize
}

impl Level {
    /// Creates a basic level with a default dungeon map and some random items
    pub fn new(width: i32, height: i32, random: &mut IsaacRng, p: Option<Entity>) -> Level {
        let mc: Box<dyn MapComponent> = box DungeonMapComponent::new_empty(width, height, random);
        let items = crate::item::place_items(mc.get_rooms(), random);

        let player = if let Some(p) = p {
            p
        } else {
            Entity::player(mc.get_player_start())
        };

        Level {
            items,
            entities: vec![Some(player)],
            map_component: mc,
            input: None,
            current_actor: 0
        }
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

    /// Updates all the living things on the level.
    pub fn update(&mut self) {
        'outer: while self.current_actor < self.entities.len() {
            let mut entity = self.entities[self.current_actor].take().unwrap();
            let action = entity.get_action(self);
            self.entities[self.current_actor] = Some(entity);

            'inner: loop {
                if let Some(action) = &action {
                    let result = action.perform(self);
                    if result {
                        self.current_actor += 1;
                        break 'inner
                    } else {
                        break 'outer
                    }
                } else {
                    return
                }
            }
        }

        self.current_actor = 0;
    }
}

pub trait State {
    fn new() -> Self;
    fn should_update_state(&self) -> bool;

    fn enter(&self) {}
    fn exit(&self) {}

    fn update(&mut self, game: &mut Game);
    fn render(&mut self, game: &mut Game) {
        game.rendering_component.before_render_new_frame();
        game.rendering_component.after_render_new_frame();
    }
}

pub struct PlayState;

impl State for PlayState {
    fn new() -> PlayState {
        PlayState
    }

    fn should_update_state(&self) -> bool {
        true
    }

    fn update(&mut self, _game: &mut Game) {
//        game.player.update(&mut game.level);
//        Game::set_player_point(game.player.get_position());
    }
}