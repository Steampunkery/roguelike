use crate::util::Point;
use crate::util::Bound;
use crate::rendering::RenderingComponent;

use rand::Rng;
use rand_isaac::IsaacRng;
use tcod::Color;

/// Maximum height and width of a room.
const ROOM_MAX_SIZE: i32 = 10;
/// Minimum height and width of a room.
const ROOM_MIN_SIZE: i32 = 6;
/// Maximum number of rooms in a level.
const MAX_ROOMS: i32 = 30;

/// Type alias for a `Vec` of `Vec` of `Tile`s.
pub type Map = Vec<Vec<Tile>>;

/// Struct representing one coordinate on the map.
#[derive(Clone, Copy, Debug)]
pub struct Tile {
    /// Whether the tile blocks walking paths.
    pub blocked: bool,
    /// Whether the tile blocks the sight of `Actor`s.
    pub block_sight: bool,
    /// Whether the tile has been explored by the player.
    pub explored: bool,
    /// Debug field for displaying AI paths.
    pub color_override: Option<Color>
}

impl Tile {
    /// Creates a new floor tile.
    pub fn empty() -> Self {
        Tile { blocked: false, block_sight: false, explored: false, color_override: None }
    }

    /// Creates a new wall tile.
    pub fn wall() -> Self {
        Tile { blocked: true, block_sight: true, explored: false, color_override: None }
    }
}

/// Simple struct representing a rectangle.
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Rect {
    /// Convenience method for creating new rectangles.
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect { x1: x, y1: y, x2: x + w, y2: y + h }
    }

    /// Returns the center of the rectangle.
    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    /// Checks if the rectangle collides with another.
    pub fn intersects_with(&self, other: &Rect) -> bool {
        // returns true if this rectangle intersects with another one
        (self.x1 <= other.x2) && (self.x2 >= other.x1) &&
            (self.y1 <= other.y2) && (self.y2 >= other.y1)
    }

    pub fn rand_point(&self, random: &mut IsaacRng) -> Point {
        Point {
            x: random.gen_range(self.x1 + 1, self.x2),
            y: random.gen_range(self.y1 + 1, self.y2),
        }
    }
}

/// This trait holds the requisite methods for a generic map generator
/// so that different level types can easily be generated.
pub trait MapComponent {
    /// Get the position of all rooms in the level.
    fn get_rooms(&self) -> &Vec<Rect>;
    /// Get the underlying `Map` object
    fn get_map(&self) -> &Map;
    /// Mutably borrow the underlying `Map` object.
    fn get_map_mut(&mut self) -> &mut Map;
    /// Gets where the level generator thinks the player should spawn.
    /// This should be a safe place for the player initially.
    fn get_player_start(&self) -> Point;
    /// Whether the current map display area contains a point.
    fn contains(&self, x: i32, y: i32) -> bool;
    /// Render the underlying map object.
    fn render(&mut self, rendering_component: &mut Box<dyn RenderingComponent>);
    /// Whether the supplied position has the `blocked` flag set.
    fn is_blocked(&self, x: i32, y: i32) -> bool;
    /// Gets the bounds (size) of the map
    fn get_bounds(&self) -> Bound;
}

/// Basic struct for simple dungeon levels.
pub struct DungeonMapComponent {
    /// The coordinates for the rooms in the dungeon.
    pub rooms: Vec<Rect>,
    /// The map object representing the level.
    pub map: Map,
    /// Where this particular map generator thinks the player should start.
    pub player_start: Point,
    /// The bounds (size) of the map
    pub bounds: Bound,
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
        (x < self.bounds.max.x && x >= 0)
        && (y < self.bounds.max.y && y >= 0)
    }

    fn render(&mut self, rendering_component: &mut Box<dyn RenderingComponent>) {
        rendering_component.render_map(&mut self.map);
    }

    fn is_blocked(&self, x: i32, y: i32) -> bool {
        if self.contains(x, y) && self.map[x as usize][y as usize].blocked {
            return true;
        }
        return false;
    }

    fn get_bounds(&self) -> Bound {
        self.bounds
    }
}

impl DungeonMapComponent {
    /// Creates a new dungeon map with the default Rust random implementation
    pub fn new(width: i32, height: i32, random: &mut IsaacRng) -> DungeonMapComponent {
        // fill map with "unblocked" tiles
        let mut map = vec![vec![Tile::wall(); height as usize]; width as usize];
        let mut rooms = vec![];

        let zero_point = Point { x: 0, y: 0 };
        let mut player_start = zero_point;
        let bounds = Bound { min: zero_point, max: Point { x: width, y: height } };

        for _ in 0..MAX_ROOMS {
            // random width and height
            let w = random.gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
            let h = random.gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
            // random position without going out of the boundaries of the map
            let x = random.gen_range(0, width - w - 1);
            let y = random.gen_range(0, height - h - 1);

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
                    if random.gen_bool(1.0/2.0) {
                        Self::create_h_tunnel(prev_x, new_x, prev_y, &mut map);
                        Self::create_v_tunnel(prev_y, new_y, new_x, &mut map);
                    } else {
                        Self::create_v_tunnel(prev_y, new_y, prev_x, &mut map);
                        Self::create_h_tunnel(prev_x, new_x, new_y, &mut map);
                    }
                }

                // finally, append the new room to the list
                rooms.push(new_room);
            }
        }

        DungeonMapComponent {
            rooms,
            map,
            player_start,
            bounds
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