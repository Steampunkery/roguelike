use crate::util::Point;
use crate::actor::Actor;
use crate::rendering::RenderingComponent;

use rand::Rng;
use tcod::Color;

use crate::game::MAP_WIDTH;
use crate::game::MAP_HEIGHT;

const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;

const MAX_ROOM_MONSTERS: i32 = 3;

pub type Map = Vec<Vec<Tile>>;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
    pub explored: bool,
    pub color_override: Option<Color>
}

impl Tile {
    pub fn empty() -> Self {
        Tile { blocked: false, block_sight: false, explored: false, color_override: None }
    }

    pub fn wall() -> Self {
        Tile { blocked: true, block_sight: true, explored: false, color_override: None }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect { x1: x, y1: y, x2: x + w, y2: y + h }
    }

    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    pub fn intersects_with(&self, other: &Rect) -> bool {
        // returns true if this rectangle intersects with another one
        (self.x1 <= other.x2) && (self.x2 >= other.x1) &&
            (self.y1 <= other.y2) && (self.y2 >= other.y1)
    }
}

pub trait MapComponent {
    fn get_rooms(&self) -> &Vec<Rect>;
    fn get_map(&self) -> &Map;
    fn get_map_mut(&mut self) -> &mut Map;
    fn get_player_start(&self) -> Point;
    fn contains(&self, x: i32, y: i32) -> bool;
    fn render(&mut self, rendering_component: &mut Box<RenderingComponent>);
    fn is_blocked(&self, x: i32, y: i32) -> bool;
}

pub struct DungeonMapComponent {
    pub rooms: Vec<Rect>,
    pub mobs: Vec<Actor>,
    pub map: Map,
    pub player_start: Point
}

impl MapComponent for DungeonMapComponent {
    fn get_rooms(&self) -> &Vec<Rect> {
        &self.rooms
    }

    fn get_map(&self) -> &Map {
        &self.map
    }

    fn get_map_mut(&mut self) -> &mut Map {
        &mut self.map
    }

    fn get_player_start(&self) -> Point {
        self.player_start
    }

    fn contains(&self, x: i32, y: i32) -> bool {
        (x < MAP_WIDTH && x >= 0)
        && (y < MAP_HEIGHT && y >= 0)
    }

    fn render(&mut self, rendering_component: &mut Box<RenderingComponent>) {
        rendering_component.render_map(&mut self.map);
    }

    fn is_blocked(&self, x: i32, y: i32) -> bool {
        if self.map[x as usize][y as usize].blocked {
            return true;
        }

        self.mobs.iter().any(|mob| {
            (mob.position.x, mob.position.y) == (x, y)
        })
    }
}

impl DungeonMapComponent {
    pub fn new() -> DungeonMapComponent {
        // fill map with "unblocked" tiles
        let mut map = vec![vec![Tile::wall(); (MAP_HEIGHT + 1) as usize]; (MAP_WIDTH + 1) as usize];
        let mut rooms = vec![];
        let mut mobs = Vec::<Actor>::new();
        let mut player_start = Point { x: 0, y: 0 };

        for _ in 0..MAX_ROOMS {
            // random width and height
            let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
            let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
            // random position without going out of the boundaries of the map
            let x = rand::thread_rng().gen_range(0, MAP_WIDTH - w);
            let y = rand::thread_rng().gen_range(0, MAP_HEIGHT - h);

            let new_room = Rect::new(x, y, w, h);

            // run through the other rooms and see if they intersect with this one
            let failed = rooms.iter().any(|other_room| new_room.intersects_with(other_room));

            if !failed {
                // this means there are no intersections, so this room is valid
                Self::create_room(new_room, &mut map);
                let (new_x, new_y) = new_room.center();

                if rooms.is_empty() {
                    // Set the players starting position to the center of the first room
                    player_start.x = new_x;
                    player_start.y = new_y;
                } else {
                    // center coordinates of the previous room
                    let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                    // coin flip
                    if rand::random() {
                        Self::create_h_tunnel(prev_x, new_x, prev_y, &mut map);
                        Self::create_v_tunnel(prev_y, new_y, new_x, &mut map);
                    } else {
                        Self::create_v_tunnel(prev_y, new_y, prev_x, &mut map);
                        Self::create_h_tunnel(prev_x, new_x, new_y, &mut map);
                    }
                }
            }

            // finally, append the new room to the list
            rooms.push(new_room);
        }

        DungeonMapComponent {
            rooms,
            mobs,
            map,
            player_start
        }
    }

    fn create_room(room: Rect, map: &mut Map) {
        for x in (room.x1 + 1)..room.x2 {
            for y in (room.y1 + 1)..room.y2 {
                map[x as usize][y as usize] = Tile::empty();
            }
        }
    }

    fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
        for x in std::cmp::min(x1, x2)..(std::cmp::max(x1, x2) + 1) {
            map[x as usize][y as usize] = Tile::empty();
        }
    }

    fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
        for y in std::cmp::min(y1, y2)..(std::cmp::max(y1, y2) + 1) {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}