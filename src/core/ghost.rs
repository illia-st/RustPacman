use crate::core::GameStatus;
use super::cell::Cell;

#[derive(Default)]
pub struct Ghost {
    // TODO: change to usize
    pub curr_cell: Cell,
    pub pacman_pos: usize,
}

impl Ghost {
    pub fn new(start_cell: Cell, pacman_pos: usize) -> Self {
        Self {
            curr_cell: start_cell,
            pacman_pos,
        }
    }
    pub fn update_state(&mut self, way: &mut [Cell]) -> GameStatus {
        todo!("implement ghost state updating");
    }
}