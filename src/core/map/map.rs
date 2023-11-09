use tui::style::Color;
use tui::widgets::canvas::Shape;

use super::graph;
use super::graph::graph::MapGraph;
use super::matrix::matrix::MapMatrix;

#[derive(Debug)]
pub struct GameMap {
    pub map_graph: MapGraph,
    pub map_state_matrix: MapMatrix,
}

impl GameMap {
    pub fn load_map_from_file(path: &str) -> GameMap {
        let matrix = MapMatrix::load_matrix_from_file(path);
        let graph = MapGraph::load_graph_from_matrix(&matrix);

        GameMap {
            map_graph: graph,
            map_state_matrix: matrix,
        }
    }
}

impl Shape for GameMap {
    fn draw(&self, painter: &mut tui::widgets::canvas::Painter) {
        for i in 0..self.map_state_matrix.height {
            for j in 0..self.map_state_matrix.width {
                if let Some((x, y)) = painter.get_point(j as f64, (self.map_state_matrix.height - i - 1) as f64) {
                    painter.paint(
                        x,
                        y,
                    match self.map_state_matrix.matrix[i][j].cell_type {
                            crate::core::map::matrix::cell::CellType::Wall => Color::Blue,
                            crate::core::map::matrix::cell::CellType::Pathway => 
                                match self.map_state_matrix.matrix[i][j].cell_presence {
                                    crate::core::map::matrix::cell::CellPresence::Pacman => Color::Yellow,
                                    crate::core::map::matrix::cell::CellPresence::Ghost => Color::Magenta,
                                    crate::core::map::matrix::cell::CellPresence::None => 
                                        match self.map_state_matrix.matrix[i][j].cell_modificator {
                                            crate::core::map::matrix::cell::CellModificator::Point => Color::LightYellow,
                                            crate::core::map::matrix::cell::CellModificator::Bonus => Color::Cyan,
                                            crate::core::map::matrix::cell::CellModificator::Super => Color::Red,
                                            crate::core::map::matrix::cell::CellModificator::None => Color::Gray,
                                        },
                                },
                        }
                    )
                }
            }
        }
    }
}
