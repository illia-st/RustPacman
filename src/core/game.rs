use crate::core::GameStatus;
use super::cell::Cell;
use super::pacman::Pacman;
use super::ghost::Ghost;

const FIELD_WIDTH: usize = 100;
const FIELD_HEIGHT: usize = 100;

#[derive(Default)]
pub struct Game {
    pub pacman: Pacman,
    pub ghosts: Vec<Ghost>,
    pub way: Vec<Cell>,
    pub cord_to_cell: Vec<Vec<Option<usize>>>,
}

impl Game {
    pub fn new(pacman: Pacman, ghosts: Vec<Ghost>, way: Vec<Cell>) -> Self {
        let mut cord_to_cell = vec![vec![None; FIELD_WIDTH]; FIELD_HEIGHT];
        for (index, cell) in way.iter().enumerate() {
            let _ = cord_to_cell.get_mut(cell.x).unwrap().get_mut(cell.y).unwrap().insert(index);
        }
        Self {
            pacman,
            ghosts,
            way,
            cord_to_cell,
        }
    }
    pub fn update_state(&mut self) -> GameStatus {
        // TODO: probably there is a sense to save who has won if we return GameStatus::Finished
        if self.pacman.update_state(&mut self.way) == GameStatus::Finished {
            return GameStatus::Finished;
        }
        for ghost in self.ghosts.iter_mut() {
            ghost.pacman_pos = self.pacman.curr_cell;
            if ghost.update_state(&mut self.way, self.pacman.curr_cell) == GameStatus::Finished {
                return GameStatus::Finished;
            }
        }
        GameStatus::Running
    }
}

