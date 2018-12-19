#![feature(box_syntax)]
use roguelike::game::Game;
use roguelike::actor::Actor;

use rand::Rng;
use tcod::input::KeyCode;

fn main() {
    let mut game = Game::new();
    let mut c = Actor::player();

    for _ in 0..3 {
        // Get a random room
        let room_num = rand::thread_rng().gen_range(0, game.level.map_component.get_rooms().len());
        let room = game.level.map_component.get_rooms()[room_num];

        // Pick random coordinates in that room
        let rand_point = room.rand_point();

        // Spawn a monster there
        game.level.mobs.push(Actor::kobold(rand_point.x, rand_point.y));
    }

    game.render(&c);
    while !(game.rendering_component.get_root_console().window_closed() || game.exit) {
        // wait for user input
        let keypress = game.wait_for_keypress();

        // update game state
        match keypress.code {
            KeyCode::Escape => game.exit = true,
            _ => {}
        }
        game.update(&mut c);

        // render
        game.render(&c);
    }
}
