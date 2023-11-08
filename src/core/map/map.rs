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
        let graph = MapGraph::loag_graph_from_matrix(&matrix);

        GameMap {
            map_graph: graph,
            map_state_matrix: matrix,
        }
    }
}