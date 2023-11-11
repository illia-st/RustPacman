use super::cell::{WFCCell, CellType};

#[derive(Default, Debug, Clone)]
pub struct WFCMatrix {
    pub height: usize,
    pub width: usize,
    pub matrix: Vec<Vec<WFCCell>>
}

impl WFCMatrix {
    pub fn new(height: usize, width: usize) -> Self {
        let mut matrix = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(WFCCell::new())
            }
            matrix.push(row)
        }

        WFCMatrix {
            height,
            width,
            matrix,
        }
    }
}