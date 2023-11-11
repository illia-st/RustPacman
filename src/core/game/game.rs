use chrono::Utc;
use chrono::Duration;
use tui::widgets::canvas::Map;
use tui::widgets::canvas::Shape;

use crate::core::GameStatus;
use crate::core::map::map::GameMap;
use crate::core::pacman::Pacman;
use crate::core::ghost::Ghost;

#[derive(Debug)]
pub struct Game {
    pub map: GameMap,
    pub pacman: Pacman,
    pub ghosts: Vec<Ghost>,
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
        // how to improve the performance?
        // first of all, need to improve ghosts
        // 1) we know in advance the place of the pacman
        // 2) At the first iteration we can compute full way to pacman using dfs
        // 3) Use computed way to go to the pacman
        // 4) If pacman has changed his position, just add his new position to the precomputed way for the ghosts
        // 5) if a ghost will see a ghost in one of the cells, just recompute the way to the pacman
        if self.pacman.update_state(&mut self.map.map_graph.graph, &mut self.map.map_state_matrix.matrix, &mut self.ghosts) == GameStatus::Finished {
            return GameStatus::Finished;
        }
        for ghost in self.ghosts.iter_mut() {
            if ghost.update_state(&mut self.pacman, &mut self.map.map_graph.graph, &mut self.map.map_state_matrix.matrix) == GameStatus::Finished {
                return GameStatus::Finished;
            }
        }
        GameStatus::Running
    }

    pub fn load_from_file(path: &str) -> Self {
        let game_map = GameMap::load_map_from_file(path);
        
        let pacman = Pacman::new(game_map.map_graph.pacman_pos, Duration::milliseconds(16), Utc::now());
        
        let mut ghosts = Vec::new();
        for ghost_pos in &game_map.map_graph.ghosts_pos {
            ghosts.push(Ghost::new(*ghost_pos, game_map.map_graph.pacman_pos, Duration::milliseconds(256), Utc::now()))
        }

        Self {
            pacman,
            ghosts,
            map: game_map,
        }
    }
}
