use std::ops::Deref;

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
        let game_map = GameMap::load_map_from_file(path);
        
        let pacman = Pacman::new(game_map.map_graph.pacman_pos, Duration::milliseconds(16), Utc::now());
        
        let mut ghosts = Vec::new();
        for ghost_pos in &game_map.map_graph.ghosts_pos {
            ghosts.push(Ghost::new(*ghost_pos, game_map.map_graph.pacman_pos, Duration::milliseconds(32), Utc::now()))
        }

        Self {
            pacman,
            ghosts,
            map: game_map,
        }
    }
}
