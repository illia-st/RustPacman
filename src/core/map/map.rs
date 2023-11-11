use tui::style::Color;
use tui::widgets::canvas::Shape;

use super::graph::graph::MapGraph;
use super::matrix::matrix::MapMatrix;

#[derive(Debug)]
pub struct GameMap {
    pub map_graph: MapGraph,
    pub map_state_matrix: MapMatrix,

    pub screen_height: usize,
    pub screen_width: usize,
    pub tile_height: usize,
    pub tile_width: usize,
}

impl GameMap {
    pub fn new(map_graph: MapGraph, map_state_matrix: MapMatrix, screen_height: usize, screen_width: usize, tile_height: usize, tile_width: usize) -> Self {
        GameMap {
            map_graph,
            map_state_matrix,
            screen_height,
            screen_width,
            tile_height,
            tile_width,
        }
    }

    pub fn load_map_from_file(path: &str) -> GameMap {
        const SCREEN_HEIGHT: usize = 35;
        const SCREEN_WIDTH: usize = 80;

        const TILE_HEIGHT: usize = 10;
        const TILE_WIDTH: usize = 10;

        let matrix = MapMatrix::load_matrix_from_file(path);
        let graph = MapGraph::load_graph_from_matrix(&matrix);

        GameMap {
            map_graph: graph,
            map_state_matrix: matrix,
            screen_height: SCREEN_HEIGHT,
            screen_width: SCREEN_WIDTH,
            tile_height: TILE_HEIGHT,
            tile_width: TILE_WIDTH,
        }
    }
}

impl Shape for GameMap {
    fn draw(&self, painter: &mut tui::widgets::canvas::Painter) {

        let top_margin = ((self.screen_height - self.map_state_matrix.height) * self.tile_height) as f64 / 2.;
        let bottom_margin  = ((self.screen_height - self.map_state_matrix.height) * self.tile_height) as f64 / 2.;
        let left_margin = ((self.screen_width - self.map_state_matrix.width) * self.tile_width) as f64 / 2.;
        let right_margin = ((self.screen_width - self.map_state_matrix.width) * self.tile_width) as f64 / 2.;

        for i in 0..self.map_state_matrix.height {
            for j in 0..self.map_state_matrix.width {
                
                let color = match self.map_state_matrix.matrix[i][j].cell_type {
                    crate::core::map::matrix::cell::CellType::Wall => Color::Blue,
                    crate::core::map::matrix::cell::CellType::Pathway => 
                        match self.map_state_matrix.matrix[i][j].cell_presence {
                            crate::core::map::matrix::cell::CellPresence::Pacman => Color::Yellow,
                            crate::core::map::matrix::cell::CellPresence::Ghost => Color::Magenta,
                            crate::core::map::matrix::cell::CellPresence::None => 
                                match self.map_state_matrix.matrix[i][j].cell_modificator {
                                    crate::core::map::matrix::cell::CellModificator::Point => Color::White,
                                    crate::core::map::matrix::cell::CellModificator::Bonus => Color::Cyan,
                                    crate::core::map::matrix::cell::CellModificator::Super => Color::Red,
                                    crate::core::map::matrix::cell::CellModificator::None => Color::Black,
                                },
                        },
                };

                for dx in 0..self.tile_width {
                    for dy in 0..self.tile_height {

                        let x: f64 = left_margin + (j * self.tile_width + dx) as f64;
                        let y: f64 = top_margin + ((self.map_state_matrix.height - i) * self.tile_height - dy) as f64;
                        
                        if let Some((x, y)) = painter.get_point(x, y) {
                            painter.paint(x, y, color)
                        }
                    }
                }
            }
        }
    }
}
