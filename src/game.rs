use crate::level::Level;
use crate::util::{Point, Bound};
use crate::rendering::{RenderingComponent, TcodRenderingComponent};

use tcod::input::Key;
use rand_isaac::IsaacRng;
use rand_core::{SeedableRng, RngCore};

/// The y offset of the map from the top
pub const MAP_OFFSET: i32 = 2; // 1 line for messages, one for padding
/// The width of the map display area
pub const MAP_WIDTH: i32 = 79;
/// The height of the map display area
pub const MAP_HEIGHT: i32 = 49;

pub const SHOW: bool = true;

/// Game struct containing all the information about the current game state
pub struct Game {
    /// Whether the game should exit on the next loop
    pub exit: bool,
    /// The boundaries of the window (the size of the map display area)
    pub window_bounds: Bound,
    /// The component for rendering all the tiles in the game
    pub rendering_component: Box<dyn RenderingComponent + 'static>,
    /// A `Level` struct containing all the information on the current level
    pub level: Level,
    /// The game's RNG
    pub random: IsaacRng,
    /// The game's RNG seed
    pub seed: u64,
}

impl Game {
    /// Creates a new game struct complete with a first level and rendering component
    pub fn new(old_seed: Option<u64>) -> Game {
        let bounds = Bound {
            min: Point { x: 0, y: 0 },
            max: Point { x: MAP_WIDTH, y: MAP_HEIGHT + MAP_OFFSET },
        };

        let (mut isaac, seed) = Self::init_rng(old_seed);

        let level = Self::init_level(&mut isaac);

        let rc = Self::init_renderer(bounds, &level);
        
        Game {
            seed,
            level,
            exit: false,
            window_bounds: bounds,
            rendering_component: rc,
            random: isaac,
        }
    }

    fn init_renderer(bounds: Bound, level: &Level) -> Box<dyn RenderingComponent + 'static> {
        box TcodRenderingComponent::new(bounds, &level.map_component)
    }

    fn init_level(random: &mut IsaacRng) -> Level {
        Level::new(MAP_WIDTH, MAP_HEIGHT, random, None)
    }

    fn init_rng(old_seed: Option<u64>) -> (IsaacRng, u64) {
        if let Some(s) = old_seed {
            (IsaacRng::seed_from_u64(s), s)
        } else {
            let mut rng = rand::thread_rng();
            let new_seed = rng.next_u64();
            (IsaacRng::seed_from_u64(new_seed), new_seed)
        }
    }

    /// Delegates rendering of the map, mobs, and player to the `rendering_component` in the correct order
    pub fn render(&mut self) {
        self.rendering_component.before_render_new_frame();

        self.refresh_messages();

        self.level.render(&mut self.rendering_component);

        self.rendering_component.after_render_new_frame();
    }

    /// Calls the update methods of ALL objects in the domain of the game. Think player, items, mobs, etc.
    pub fn update(&mut self) { self.level.update() }

    pub fn refresh_messages(&mut self) {
        if self.level.messages.len() as i32 > self.level.message_seek + 1 {
            self.level.message_seek += 1;
            self.rendering_component.write_message(&self.level.messages[self.level.message_seek as usize], 0, 0);
        }
    }

    /// Receives the keypresses in the game loop
    pub fn wait_for_keypress(&mut self) -> Key {
        let ks = self.rendering_component.wait_for_keypress();
        self.level.input = Some(ks);
        return ks;
    }
}