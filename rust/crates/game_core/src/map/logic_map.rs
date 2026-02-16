use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;

use platform::types::{Vector2D, Vector2Di};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum StepType {
    None,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct LogicCell {
    pub walkable: bool,
    pub height: i32,
    pub step_type: StepType,
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
        self.get_cell_level(cell2.x, cell2.y) - self.get_cell_level(cell1.x, cell1.y)
    }

    // Checks if it's possible to move from from_position to to_position
    pub fn is_walkable_from(&self, from_position: Vector2Di, to_position: Vector2Di) -> bool {
        if !self.is_walkable(to_position.x, to_position.y) {
            return false;
        }

        if self.get_step_type(from_position) != StepType::None
            && self.get_step_type(to_position) != StepType::None
        {
            return true;
        }

        if self.cmp_levels(from_position, to_position) != 0 {
            return false;
        }

        true
    }

    pub fn get_step_type(&self, coordinate: Vector2Di) -> StepType {
        let offset = match self.get_data_offset(coordinate.x, coordinate.y) {
            Some(val) => val,
            None => return StepType::None,
        };

        if let Some(cell) = self.map_data[offset] {
            cell.step_type
        } else {
            StepType::None
        }
    }

    /* *
        /// Upper step: this step cell has another step cell below it (y+1)
        pub fn is_upper_step(&self, coordinate: Vector2Di) -> bool {
            if !self.is_step(coordinate) {
                return false;
            }
            self.is_step(Vector2Di {
                x: coordinate.x,
                y: coordinate.y + 1,
            })
        }

        /// Lower step: this step cell has another step cell above it (y-1)
        pub fn is_lower_step(&self, coordinate: Vector2Di) -> bool {
            if !self.is_step(coordinate) {
                return false;
            }
            self.is_step(Vector2Di {
                x: coordinate.x,
                y: coordinate.y - 1,
            })
        }
    */

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

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a 6×3 map:
    /// ```text
    /// Row 0: flat(h=0) flat(h=0) step(h=0) step(h=1) flat(h=1) flat(h=1)
    /// Row 1: flat(h=0) flat(h=0) step(h=0) step(h=1) flat(h=1) flat(h=1)
    /// Row 2: flat(h=0) flat(h=0) flat(h=0) flat(h=0) flat(h=1) flat(h=1)
    /// ```
    fn make_step_map() -> LogicMap {
        let mut map = LogicMap::new(6, 3);
        for j in 0..3 {
            for i in 0..6 {
                let (height, step_type) = match (i, j) {
                    (2, 0) | (2, 1) => (0, StepType::Left),
                    (3, 0) | (3, 1) => (1, StepType::Left),
                    (4, _) | (5, _) => (1, StepType::None),
                    _ => (0, StepType::None),
                };
                map.set_cell(
                    i,
                    j,
                    Some(LogicCell {
                        walkable: true,
                        height,
                        step_type: step_type,
                    }),
                );
            }
        }
        map
    }

    #[test]
    fn test_non_step_returns_false() {
        let map = make_step_map();
        let flat = Vector2Di::new(0, 0);
        assert!(map.get_step_type(flat) == StepType::None);
    }
}
