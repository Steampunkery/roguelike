use crate::game::Game;
use crate::actor::Mob;
use crate::actor::Actor;
use crate::item::ItemsMap;
use crate::player::Player;
use crate::rendering::RenderingComponent;
use crate::map::{DungeonMapComponent, MapComponent};

use rand_isaac::IsaacRng;

/// Struct containing all of the data necessary
/// for representing a single level of the game.
pub struct Level {
    /// A vector of the friendly and aggressive mobs on the level
    pub mobs: Vec<Mob>,
    /// A vector of all the items on the level
    pub items: ItemsMap,
    /// The actual `MapComponent` that hold the meat of the level data
    pub map_component: Box<dyn MapComponent + 'static>,
}

impl Level {
    /// Creates a basic level with a default dungeon map and some random items
    pub fn new(width: i32, height: i32, random: &mut IsaacRng) -> Level {
        let mc: Box<dyn MapComponent> = box DungeonMapComponent::new_empty(width, height, random);
        let items = crate::item::place_items(mc.get_rooms(), random);

        Level {
            mobs: vec![],
            items,
            map_component: mc,
        }
    }

    /// Calls the render method of the following things in order: Map, Mobs, Items
    pub fn render(&mut self, rendering_component: &mut Box<dyn RenderingComponent>, player: &Player) {
        self.map_component.render(rendering_component, player);
        for item in self.items.values() {
            item.render(rendering_component);
        }
        for i in self.mobs.iter() {
            i.render(rendering_component);
        }
        player.render(rendering_component);
    }

    /// Updates all the living things on the level.
    pub fn update(&mut self, p: &mut Player) {
        p.update(self);
        if let Some(action) = p.action {
            action.perform(&mut self.map_component);
        } else {
            return;
        }

        for i in self.mobs.iter_mut() {
            i.update(&mut self.map_component, &*p);
        }
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
        for mob in game.level.mobs.iter() {
            mob.render(&mut game.rendering_component);
        }
        game.player.render(&mut game.rendering_component);
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

    fn update(&mut self, game: &mut Game) {
//        game.player.update(&mut game.level);
//        Game::set_player_point(game.player.get_position());
    }
}