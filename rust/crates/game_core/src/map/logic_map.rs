#[derive(Debug, Copy, Clone)]
pub struct LogicCell {
    pub walkable: bool,
    pub height: i32,
    pub is_step: bool,
}

#[derive(Debug)]
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
}
