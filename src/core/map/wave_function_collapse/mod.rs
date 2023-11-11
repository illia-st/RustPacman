use crate::core::map::wave_function_collapse::matrix::WFCMatrix;

use super::matrix::matrix::MapMatrix;

pub mod cell;
pub mod matrix;

struct WFC {}

impl WFC {
    pub fn collapse(heigth: usize, width: usize) -> MapMatrix {
        let matrix = WFCMatrix::new(heigth, width);

        MapMatrix::convert_from_wfc(matrix)
    }
}