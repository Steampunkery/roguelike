use crate::util::Point;
use crate::map::MapComponent;

use pathfinding::prelude::astar;

pub const DEBUG_AI: bool = false;

pub fn find_astar_path(map: &mut Box<MapComponent>, start: Point, goal: Point) -> Option<Vec<Point>> {
    let result: Option<(Vec<Point>, u32)> = astar(&start, |p| p.successors(map),
                       |p| p.distance(&goal) / 3,
                       |p| *p == goal);

    result.and_then(|f| Some(f.0))
}