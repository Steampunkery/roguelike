#![feature(box_syntax)]
pub mod util;
pub mod game;
pub mod rendering;
pub mod brain;
pub mod actor;
pub mod map;
pub mod ai;
pub mod level;
pub mod item;
pub mod action;
pub mod state;

use game::Game;

use std::fs;
use std::path::PathBuf;
use std::io::{ErrorKind, Error as IOE};

use dirs::home_dir;
use crate::state::{PlayState, State};

#[derive(PartialEq, Copy, Clone)]
pub enum Exit {
    Save,
    Die
}

fn main() {
    let seed = try_load_game();

    let game = Game::new(seed);
    let mut state = box PlayState::new(game) as Box<dyn State>;

    state.render();
    let save = loop {
        if state.get_game().rendering_component.get_root_console().window_closed() {
            break Exit::Die
        }

        // Being the update loop sequence, which contains multiple sub-loops
        state.update();

        state.render();

        if let Some(exit) = state.maybe_exit_game() {
            break exit
        }
    };

    // If the player wants to save, attempt to write game info
    if save == Exit::Save {
        match save_game(state.get_game().seed) {
            Err(e) => eprintln!("Could not save: {}", e.to_string()),
            _ => ()
        }
        return;
    }

    // Else, scrub the save file to prevent game replay
    match fs::File::create([home_dir().unwrap().to_str().unwrap(), ".config", "mrtom", "save.dat"].iter().collect::<PathBuf>()) {
        _ => () // To prevent warnings (they're annoying)
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
