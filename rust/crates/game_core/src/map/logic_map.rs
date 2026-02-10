#[derive(Debug, Copy, Clone)]
pub struct LogicCell {
    pub walkable: bool,
    pub height: i32,
    pub is_step: bool,
}

#[derive(Debug, Clone)]
pub struct LogicMap {
    map_data: Vec<Option<LogicCell>>,
    width: usize,
    height: usize,
}

impl LogicMap {
    pub fn new(width: usize, height: usize) -> Self {
        LogicMap {
            map_data: vec![None; width * height],
            width,
            height,
        }
    }

    pub fn set_size(&mut self, width: usize, height: usize) {
        self.map_data.resize(width * height, None);
        self.width = width;
        self.height = height;
    }

    pub fn get_data_len(&self) -> usize {
        self.map_data.len()
    }

    pub fn set_cell(&mut self, i: usize, j: usize, cell: Option<LogicCell>) {
        if j * self.width + i < self.map_data.len() {
            self.map_data[j * self.width + i] = cell
        }
    }

    pub fn is_walkable(&self, i: i32, j: i32) -> bool {
        if i < 0 || j < 0 || i >= self.width as i32 || j >= self.height as i32 {
            return false;
        }

        if let Some(cell) = self.map_data[j as usize * self.width + i as usize] {
            cell.walkable
        } else {
            false
        }
    }
}
