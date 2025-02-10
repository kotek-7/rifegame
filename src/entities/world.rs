use crate::entities::cell::Cell;
use crate::values::{
    grid::{Grid, WORLD_HEIGHT, WORLD_WIDTH},
    rule::Rule,
    vec2::Vec2,
};
use std::fmt::Write;

#[derive(Default)]
pub struct World {
    grid: Grid,
    rules: Vec<Box<dyn Rule>>,
}

impl World {
    pub fn new(grid: Grid, rules: Vec<Box<dyn Rule>>) -> World {
        World { grid, rules }
    }
    pub fn update(&mut self) {
        let grid_snapshot = self.grid.clone();
        for y in 0..WORLD_HEIGHT {
            for x in 0..WORLD_WIDTH {
                let position = Vec2::new(x as i32, y as i32);
                let target_cell = grid_snapshot.look(position).copied().unwrap_or(Cell::Dead);
                let total_alive_neighbors = grid_snapshot.count_alive_neighbors(position);
                for rule in &self.rules {
                    self.grid
                        .set(position, rule.apply(target_cell, total_alive_neighbors))
                        .unwrap();
                }
            }
        }
    }
    pub fn draw<T: std::io::Write>(&self, screen: &mut T) {
        let mut visual_grid: [String; WORLD_HEIGHT] = Default::default();
        for y in 0..WORLD_HEIGHT {
            let row = self.grid.0[y]
                .iter()
                .map(|cell| cell.to_string())
                .collect::<Vec<String>>()
                .concat();
            visual_grid[y] = row;
        }
        let mut buf = String::new();
        for (i, row) in visual_grid.iter().enumerate() {
            write!(buf, "{}{}", termion::cursor::Goto(1, i as u16), row).unwrap();
        }
        write!(screen, "{}{}", termion::cursor::Goto(1, 1), buf).unwrap();
    }
}

impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.grid.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        entities::rules,
        values::{grid::Grid, rule::Rule},
    };

    use super::World;

    #[test]
    fn test_update() {
        let grid = Grid::try_from([
            "                ",
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
        ])
        .unwrap();
        let rules: Vec<Box<dyn Rule>> = vec![
            Box::new(rules::Overpopulation),
            Box::new(rules::Survival),
            Box::new(rules::Underpopulation),
            Box::new(rules::Reproduction),
        ];
        let mut world = World::new(grid, rules);
        println!("{}", world);
        world.update();
        println!("{}", world);
    }
}
