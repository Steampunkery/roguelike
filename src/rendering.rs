use crate::map::{MapComponent, Map};
use crate::util::{Point, Bound};
use crate::game::{MAP_WIDTH, MAP_HEIGHT, MAP_OFFSET, SHOW_MAP};
use crate::actor::Actor;

use tcod::Color;
use tcod::input::Key;
use tcod::map::{Map as FovMap, FovAlgorithm};
use tcod::console::{Root, BackgroundFlag, Console};
use crate::actor::Entity;

/// The distance of the character's FOV
const PLAYER_FOV: i32 = 10;
/// Whether the walls should be lit by the FOV algorithm (purely aesthetic)
const FOV_LIGHT_WALLS: bool = true;
/// The algorithm type for calculating FOV
const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;

/// This trait represents the requisite functions for an arbitrary rendering component,
/// such that any rendering component may be dropped into the game
pub trait RenderingComponent {
    /// Hook method to be executed before each frame
    fn before_render_new_frame(&mut self);
    /// Renders every explored tile in a `Map`
    fn render_map(&mut self, map: &mut Map, player: &Entity);
    /// Renders a specific explored tile
    fn render_tile(&mut self, x: i32, y: i32, symbol: char, explored: &mut bool);
    /// Renders a single object
    fn render_object(&mut self, point: Point, symbol: char);
    /// Writes a game message
    fn push_message(&mut self, message: &String);
    /// Writes a game message in color
    fn push_message_color(&mut self, message: &String, color: Color);
    /// Prints a message to some point
    fn print(&mut self, message: &String, x: i32, y: i32);
    /// Hook method to be executed after each frame is done being rendered completely
    fn after_render_new_frame(&mut self);
    /// Wait for keypresses in the console
    fn wait_for_keypress(&mut self) -> Key;
    /// Returns the console object used for rendering
    fn get_root_console(&self) -> &Root;
}

/// The basic text rendering component which is used by default
pub struct TcodRenderingComponent {
    /// The console that is flushed to the screen every tick
    pub console: Root,
    /// The map corresponding to the character's FOV
    pub fov_map: FovMap,
    prev_message: (String, i32),
    new_message: bool,
}

impl TcodRenderingComponent {
    /// Create a basic new rendering component.
    /// The `MapComponent` is needed for the initial calculation of the character's FOV
    pub fn new(bounds: Bound, map_component: &Box<dyn MapComponent>) -> Self {
        let console = Root::initializer()
            .size(bounds.max.x - 1, bounds.max.y - 1)
            .title("Monochrome Rogue-like: The Original Masterpiece")
            .fullscreen(false)
            .init();

        let map = map_component.get_map();

        // Build FOV map
        let mut fov_map = FovMap::new(MAP_WIDTH, MAP_HEIGHT);
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                fov_map.set(x, y,
                            !map[x as usize][y as usize].block_sight,
                            !map[x as usize][y as usize].blocked);
            }
        }

        TcodRenderingComponent {
            console,
            fov_map,
            prev_message: (String::new(), 0),
            new_message: false,
        }
    }
}

impl RenderingComponent for TcodRenderingComponent {
    fn before_render_new_frame(&mut self) {
        self.console.clear();
        self.new_message = false;
    }

    fn render_map(&mut self, map: &mut Map, player: &Entity) {
        // Recompute the FOV before we render the map
        let player_pos = player.get_position();
        if player_pos != player.get_last_position() {
            self.fov_map.compute_fov(player_pos.x, player_pos.y, PLAYER_FOV, FOV_LIGHT_WALLS, FOV_ALGO);
        }

        for x in 0..map.len() {
            for y in 0..map[x].len() {
                let wall = map[x][y].block_sight;
                let color_override = map[x][y].color_override;

                if !wall {
                    self.render_tile(x as i32, y as i32, '.', &mut map[x][y].explored);
                } else {
                    self.render_tile(x as i32, y as i32, '+', &mut map[x][y].explored);
                }

                if let Some(color) = color_override {
                    self.console.set_char_background(x as i32, y as i32 + MAP_OFFSET, color, BackgroundFlag::Set);
                    map[x][y].color_override = None;
                }
            }
        }
    }

    fn render_tile(&mut self, x: i32, y: i32, symbol: char, explored: &mut bool) {
        if self.fov_map.is_in_fov(x, y) || SHOW_MAP {
            self.console.put_char(x, y + MAP_OFFSET, symbol, BackgroundFlag::Set);
            *explored = true;
        } else if *explored {
            self.console.put_char(x, y + MAP_OFFSET, symbol, BackgroundFlag::Set);
            self.console.set_char_foreground(x, y + MAP_OFFSET, Color { r: 105, g: 105, b: 105 });
        }
    }

    fn render_object(&mut self, position: Point, symbol: char) {
        if self.fov_map.is_in_fov(position.x, position.y) || SHOW_MAP {
            self.console.put_char(position.x, position.y + MAP_OFFSET, symbol, BackgroundFlag::Set);
        }
    }

    fn push_message(&mut self, message: &String) {
        if *message != self.prev_message.0 {
            self.console.print(0, 0, message);
            self.prev_message = (message.clone(), 1);
        } else {
            self.push_message_color(&(self.prev_message.0.clone()), Color {r: 105, g: 105, b: 105});
            self.prev_message.1 += 1;
            if self.prev_message.1 > 1 {
                self.console.print((self.prev_message.0).len() as i32 + 1, 0, format!("(x{})", self.prev_message.1));
            }
        }
        self.new_message = true;
    }

    fn push_message_color(&mut self, message: &String, color: Color) {
        for (i, s) in message.chars().enumerate() {
            self.console.put_char(i as i32, 0, s, BackgroundFlag::None);
            self.console.set_char_foreground(i as i32, 0, color);
        }
    }

    fn print(&mut self, message: &String, x: i32, y: i32) {
        self.console.print(x, y, message);
    }

    fn after_render_new_frame(&mut self) {
        if !self.new_message {
            self.push_message_color(&self.prev_message.0.clone(), Color { r: 105, g: 105, b: 105 });
        }
        self.console.flush();
    }

    fn wait_for_keypress(&mut self) -> Key {
        self.console.wait_for_keypress(true)
    }

    fn get_root_console(&self) -> &Root {
        &self.console
    }
}

