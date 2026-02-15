use platform::log_debug;
use platform::logger::LogType;

use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;

use platform::types::{Vector2D, Vector2Di};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct LogicCell {
    pub walkable: bool,
    pub height: i32,
    pub is_step: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicMap {
    map_data: Vec<Option<LogicCell>>,
    pub width: usize,
    pub height: usize,
    cell_size: f32,
}

impl LogicMap {
    pub fn new(width: usize, height: usize) -> Self {
        LogicMap {
            map_data: vec![None; width * height],
            width,
            height,
            cell_size: 64.0,
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

    fn get_cell(&self, position: Vector2Di) -> Option<LogicCell> {
        let offset = match self.get_data_offset(position.x, position.y) {
            Some(val) => val,
            None => return None,
        };

        return self.map_data[offset];
    }

    pub fn is_walkable(&self, i: i32, j: i32) -> bool {
        let offset = match self.get_data_offset(i, j) {
            Some(val) => val,
            None => return false,
        };

        if let Some(cell) = self.map_data[offset] {
            cell.walkable
        } else {
            false
        }
    }

    // Returns difference between cell levels with coordinate cell1 and cell2
    pub fn cmp_levels(&self, cell1: Vector2Di, cell2: Vector2Di) -> i32 {
        self.get_cell_level(cell2.x, cell2.y) - self.get_cell_level(cell1.x, cell2.y)
    }

    // Checks if it's possible to move from from_position to to_position
    pub fn is_walkable_from(&self, from_position: Vector2Di, to_position: Vector2Di) -> bool {
        if !self.is_walkable(to_position.x, to_position.y) {
            return false;
        }

        if self.is_step(from_position) && self.is_step(to_position) {
            return true;
        }

        if self.cmp_levels(from_position, to_position) != 0 {
            return false;
        }

        true
    }

    pub fn is_step(&self, coordinate: Vector2Di) -> bool {
        let offset = match self.get_data_offset(coordinate.x, coordinate.y) {
            Some(val) => val,
            None => return false,
        };

        if let Some(cell) = self.map_data[offset] {
            cell.is_step
        } else {
            false
        }
    }

    #[inline]
    fn get_data_offset(&self, i: i32, j: i32) -> Option<usize> {
        if i < 0 || j < 0 || i >= self.width as i32 || j >= self.height as i32 {
            return None;
        }
        Some(j as usize * self.width + i as usize)
    }

    pub fn get_cell_level(&self, i: i32, j: i32) -> i32 {
        let offset = match self.get_data_offset(i, j) {
            Some(val) => val,
            None => return 0,
        };
        if let Some(cell) = self.map_data[offset] {
            cell.height
        } else {
            0
        }
    }

    pub fn get_cell_position(&self, position: Vector2D) -> Vector2Di {
        let i = (position.x / self.cell_size) as i32;
        let j = (position.y / self.cell_size) as i32;
        Vector2Di { x: i, y: j }
    }

    fn get_cell_coordinates(&self, position: Vector2Di) -> Vector2D {
        Vector2D {
            x: position.x as f32 * self.cell_size,
            y: position.y as f32 * self.cell_size,
        }
    }

    pub fn get_step_y_offset(&self, position: Vector2D) -> f32 {
        // get cell coordinates
        let cell_position = self.get_cell_position(position);

        // convert cell coordinates to screen coordinates
        let cell_coordinates = self.get_cell_coordinates(cell_position);

        match self.get_cell(Vector2Di {
            x: cell_position.x,
            y: cell_position.y + 1,
        }) {
            Some(cell) => {
                if cell.is_step {
                    //this is upper part, calculate the offset for the upper part of steps
                    return position.y - cell_coordinates.y - self.cell_size - self.cell_size / 2.0
                        + position.x
                        - cell_coordinates.x;
                }
            }
            None => (),
        };

        // offset for the lower part of the steps
        position.y - (cell_coordinates.y + self.cell_size / 2.0 - position.x + cell_coordinates.x)
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(filename)?;

        // Configure pretty printing (indentation, etc.)
        let config = PrettyConfig::default();
        let ron_string = to_string_pretty(self, config)?;

        file.write_all(ron_string.as_bytes())?;

        Ok(())
    }

    pub fn load_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(filename)?;

        let map: LogicMap = ron::from_str(&content)?;

        Ok(map)
    }
}
