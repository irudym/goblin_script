pub enum TileType {
    Ground,
    Tree,
    Bush,
    Rock,
    Water,
}

pub struct GameMap {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<TileType>,
}

impl GameMap {
    pub fn new(width: usize, height: usize) -> Self {
        GameMap {
            width,
            height,
            tiles: Vec::with_capacity(width * height),
        }
    }
    pub fn is_blocked(&self, i: i32, j: i32) -> bool {
        let offset = j as usize * self.width + i as usize;
        match self.tiles[offset] {
            TileType::Ground => false,
            _ => true,
        }
    }

    pub fn set_tile(&mut self, i: i32, j: i32, tile_type: TileType) {
        let offset = j as usize * self.width + i as usize;
        self.tiles[offset] = tile_type;
    }
}
