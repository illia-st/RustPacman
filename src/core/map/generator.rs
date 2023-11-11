use crate::core::map::graph::graph::MapGraph;
use crate::core::map::matrix::matrix::MapMatrix;

use super::map::GameMap;
use super::matrix::cell::MatrixCell;

use rand::{self, Rng};

pub trait MapGenerator {
    fn generate_map() -> GameMap;
}

pub struct DefaultMapGenerator {}

impl DefaultMapGenerator {
    fn generate_map_matrix(height: usize, width: usize) -> MapMatrix {
        let mut matrix: Vec<Vec<MatrixCell>> = Vec::new();

        for _ in 0..height {
            let mut row: Vec<MatrixCell> = Vec::new();
            for _ in 0..width {
                row.push(MatrixCell::point())
            }
            matrix.push(row)
        }

        let mut random = rand::thread_rng();

        matrix[random.gen_range(0..height)][random.gen_range(0..width)] = MatrixCell::pacman();

        MapMatrix::new(
            height,
            width,
            matrix
        )
    }
}

impl MapGenerator for DefaultMapGenerator {
    fn generate_map() -> GameMap {
        const SCREEN_HEIGHT: usize = 42;
        const SCREEN_WIDTH: usize = 60;

        const MAP_HEIGHT: usize = 33;
        const MAP_WIDTH: usize = 30;

        const TILE_HEIGHT: usize = 10;
        const TILE_WIDTH: usize = 10;

        let matrix = Self::generate_map_matrix(MAP_HEIGHT, MAP_WIDTH);
        let graph = MapGraph::load_graph_from_matrix(&matrix);

        GameMap::new(
            graph,
            matrix,
            SCREEN_HEIGHT,
            SCREEN_WIDTH,
            TILE_HEIGHT,
            TILE_WIDTH,
        )
    }
}