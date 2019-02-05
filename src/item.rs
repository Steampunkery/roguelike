use crate::map::Rect;
use crate::util::Point;
use crate::rendering::RenderingComponent;

use hashbrown::HashMap;

use rand_isaac::IsaacRng;

// These constants shamelessly copy/pasted from NetHack source code
const ILLOBJ_SYM: char = ']'; /* also used for mimics */
const WEAPON_SYM: char = ')';
const ARMOR_SYM: char = '[';
const RING_SYM: char = '=';
const AMULET_SYM: char = '"';
const TOOL_SYM: char = '(';
const FOOD_SYM: char = '%';
const POTION_SYM: char = '!';
const SCROLL_SYM: char = '?';
const SPBOOK_SYM: char = '+';
const WAND_SYM: char = '/';
const COIN_SYM: char = '$';
const GEM_SYM: char = '*';
const ROCK_SYM: char = '`';
const BALL_SYM: char = '0';
const CHAIN_SYM: char = '_';
const VENOM_SYM: char = '.';

pub type ItemsMap = HashMap<Point, Item>;

// Also shamelessly copy/pasted
pub enum ItemType {
    ILLOBJ,
    WEAPON,
    ARMOR,
    RING,
    AMULET,
    TOOL,
    FOOD,
    POTION,
    SCROLL,
    SPBOOK,
    WAND,
    COIN,
    GEM,
    ROCK,
    BALL,
    CHAIN,
    VENOM,
}

/// Struct representing a single item on the map
pub struct Item {
    pub position: Point,
    pub item_type: ItemType,
    pub name: String,
}

impl Item {
    pub fn position(&self) -> Point {
        self.position
    }

    /// Basic render method. See [render_object](../rendering/trait.RenderingComponent.html#tymethod.render_object)
    pub fn render(&self, rendering_component: &mut Box<dyn RenderingComponent>) {
        rendering_component.render_object(self.position(), symbol_for_type(&self.item_type));
    }
}

/// Given a vector of rooms, return a hashmap of points containing items
pub fn place_items(rooms: &Vec<Rect>, random: &mut IsaacRng) -> ItemsMap {
    let mut items = ItemsMap::new();
    let room = rooms[0];
    let rand_point = room.rand_point(random);
    items.insert(rand_point, Item {
        position: rand_point,
        item_type: ItemType::WEAPON,
        name: "Sword".to_string(),
    });

    items
}

pub fn symbol_for_type(item_type: &ItemType) -> char {
    use self::ItemType::*;
    match item_type {
        WEAPON => WEAPON_SYM,
        ARMOR => ARMOR_SYM,
        RING => RING_SYM,
        AMULET => AMULET_SYM,
        TOOL => TOOL_SYM,
        FOOD => FOOD_SYM,
        POTION => POTION_SYM,
        SCROLL => SCROLL_SYM,
        SPBOOK => SPBOOK_SYM,
        WAND => WAND_SYM,
        COIN => COIN_SYM,
        GEM => GEM_SYM,
        ROCK => ROCK_SYM,
        BALL => BALL_SYM,
        CHAIN => CHAIN_SYM,
        VENOM => VENOM_SYM,
        _ => ILLOBJ_SYM
    }
}