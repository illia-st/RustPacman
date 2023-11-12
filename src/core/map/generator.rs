use crate::core::map::graph::graph::MapGraph;
use crate::core::map::matrix;
use crate::core::map::matrix::matrix::MapMatrix;

use super::matrix::cell::{CellType, MatrixCell, CellModificator};
use super::{map::GameMap, matrix::cell::CellPresence};

use rand;
use rand::Rng;

pub trait MapGenerator {
    fn generate_map() -> GameMap;
}

pub struct DefaultMapGenerator {}

impl DefaultMapGenerator {
    fn generate_map_matrix_from_template(mut path_len: usize) -> Result<MapMatrix, ()> {
        let mut matrix = MapMatrix::load_matrix_from_file("res/template.txt");
        let mut path: Vec<(usize, usize)> = Vec::new();
        let mut whole_path: Vec<(usize, usize)> = Vec::new();

        for i in 0..matrix.height {
            for j in 0..matrix.width {
                if matrix.matrix[i][j].cell_modificator == CellModificator::Point {
                    path.push((j, i));
                }
            }
        }

        while path_len > 0 {
            let mut inserted = false;
            let mut iter = 0;
            while !inserted {
                iter += 1;
                if iter >= 1000 {
                    return Err(());
                }
                let (mut x, mut y) = path[rand::thread_rng().gen_range(0..path.len())];
                match rand::thread_rng().gen_range(0..4) {
                    //UP
                    0 => y -= 1,
                    //DOWN
                    1 => y += 1,
                    //RIGHT
                    2 => x += 1,
                    //LEFT
                    3 => x -= 1,
                    _ => panic!()
                }
                if !(x > 1 && x < matrix.width - 1 && y > 1 && y < matrix.height - 1) {
                    continue;
                }
                if matrix.matrix[y][x].cell_type == CellType::None {
                    matrix.matrix[y][x] = MatrixCell::point();
                    path.push((x, y));
                    whole_path.push((x, y));
                    inserted = true;
                }
            }

            inserted = false;
            let mut iter = 0;
            while !inserted && (path_len % 5 == 0 || path_len % 5 == 1 || path_len % 5 == 2) {
                iter += 1;
                if iter >= 2000 {
                    return Err(());
                }
                let (mut x, mut y) = path[rand::thread_rng().gen_range(0..path.len())];
                match rand::thread_rng().gen_range(0..4) {
                    //UP
                    0 => y -= 1,
                    //DOWN
                    1 => y += 1,
                    //RIGHT
                    2 => x += 1,
                    //LEFT
                    3 => x -= 1,
                    _ => panic!()
                }
                if !(x > 1 && x < matrix.width - 1 && y > 1 && y < matrix.height - 1) {
                    continue;
                }
                if matrix.matrix[y][x].cell_type == CellType::None {
                    matrix.matrix[y][x] = MatrixCell::wall();
                    inserted = true;
                }
            }

            let mut to_remove = Vec::new();
            for i in 0..path.len() {
                let (x, y) = path[i];
                if matrix.matrix[y + 1][x].cell_type == CellType::Wall && matrix.matrix[y - 1][x].cell_type == CellType::Wall &&
                matrix.matrix[y][x + 1].cell_type == CellType::Wall && matrix.matrix[y][x - 1].cell_type == CellType::Wall {
                    to_remove.push(i)
                }
            }
            for index in to_remove.iter().rev() {
                path.remove(*index);
            }

            path_len -= 1;
        }

        for _ in 0..4 {
            let (x, y) = path[rand::thread_rng().gen_range(0..path.len())];
            matrix.matrix[y][x] = MatrixCell::bonus();
        }

        Ok(matrix)
    }
}

impl MapGenerator for DefaultMapGenerator {
    fn generate_map() -> GameMap {
        const SCREEN_HEIGHT: usize = 37;
        const SCREEN_WIDTH: usize = 42;

        const PATH_LEN: usize = 400;

        const TILE_HEIGHT: usize = 10;
        const TILE_WIDTH: usize = 10;

        let mut matrix = Self::generate_map_matrix_from_template(PATH_LEN);
        while let Err(_) = matrix {
            matrix = Self::generate_map_matrix_from_template(PATH_LEN);
            println!("ERROR")
        }
        let matrix = matrix.unwrap();

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