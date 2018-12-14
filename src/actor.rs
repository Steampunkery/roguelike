use crate::util::{Point, Bound};
use crate::game::Game;
use crate::rendering::RenderingComponent;
use crate::map::MapComponent;
use crate::movement::{AggroMovementComponent, RandomMovementComponent, TcodUserMovementComponent, MovementComponent};

pub struct Actor {
    pub position: Point,
    pub health: i32,
    pub display_char: char,
    pub movement_component: Box<MovementComponent + 'static>,
}

impl Actor {
    pub fn new(x: i32, y: i32, health: i32, dc: char, mc: Box<MovementComponent>) -> Actor {
        Actor { position: Point { x, y }, health, display_char: dc, movement_component: mc }
    }

    pub fn update(&mut self, map_component: &mut Box<MapComponent>) {
        if let Some(position) = self.movement_component.update(self.position, map_component) {
            self.position = position;
        }
    }

    pub fn render(&self, rendering_component: &mut Box<RenderingComponent>) {
        rendering_component.render_object(self.position, self.display_char);
    }

    pub fn get_health(&self) -> i32 {
        self.health
    }

    pub fn dog(x: i32, y: i32, bound: Bound) -> Actor {
        let mc: Box<RandomMovementComponent> = box RandomMovementComponent::new(bound);
        Actor::new(x, y, 10, 'd', mc)
    }

    pub fn cat(x: i32, y: i32, bound: Bound) -> Actor {
        let mc: Box<RandomMovementComponent> = box RandomMovementComponent::new(bound);
        Actor::new(x, y, 5, 'c', mc)
    }

    pub fn player(bound: Bound) -> Actor {
        let point = Game::get_character_point();
        let mc: Box<TcodUserMovementComponent> = box TcodUserMovementComponent::new();
        Actor::new(point.x, point.y, 15, '@', mc)
    }

    pub fn kobold(x: i32, y: i32) -> Actor {
        let mc: Box<AggroMovementComponent> = box AggroMovementComponent::new();
        Actor::new(x, y, 12, 'k', mc)
    }
}
