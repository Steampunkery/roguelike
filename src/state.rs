use crate::game::{Game, MAP_WIDTH};
use crate::Exit;
use crate::util::add_punctuation;

use tcod::input::KeyCode;

pub trait State {
    fn maybe_new_state(&mut self) -> Option<Box<dyn State>>;
    fn maybe_exit_game(&self) -> Option<Exit>;
    fn should_exit(&self) -> bool;

    fn enter(&self) {}
    fn exit(&mut self) -> Game;

    fn update(&mut self);
    fn render(&mut self) {
        self.get_game_mut().rendering_component.before_render_new_frame();
        self.get_game_mut().render();
        self.get_game_mut().rendering_component.after_render_new_frame();
    }

    fn get_game(&self) -> &Game;
    fn get_game_mut(&mut self) -> &mut Game;

    fn set_game(&mut self, game: Game);
}

pub struct PlayState {
    game: Option<Game>,
    should_exit: Option<Exit>,
}

pub struct MessageState {
    game: Option<Game>,
    should_exit: Option<Exit>,
}

impl PlayState {
    pub fn new(game: Game) -> PlayState {
        PlayState { game: Some(game), should_exit: None, }
    }
}

impl State for PlayState {
    fn maybe_new_state(&mut self) -> Option<Box<dyn State>> {
        if !self.game.as_ref().unwrap().level.message_queue.is_empty() {
            return Some(box MessageState::new(self.game.take().unwrap()))
        }

        None
    }
    fn maybe_exit_game(&self) -> Option<Exit> { self.should_exit }
    fn should_exit(&self) -> bool { false }

    fn exit(&mut self) -> Game { self.game.take().unwrap() }

    fn update(&mut self) {
        let keypress = self.game.as_mut().unwrap().wait_for_keypress();
        match keypress.code {
            KeyCode::Escape => if keypress.shift {
                self.should_exit = Some(Exit::Save);
                return
            } else {
                self.should_exit = Some(Exit::Die);
                return
            }
            _ => {}
        }

        self.game.as_mut().unwrap().update();
    }

    fn get_game(&self) -> &Game { self.game.as_ref().unwrap() }
    fn get_game_mut(&mut self) -> &mut Game { self.game.as_mut().unwrap() }

    fn set_game(&mut self, game: Game) { self.game = Some(game) }
}

impl MessageState {
    pub fn new(game: Game) -> MessageState { MessageState { game: Some(game), should_exit: None, } }
}

impl State for MessageState {
    fn maybe_new_state(&mut self) -> Option<Box<dyn State>> { None }
    fn maybe_exit_game(&self) -> Option<Exit> { self.should_exit }
    fn should_exit(&self) -> bool { self.game.as_ref().unwrap().level.message_queue.is_empty() }

    fn exit(&mut self) -> Game { self.game.take().unwrap() }

    fn update(&mut self) {}

    fn render(&mut self) {
        self.get_game_mut().rendering_component.before_render_new_frame();
        self.get_game_mut().render();

        if !self.game.as_mut().unwrap().level.message_queue.is_empty() {
            let game = self.game.as_mut().unwrap();

            let mut message = game.level.message_queue.drain(..).map(|s| add_punctuation(s)).collect::<Vec<String>>().join(" ");
            if message.len() > (MAP_WIDTH - 21) as usize {
                let spaces = message.match_indices(" ").collect::<Vec<_>>();
                let m_clone = message.clone();
                let (first, last) = m_clone.split_at(spaces[spaces.len()/2].0);
                message = first.to_string();
                game.level.message_queue.insert(0, last.to_string());
            }
            game.rendering_component.push_message(&message);
            game.level.message_cache.push(message);
            if !game.level.message_queue.is_empty() {
                game.rendering_component.print(&"-- enter for more --".to_string(), MAP_WIDTH - 21, 0);
            }
        }

        self.get_game_mut().rendering_component.after_render_new_frame();

        if !self.game.as_mut().unwrap().level.message_queue.is_empty() {
            let mut keypress;
            loop {
                keypress = self.game.as_mut().unwrap().wait_for_keypress();
                match keypress.code {
                    KeyCode::Enter => break,
                    _ => continue,
                }
            }
        }
    }

    fn get_game(&self) -> &Game { &self.game.as_ref().unwrap() }
    fn get_game_mut(&mut self) -> &mut Game { self.game.as_mut().unwrap() }

    fn set_game(&mut self, game: Game) { self.game = Some(game) }
}