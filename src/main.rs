#![feature(box_syntax)]
use roguelike::game::Game;
use roguelike::actor::Actor;

use std::fs;
use std::path::PathBuf;
use std::io::{ErrorKind, Error as IOE};

use rand::Rng;
use dirs::home_dir;
use tcod::input::KeyCode;

#[derive(PartialEq)]
enum Exit {
    Save,
    Die
}

fn main() {
    let seed = try_load_game();

    let mut game = Game::new(seed);

    // A hack. Fix later
    spawn_monsters(&mut game);

    game.render();
    let save = loop {
        if game.rendering_component.get_root_console().window_closed() {
            break Exit::Die
        }

        // wait for user input
        let keypress = game.wait_for_keypress();

        // update game state
        match keypress.code {
            KeyCode::Escape => if keypress.shift { break Exit::Save } else { break Exit::Die }
            _ => {}
        }

        game.update();

        if !game.did_take_turn {
            continue;
        }

        // render
        game.render();
    };

    // If the player wants to save, attempt to write game info
    if save == Exit::Save {
        match save_game(game.seed) {
            Err(e) => eprintln!("Could not save: {}", e.to_string()),
            _ => ()
        }
        return;
    }

    // Else, scrub the save file to prevent game replay
    match fs::File::create([home_dir().unwrap().to_str().unwrap(), ".config", "mrtom", "save.dat"].iter().collect::<PathBuf>()) {
        _ => () // To prevent warnings (They're annoying)
    }
}

fn spawn_monsters(game: &mut Game) {
    for _ in 0..3 {
        // Get a random room
        let room_num = game.random.gen_range(0, game.level.map_component.get_rooms().len());
        let room = game.level.map_component.get_rooms()[room_num];

        // Pick random coordinates in that room
        let rand_point = room.rand_point(&mut game.random);

        // Spawn a monster there
        game.level.mobs.push(Actor::kobold(rand_point.x, rand_point.y));
    }
}

fn try_load_game() -> Option<u64> {
    let user_home = home_dir()?;
    let seed = fs::read_to_string([user_home.to_str().unwrap(), ".config", "mrtom", "save.dat"].iter().collect::<PathBuf>());

    match seed {
        Ok(s) => {
            let s = s.parse::<u64>();
            if let Ok(t) = s {
                Some(t)
            } else {
                return None;
            }
        }
        Err(_) => None
    }
}

fn save_game(seed: u64) -> std::io::Result<()> {
    let user_home = home_dir().ok_or(IOE::from(ErrorKind::NotFound))?;
    let mut game_dir: PathBuf = [user_home.to_str().unwrap(), ".config", "mrtom"].iter().collect();
    fs::create_dir_all(&game_dir)?;

    game_dir.push("save.dat");
    fs::write(game_dir, seed.to_string())?;
    Ok(())
}
