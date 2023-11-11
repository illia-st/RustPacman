use chrono::Duration;
use chrono::Utc;

use crate::core::ghost::Ghost;
use crate::core::map::generator::MapGenerator;
use crate::core::map::generator::DefaultMapGenerator;
use crate::core::pacman::Pacman;

use super::game::Game;

pub trait GameGenerator {
    fn generate_game() -> Game;
}

pub struct DefaultGameGenerator {}

impl GameGenerator for DefaultGameGenerator {
    fn generate_game() -> Game {
        let game_map = DefaultMapGenerator::generate_map();
        
        let pacman = Pacman::new(game_map.map_graph.pacman_pos, Duration::milliseconds(1), Utc::now());
        
        let mut ghosts = Vec::new();
        for ghost_pos in &game_map.map_graph.ghosts_pos {
            ghosts.push(Ghost::new(*ghost_pos, game_map.map_graph.pacman_pos, Duration::milliseconds(256), Utc::now()))
        }

        Game::new(
                pacman,
                ghosts,
                game_map,
        )
        // Game::load_from_file("/home/tr3tiakoff/University/RustPacman/res/test_map.txt")
        // Game::load_from_file("/home/tr3tiakoff/University/RustPacman/res/default_map.txt")
        // Game::load_from_file("/home/tr3tiakoff/University/RustPacman/res/template.txt")
    }
}