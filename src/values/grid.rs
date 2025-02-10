use std::fmt::Write;

use crate::entities::cell::Cell;
use crate::errors::{IndexOutOfBoundsError, InvalidFormatError};
use crate::values::vec2::Vec2;

pub const WORLD_WIDTH: usize = 16;
pub const WORLD_HEIGHT: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Grid(pub [[Cell; WORLD_WIDTH]; WORLD_HEIGHT]);

impl Grid {
    pub fn new(grid: [[Cell; WORLD_WIDTH]; WORLD_HEIGHT]) -> Grid {
        Grid(grid)
    }

    pub fn look(&self, position: Vec2) -> Option<&Cell> {
        Some(self.0.get(position.y as usize)?.get(position.x as usize)?)
    }

    pub fn set(&mut self, position: Vec2, cell: Cell) -> Result<(), IndexOutOfBoundsError> {
        let target_cell = self
            .0
            .get_mut(position.y as usize)
            .ok_or(IndexOutOfBoundsError)?
            .get_mut(position.x as usize)
            .ok_or(IndexOutOfBoundsError)?;
        *target_cell = cell;
        Ok(())
    }

    pub fn count_alive_neighbors(&self, center: Vec2) -> u32 {
        self.look(Vec2::new(center.y - 1, center.x))
            .copied()
            .unwrap_or(Cell::Dead) as u32
            + self
                .look(Vec2::new(center.y - 1, center.x + 1))
                .copied()
                .unwrap_or(Cell::Dead) as u32
            + self
                .look(Vec2::new(center.y, center.x + 1))
                .copied()
                .unwrap_or(Cell::Dead) as u32
            + self
                .look(Vec2::new(center.y + 1, center.x + 1))
                .copied()
                .unwrap_or(Cell::Dead) as u32
            + self
                .look(Vec2::new(center.y + 1, center.x))
                .copied()
                .unwrap_or(Cell::Dead) as u32
            + self
                .look(Vec2::new(center.y + 1, center.x - 1))
                .copied()
                .unwrap_or(Cell::Dead) as u32
            + self
                .look(Vec2::new(center.y, center.x - 1))
                .copied()
                .unwrap_or(Cell::Dead) as u32
            + self
                .look(Vec2::new(center.y - 1, center.x - 1))
                .copied()
                .unwrap_or(Cell::Dead) as u32
    }
}

impl TryFrom<[&str; WORLD_HEIGHT]> for Grid {
    type Error = InvalidFormatError;

    fn try_from(value: [&str; WORLD_HEIGHT]) -> Result<Self, Self::Error> {
        let mut temp_grid = [[Cell::default(); WORLD_WIDTH]; WORLD_HEIGHT];
        for (y, string) in value.into_iter().enumerate() {
            if string.len() != WORLD_WIDTH {
                return Err(InvalidFormatError);
            }
            let chars = string.chars();
            for (x, char) in chars.into_iter().enumerate() {
                let cell = match char {
                    ' ' => Cell::Dead,
                    'o' => Cell::Alive,
                    _ => {
                        return Err(InvalidFormatError);
                    }
                };
                temp_grid[y][x] = cell;
            }
        }
        Ok(Grid(temp_grid))
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        for row in self.0 {
            for cell in row {
                write!(buf, "{}", cell).unwrap();
            }
            write!(buf, "{}", "\n").unwrap();
        }
        write!(f, "{}", buf)
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;

    #[test]
    fn test_from() {
        let allay = [
            "                ",
            "                ",
            "                ",
            "                ",
            "                ",
            "        o       ",
            "        o       ",
            "        o       ",
            "                ",
            "                ",
            "                ",
            "                ",
            "                ",
            "                ",
            "                ",
            "                ",
        ];
        let grid: Grid = allay.try_into().unwrap();
        println!("{}", grid);
    }
}