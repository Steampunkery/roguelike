extern crate tcod;

use util::{Point, Bound};
use rendering::{TcodRenderingComponent, RenderingComponent};
use actor::Actor;
use map::{DungeonMapComponent, MapComponent};

use self::tcod::input::Key;

static mut LAST_KEYPRESS: Option<Key> = None;
static mut CHAR_LOCATION: Point = Point { x: 0, y: 0 };

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 50;

pub struct Game {
    pub exit: bool,
    pub window_bounds: Bound,
    pub rendering_component: Box<RenderingComponent + 'static>,
    pub map_component: Box<MapComponent + 'static>
}

impl Game {
    pub fn new() -> Game {
        let bounds = Bound {
            min: Point { x: 0, y: 0 },
            max: Point { x: MAP_WIDTH, y: MAP_HEIGHT },
        };
        let rc: Box<TcodRenderingComponent> = box TcodRenderingComponent::new(bounds);
        let mc: Box<DungeonMapComponent> = box DungeonMapComponent::new();

        unsafe { CHAR_LOCATION = mc.player_start };
        Game {
            exit: false,
            window_bounds: bounds,
            rendering_component: rc,
            map_component: mc
        }
    }

    pub fn get_last_keypress() -> Option<Key> {
        unsafe { LAST_KEYPRESS }
    }

    pub fn set_last_keypress(ks: Key) {
        unsafe { LAST_KEYPRESS = Some(ks); }
    }

    pub fn get_character_point() -> Point {
        unsafe { CHAR_LOCATION }
    }

    pub fn set_character_point(point: Point) {
        unsafe { CHAR_LOCATION = point; }
    }

    pub fn render(&mut self, npcs: &Vec<Actor>, c: &Actor) {
        self.rendering_component.before_render_new_frame();
        self.map_component.render(&mut self.rendering_component);
        for i in npcs.iter() {
            i.render(&mut self.rendering_component);
        }
        c.render(&mut self.rendering_component);
        self.rendering_component.after_render_new_frame();
    }

    pub fn update(&mut self, npcs: &mut Vec<Actor>, c: &mut Actor) {
        c.update(&self.map_component);
        Game::set_character_point(c.position);
        for i in npcs.iter_mut() {
            i.update(&self.map_component);
        }
    }

    pub fn wait_for_keypress(&mut self) -> Key {
        let ks = self.rendering_component.wait_for_keypress();
        Game::set_last_keypress(ks);
        return ks;
    }
}
