extern crate tcod;

use self::tcod::console::{Root, BackgroundFlag, Console};
use self::tcod::input::Key;

use util::{Point, Bound};

pub trait RenderingComponent {
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, point: Point, symbol: char);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&mut self) -> Key;
    fn get_root_console(&self) -> &Box<Root>;
}

pub struct TcodRenderingComponent {
    pub root_console: Box<Root>
}

impl TcodRenderingComponent {
    pub fn new(bounds: Bound) -> Self {
        let console = Root::initializer()
            .size(bounds.max.x + 1, bounds.max.y + 1)
            .title("Tom's Rogue-like")
            .fullscreen(false)
            .init();

        TcodRenderingComponent {
            root_console: Box::new(console)
        }
    }
}

impl RenderingComponent for TcodRenderingComponent {

    fn before_render_new_frame(&mut self) {
        self.root_console.clear();
    }

    fn render_object(&mut self, position: Point, symbol: char) {
        self.root_console.put_char(position.x, position.y, symbol, BackgroundFlag::Set);
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

