use crate::entities::cell::Cell;
use crate::values::rule::Rule;

pub struct StandardRule;

impl Rule for StandardRule {
    fn apply(&self, target_cell: Cell, total_alive_neighbors: u32) -> Cell {
        match target_cell {
            Cell::Dead => {
                // Reproduction
                if total_alive_neighbors == 3 {
                    return Cell::Alive;
                }
            }
            Cell::Alive => {
                // Underpopulation
                if total_alive_neighbors < 2 {
                    return Cell::Dead;
                }
                // Survival
                if total_alive_neighbors == 2 || total_alive_neighbors == 3 {
                    return Cell::Alive;
                }
                // Overpopulation
                if total_alive_neighbors > 3 {
                    return Cell::Dead;
                }
            }
        }
        target_cell
    }
}