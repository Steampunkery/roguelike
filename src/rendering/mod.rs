use tcod::Color;
use tcod::input::Key;
use tcod::map::{Map as FovMap, FovAlgorithm};
use tcod::console::{Root, BackgroundFlag, Console};

use crate::game::Game;
use crate::map::{MapComponent, Map};
use crate::util::{Point, Bound};
use crate::game::{MAP_WIDTH, MAP_HEIGHT};

const TORCH_RADIUS: i32 = 10;
const FOV_LIGHT_WALLS: bool = true;
const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;

pub trait RenderingComponent {
    fn before_render_new_frame(&mut self);
    fn render_map(&mut self, map: &mut Map);
    fn render_tile(&mut self, x: i32, y: i32, symbol: char, explored: &mut bool);
    fn render_object(&mut self, point: Point, symbol: char);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&mut self) -> Key;
    fn get_root_console(&self) -> &Box<Root>;
}

pub struct TcodRenderingComponent {
    pub root_console: Box<Root>,
    pub fov_map: FovMap
}

impl TcodRenderingComponent {
    pub fn new(bounds: Bound, map_component: &Box<MapComponent>) -> Self {
        let console = Root::initializer()
            .size(bounds.max.x + 1, bounds.max.y + 1)
            .title("Tom's Rogue-like")
            .fullscreen(false)
            .init();

        let map = map_component.get_map();

        let mut fov_map = FovMap::new(MAP_WIDTH, MAP_HEIGHT);
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                fov_map.set(x, y,
                            !map[x as usize][y as usize].block_sight,
                            !map[x as usize][y as usize].blocked);
            }
        }

        TcodRenderingComponent {
            root_console: Box::new(console),
            fov_map
        }
    }
}

impl RenderingComponent for TcodRenderingComponent {

    fn before_render_new_frame(&mut self) {
        self.root_console.clear();

        let char_point = Game::get_character_point();
        if char_point != Game::get_last_character_point() {
            self.fov_map.compute_fov(char_point.x, char_point.y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);
        }
    }

    fn render_map(&mut self, map: &mut Map) {
        for x in 0..map.len() - 1 {
            for y in 0..map[x].len() - 1 {
                let wall = map[x][y].block_sight;
                let color_override = map[x][y].color_override;

                if !wall {
                    self.render_tile(x as i32, y as i32, '.', &mut map[x][y].explored);
                } else {
                    self.render_tile(x as i32, y as i32, '+', &mut map[x][y].explored);
                }

                if let Some(color) = color_override {
                    self.root_console.set_char_background(x as i32, y as i32, color, BackgroundFlag::Set);
                    map[x][y].color_override = None;
                }
            }
        }
    }

    fn render_tile(&mut self, x: i32, y: i32, symbol: char, explored: &mut bool) {
        if self.fov_map.is_in_fov(x, y) {
            self.root_console.put_char(x, y, symbol, BackgroundFlag::Set);
            *explored = true;
        } else if *explored {
            self.root_console.put_char(x, y, symbol, BackgroundFlag::Set);
            self.root_console.set_char_foreground(x, y, Color { r: 105, g: 105, b: 105 });
        }
    }

    fn render_object(&mut self, position: Point, symbol: char) {
        if self.fov_map.is_in_fov(position.x, position.y) {
            self.root_console.put_char(position.x, position.y, symbol, BackgroundFlag::Set);
        }
    }

    fn after_render_new_frame(&mut self) {
        self.root_console.flush();
    }

    fn wait_for_keypress(&mut self) -> Key {
        self.root_console.wait_for_keypress(true)
    }

    fn get_root_console(&self) -> &Box<Root> {
        &self.root_console
    }
}

