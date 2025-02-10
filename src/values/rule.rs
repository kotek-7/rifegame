use crate::entities::cell::Cell;

pub trait Rule {
    fn apply(&self, target_cell: Cell, total_alive_neighbors: u32) -> Cell;
}