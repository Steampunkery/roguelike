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

    let mut states: Vec<Box<dyn State>> = vec![];

    let game = Game::new(seed);
    let play_state = Box::new(PlayState::new(game));

    states.push(play_state);

    states[0].render();
    let save = loop {
        let mut i = states.len() - 1;

        if states[i].get_game().rendering_component.get_root_console().window_closed() {
            break Exit::Die
        }

        // Update state
        if states[i].should_exit() {
            let game = states.pop().unwrap().exit();
            i -= 1;
            states[i].set_game(game);
        } else {
            if let Some(new_state) = states[i].maybe_new_state() {
                states.push(new_state);
                i += 1;
            }
        }

        states[i].update();
        states[i].render();

        // Check if the game should exit
        if let Some(exit) = states[i].maybe_exit_game() {
            break exit
        }
    };

    // If the player wants to save, attempt to write game info
    if save == Exit::Save {
        match save_game(states.last().unwrap().get_game().seed) {
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
