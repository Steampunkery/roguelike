use crate::level::Level;
use crate::util::{Point, Bound};
use crate::rendering::{RenderingComponent, TcodRenderingComponent};

use tcod::input::Key;
use rand_isaac::IsaacRng;
use rand_core::{SeedableRng, RngCore};

/// The y offset of the map from the top
pub const MAP_OFFSET: i32 = 2; // 1 line for messages, one for padding
/// The width of the map display area
pub const MAP_WIDTH: i32 = 80;
/// The height of the map display area
pub const MAP_HEIGHT: i32 = 50;

pub const SHOW_MAP: bool = true;

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
    /// A vector of messages to show to the player
    pub message_queue: Vec<String>,
    /// Where we are in the massages vector
    pub message_cache: Vec<String>,
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
            message_queue: vec!["Welcome to MR: TOM".to_string()],
            message_cache: vec![],
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

    /// Calls the update methods of all objects in the domain of the game. Think player, items, mobs, etc.
    pub fn update(&mut self) {
        'outer: while self.level.current_actor < self.level.entities.len() {
            let mut entity = self.level.entities[self.level.current_actor].take().unwrap();
            let mut action = entity.get_action(&mut self.level);
            self.level.entities[self.level.current_actor] = Some(entity);

            if action.is_some() {
                'inner: loop {
                    let act = action.unwrap();
                    let result = act.perform(&mut self.level);
                    if !result.success { return }
                    if result.alternate.is_none() { break 'inner }
                    action = result.alternate;
                }
                self.level.current_actor += 1;
            } else {
                return
            }

        }

        self.level.current_actor = 0;
    }

    /// Delegates rendering of the map, mobs, and player to the `rendering_component` in the correct order
    pub fn render(&mut self) {
        self.level.map_component.render(&mut self.rendering_component, &self.level.entities[0].as_ref().unwrap());

        for item in self.level.items.values() {
            item.render(&mut self.rendering_component);
        }

        // reverse to render the player last because it's always 0
        for i in self.level.entities.iter().rev() {
            i.as_ref().unwrap().render(&mut self.rendering_component);
        }
    }

    /// Receives the keypresses in the game loop
    pub fn wait_for_keypress(&mut self) -> Key {
        let ks = self.rendering_component.wait_for_keypress();
        self.level.input = Some(ks);
        return ks;
    }

    pub fn game_log(&mut self, message: String) {
        self.message_queue.push(message);
    }
}