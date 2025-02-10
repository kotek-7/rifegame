use crate::entities::cell::Cell;
use crate::values::rule::Rule;

pub struct Underpopulation;

impl Rule for Underpopulation {
    fn apply(&self, target_cell: Cell, total_alive_neighbors: u32) -> Cell {
        if let Cell::Alive = target_cell {
            if total_alive_neighbors < 2 {
                return Cell::Dead;
            }
        }
        target_cell
    }
}

pub struct Survival;

impl Rule for Survival {
    fn apply(&self, target_cell: Cell, total_alive_neighbors: u32) -> Cell {
        if let Cell::Alive = target_cell {
            if total_alive_neighbors == 2 || total_alive_neighbors == 3 {
                return Cell::Alive;
            }
        }
        target_cell
    }
}

pub struct Overpopulation;

impl Rule for Overpopulation {
    fn apply(&self, target_cell: Cell, total_alive_neighbors: u32) -> Cell {
        if let Cell::Alive = target_cell {
            if total_alive_neighbors > 3 {
                return Cell::Dead;
            }
        }
        target_cell
    }
}

pub struct Reproduction;

impl Rule for Reproduction {
    fn apply(&self, target_cell: Cell, total_alive_neighbors: u32) -> Cell {
        if let Cell::Dead = target_cell {
            if total_alive_neighbors == 3 {
                return Cell::Alive;
            }
        }
        target_cell
    }
}