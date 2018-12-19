use crate::level::Level;
use crate::actor::Actor;
use crate::util::{Point, Bound};
use crate::rendering::{RenderingComponent, TcodRenderingComponent};

use tcod::input::Key;

static mut LAST_KEYPRESS: Option<Key> = None;
static mut LAST_CHAR_POSITION: Point = Point { x: -1, y: -1 };
static mut CHAR_LOCATION: Point = Point { x: 0, y: 0 };

// Make these constants two more than you want them to actually be
/// The width of the map display area
pub const MAP_WIDTH: i32 = 82;
/// The height of the map display area
pub const MAP_HEIGHT: i32 = 52;

/// Game struct containing all the information about the current game state
pub struct Game {
    /// Whether the game should exit on the next loop
    pub exit: bool,
    /// The boundaries of the window (the size of the map display area)
    pub window_bounds: Bound,
    /// The component for rendering all the tiles in the game
    pub rendering_component: Box<dyn RenderingComponent + 'static>,
    /// A `Level` struct containing all the information on the current level
    pub level: Level
}

impl Game {
    /// Creates a new game struct complete with a first level and rendering component
    pub fn new() -> Game {
        let bounds = Bound {
            min: Point { x: 0, y: 0 },
            max: Point { x: MAP_WIDTH, y: MAP_HEIGHT },
        };

        let level = Level::new();
        let rc: Box<TcodRenderingComponent> = box TcodRenderingComponent::new(bounds, &level.map_component);

        unsafe { CHAR_LOCATION = level.map_component.get_player_start() };
        Game {
            exit: false,
            window_bounds: bounds,
            rendering_component: rc,
            level
        }
    }

    /// Delegates rendering of the map, mobs, and character to the `rendering_component` in the correct order
    pub fn render(&mut self, c: &Actor) {
        self.rendering_component.before_render_new_frame();
        self.level.render(&mut self.rendering_component);
        for i in self.level.mobs.iter() {
            i.render(&mut self.rendering_component);
        }
        c.render(&mut self.rendering_component);
        self.rendering_component.after_render_new_frame();
    }

    /// Calls the update methods of ALL objects in the domain of the game. Think player, items, mobs, etc.
    pub fn update(&mut self, c: &mut Actor) {
        c.update(&mut self.level.map_component);
        Game::set_character_point(c.position);
        for i in self.level.mobs.iter_mut() {
            i.update(&mut self.level.map_component);
        }
    }

    /// Returns the last keypress received by the game loop
    pub fn get_last_keypress() -> Option<Key> {
        unsafe { LAST_KEYPRESS }
    }

    /// Sets the last keypress as it is received from the game loop
    pub fn set_last_keypress(ks: Key) {
        unsafe { LAST_KEYPRESS = Some(ks); }
    }

    /// Returns the current `Point` the character is at
    pub fn get_character_point() -> Point {
        unsafe { CHAR_LOCATION }
    }

    /// Sets the current `Point` that the character is at
    pub fn set_character_point(point: Point) {
        unsafe { CHAR_LOCATION = point; }
    }

    /// Returns the previous position of the character
    pub fn get_last_character_point() -> Point {
        unsafe { LAST_CHAR_POSITION }
    }

    /// Sets the previous position of the character
    pub fn set_last_character_point(point: Point) {
        unsafe { LAST_CHAR_POSITION = point; }
    }

    /// Receives the keypresses in the game loop
    pub fn wait_for_keypress(&mut self) -> Key {
        let ks = self.rendering_component.wait_for_keypress();
        Game::set_last_keypress(ks);
        return ks;
    }
}
