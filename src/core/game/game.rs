use crate::core::GameStatus;
use crate::core::map::map::GameMap;
use crate::core::pacman::Pacman;
use crate::core::ghost::Ghost;

pub struct Game {
    pub pacman: Pacman,
    pub ghosts: Vec<Ghost>,
    pub map: GameMap,
}

impl Game {
    pub fn new(pacman: Pacman, ghosts: Vec<Ghost>, map: GameMap) -> Self {
        Self {
            pacman,
            ghosts,
            map,
        }
    }
    pub fn update_state(&mut self) -> GameStatus {
        // TODO: probably there is a sense to save who has won if we return GameStatus::Finished
        if self.pacman.update_state(&mut self.map.map_graph.graph) == GameStatus::Finished {
            return GameStatus::Finished;
        }
        for ghost in self.ghosts.iter_mut() {
            ghost.pacman_pos = self.pacman.curr_cell;
            if ghost.update_state(&mut self.map.map_graph.graph, self.pacman.curr_cell) == GameStatus::Finished {
                return GameStatus::Finished;
            }
        }
        GameStatus::Running
    }

    pub fn load_from_file(path: &str) -> Self {
        Self {
            pacman: todo!(),
            ghosts: todo!(),
            map: todo!(),
        }
    }
}

