use std::collections::HashMap;
use crate::core::map::matrix::cell::CellType;
use crate::core::map::matrix::cell::CellPresence;
use crate::core::map::matrix::cell::CellModificator;
use crate::core::map::matrix::matrix::MapMatrix;

use super::cell::GraphCell;

#[derive(Default, Debug)]
pub struct MapGraph {
    pub graph: Vec<GraphCell>
}

impl MapGraph {
    pub fn loag_graph_from_matrix(matrix: &MapMatrix) -> MapGraph {
        let mut graph: Vec<GraphCell> = Vec::new();

        let mut positions: HashMap<(usize, usize), usize> = HashMap::new();
        let mut index: usize = 0;

        for i in 0..matrix.height {
            for j in 0..matrix.width {
                let cell = matrix.matrix[i][j].clone();
                if cell.cell_type != CellType::Pathway {
                    continue;
                }
                graph.push(GraphCell {
                    x: i,
                    y: j,
                    next_cells: Vec::new(),
                    ghost_presence: cell.cell_presence == CellPresence::Ghost,
                    point_presence: cell.cell_modificator == CellModificator::Point,
                    pacman_presence: cell.cell_presence == CellPresence::Pacman,
                });
                positions.insert((i, j), index);
                index += 1;
            }
        }

        for cell in &mut graph {
            let mut neighbors = Vec::new();

            if cell.x.checked_sub(1).is_some() {
                if matrix.matrix[cell.x - 1][cell.y].cell_type == CellType::Pathway {
                    neighbors.push(positions[&(cell.x - 1, cell.y)]);
                }
            }
            if cell.y.checked_sub(1).is_some() {
                if matrix.matrix[cell.x][cell.y - 1].cell_type == CellType::Pathway {
                    neighbors.push(positions[&(cell.x, cell.y - 1)]);
                }
            }

            if cell.x + 1 <= matrix.height - 1 {
                if matrix.matrix[cell.x + 1][cell.y].cell_type == CellType::Pathway {
                    neighbors.push(positions[&(cell.x + 1, cell.y)]);
                }
            }
            if cell.y + 1 <= matrix.width - 1 {
                if matrix.matrix[cell.x][cell.y + 1].cell_type == CellType::Pathway {
                    neighbors.push(positions[&(cell.x, cell.y + 1)]);
                }
            }
            
            cell.next_cells = neighbors;
        }

        MapGraph {
            graph
        }
    }
}