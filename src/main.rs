#![feature(box_syntax)]
use roguelike::game::Game;
use roguelike::actor::Actor;

use rand::Rng;
use tcod::input::KeyCode;

fn main() {
    let mut game = Game::new();
    let mut c = Actor::player(game.window_bounds);
    let mut npcs: Vec<Actor> = vec![];

    for _ in 0..3 {
        let room_num = rand::thread_rng().gen_range(0, game.map_component.get_rooms().len());
        let room = game.map_component.get_rooms().get(room_num).unwrap();

        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);
        npcs.push(Actor::kobold(x, y));
    }

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
