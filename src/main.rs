#![feature(box_syntax)]
use roguelike::game::Game;
use roguelike::actor::Actor;

use tcod::input::KeyCode;

fn main() {
    let mut game = Game::new();
    let mut c = Actor::player(game.window_bounds);
    let mut npcs: Vec<Actor> = vec![];

    game.render(&npcs, &c);
    while !(game.rendering_component.get_root_console().window_closed() || game.exit) {
        // wait for user input
        let keypress = game.wait_for_keypress();

        // update game state
        match keypress.code {
            KeyCode::Escape => game.exit = true,
            _ => {}
        }
        game.update(&mut npcs, &mut c);

        // render
        game.render(&npcs, &c);
    }
}
