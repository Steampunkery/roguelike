use crate::game::Game;
use crate::Exit;
use tcod::input::KeyCode;

pub trait State {
    fn maybe_new_state(&self) -> Option<Box<dyn State>>;
    fn maybe_exit_game(&self) -> Option<Exit>;

    fn enter(&self) {}
    fn exit(self) -> Game;

    fn update(&mut self);
    fn render(&mut self);

    fn get_game(&self) -> &Game;
    fn get_game_mut(&mut self) -> &mut Game;
}

pub struct PlayState {
    game: Game,
    should_exit: Option<Exit>,
}

impl PlayState {
    pub fn new(game: Game) -> PlayState {
        PlayState { game, should_exit: None, }
    }
}

impl State for PlayState {
    fn maybe_new_state(&self) -> Option<Box<dyn State>> {
        None
    }
    fn maybe_exit_game(&self) -> Option<Exit> { self.should_exit }

    fn exit(self) -> Game { self.game }

    fn update(&mut self) {
        let keypress = self.game.wait_for_keypress();
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

        self.game.update();
    }

    fn render(&mut self) {
        self.game.rendering_component.before_render_new_frame();
        self.game.render();
        self.game.rendering_component.after_render_new_frame();
    }

    fn get_game(&self) -> &Game { &self.game }
    fn get_game_mut(&mut self) -> &mut Game { &mut self.game }
}