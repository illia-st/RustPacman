use super::graph::graph::MapGraph;
use super::matrix::matrix::MapMatrix;

pub struct GameMap {
    pub map_graph: MapGraph,
    pub map_state_matrix: MapMatrix,
}