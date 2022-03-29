use crate::prelude::*;

const NUM_TILES: usize = { SCREEN_WIDTH * SCREEN_HEIGHT } as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>,
}

#[allow(clippy::cast_sign_loss)]
pub fn get_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

#[cfg(debug_assertions)]
fn at_border(x: i32, y: i32, outline: Rect) -> bool {
    let between_x = x >= outline.x1 && x <= outline.x2;
    let between_y = y >= outline.y1 && y <= outline.y2;

    (x == outline.x1 && between_y)
        || (x == outline.x2 && between_y)
        || (y == outline.y1 && between_x)
        || (y == outline.y2 && between_x)
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
            revealed_tiles: vec![false; NUM_TILES],
        }
    }

    pub fn in_bounds(point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        Map::in_bounds(point) && self.tiles[get_idx(point.x, point.y)] == TileType::Floor
    }

    pub fn try_idx(point: Point) -> Option<usize> {
        if Map::in_bounds(point) {
            Some(get_idx(point.x, point.y))
        } else {
            None
        }
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds(destination) {
            if self.can_enter_tile(destination) {
                let idx = self.point2d_to_index(destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Print out the complete map on console for debugging purposes
    #[cfg(debug_assertions)]
    pub fn dump(&self, outline: Option<Rect>) {
        for y in 0..SCREEN_HEIGHT {
            let mut line = String::new();
            for x in 0..SCREEN_WIDTH {
                let idx = get_idx(x, y);
                let mut glyph = match self.tiles.get(idx).unwrap() {
                    TileType::Floor => '.',
                    TileType::Wall => '#',
                };
                if let Some(outl) = outline {
                    if at_border(x, y, outl) {
                        glyph = '*';
                    }
                }
                line.push(glyph);
            }
            println!("{:02} {}", y, line);
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, point: Point) -> bool {
        Map::in_bounds(point)
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0));
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0));
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0));
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0));
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }

    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] != TileType::Floor
    }
}
