#[derive(Default)]
pub struct Cell {
    pub x: i64,
    pub y: i64,
    // ways is a vector of indexes of the cells to which we can go from the current one
    pub next_cells: Vec<usize>,
    pub ghost_presence: bool,
    pub point_presence: bool,
    pub pacman_presence: bool,
}