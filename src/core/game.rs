use crate::core::GameStatus;
use super::cell::Cell;
use super::pacman::Pacman;
use super::ghost::Ghost;

#[derive(Default)]
pub struct Game {
    pub pacman: Pacman,
    pub ghosts: Vec<Ghost>,
    pub way: Vec<Cell>,
}

impl Game {
    pub fn new(pacman: Pacman, ghosts: Vec<Ghost>, way: Vec<Cell>) -> Self {
        Self {
            pacman,
            ghosts,
            way,
        }
    }
    pub fn update_state(&mut self) -> GameStatus {
        // TODO: probably there is a sense to save who has won if we return GameStatus::Finished
        if self.pacman.update_state() == GameStatus::Finished {
            return GameStatus::Finished;
        }
        for ghost in self.ghosts.iter_mut() {
            if ghost.update_state() == GameStatus::Finished {
                return GameStatus::Finished;
            }
        }
        GameStatus::Running
    }
}

