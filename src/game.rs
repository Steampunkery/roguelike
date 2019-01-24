use crate::level::Level;
use crate::player::Player;
use crate::util::{Point, Bound};
use crate::rendering::{RenderingComponent, TcodRenderingComponent};

use tcod::input::Key;

static mut LAST_KEYPRESS: Option<Key> = None;
static mut LAST_PLAYER_POS: Point = Point { x: -1, y: -1 };
static mut PLAYER_POS: Point = Point { x: 0, y: 0 };

/// The y offset of the map from the top
pub const MAP_OFFSET: i32 = 5; // 1 line for messages, one for padding
/// The width of the map display area
pub const MAP_WIDTH: i32 = 80;
/// The height of the map display area
pub const MAP_HEIGHT: i32 = 50;

/// Game struct containing all the information about the current game state
pub struct Game<'a> {
    /// Whether the game should exit on the next loop
    pub exit: bool,
    pub player: Player<'a>,
    /// The boundaries of the window (the size of the map display area)
    pub window_bounds: Bound,
    /// The component for rendering all the tiles in the game
    pub rendering_component: Box<dyn RenderingComponent + 'static>,
    /// A `Level` struct containing all the information on the current level
    pub level: Level,
    /// Whether the player made a move
    pub did_take_turn: bool,
    /// A vector of messages to show to the player
    pub messages: Vec<String>,
    /// Where we are in the massages vector
    pub message_seek: usize,
}

impl<'a> Game<'a> {
    /// Creates a new game struct complete with a first level and rendering component
    pub fn new() -> Game<'a> {
        let bounds = Bound {
            min: Point { x: 0, y: 0 },
            max: Point { x: MAP_WIDTH, y: MAP_HEIGHT + MAP_OFFSET },
        };

        let level = Level::new(MAP_WIDTH, MAP_HEIGHT);
        let rc: Box<TcodRenderingComponent> = box TcodRenderingComponent::new(bounds, &level.map_component);

        let p_start = level.map_component.get_player_start();
        Self::set_player_point(p_start);
        let p = Player::new(p_start);
        
        Game {
            level,
            player: p,
            exit: false,
            window_bounds: bounds,
            rendering_component: rc,
            did_take_turn: false,
            messages: vec!["Welcome to MR: TOM".to_string()],
            message_seek: 0,
        }
    }

    /// Delegates rendering of the map, mobs, and player to the `rendering_component` in the correct order
    pub fn render(&mut self) {
        self.rendering_component.before_render_new_frame();

        self.refresh_messages();

        self.level.render(&mut self.rendering_component, &self.player);

        self.rendering_component.after_render_new_frame();
    }

    /// Calls the update methods of ALL objects in the domain of the game. Think player, items, mobs, etc.
    pub fn update(&mut self) {
        self.did_take_turn = self.level.update(&mut self.player);
    }

    pub fn refresh_messages(&mut self) {
        self.message_seek = if self.messages.len() < 5 { 0 } else { self.messages.len() % 5 };
        for (i, message) in self.messages.iter().skip(self.message_seek).enumerate() {
            self.rendering_component.write_message(message, 0, (i as i32 - 4).abs());
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

    /// Returns the current `Point` the player is at
    pub fn get_player_point() -> Point {
        unsafe { PLAYER_POS }
    }

    /// Sets the current `Point` that the player is at
    pub fn set_player_point(point: Point) {
        unsafe { PLAYER_POS = point; }
    }

    /// Returns the previous position of the player
    pub fn get_last_player_point() -> Point {
        unsafe { LAST_PLAYER_POS }
    }

    /// Sets the previous position of the player
    pub fn set_last_player_point(point: Point) {
        unsafe { LAST_PLAYER_POS = point; }
    }

    /// Receives the keypresses in the game loop
    pub fn wait_for_keypress(&mut self) -> Key {
        let ks = self.rendering_component.wait_for_keypress();
        Game::set_last_keypress(ks);
        return ks;
    }
}