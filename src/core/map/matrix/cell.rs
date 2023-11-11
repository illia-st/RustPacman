#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum CellPresence {
    Pacman,
    Ghost,
    None
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum CellModificator {
    Point,
    Bonus,
    Super,
    None,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum CellType {
    Wall,
    Pathway,
    None,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct MatrixCell {
    pub cell_type: CellType,
    pub cell_modificator: CellModificator,
    pub cell_presence: CellPresence,
}

impl MatrixCell {
    pub fn new(cell_type: CellType, cell_modificator: CellModificator, cell_presence: CellPresence) -> Self {
        MatrixCell {
            cell_type,
            cell_modificator,
            cell_presence
        }
    }

    pub fn wall() -> Self {
        MatrixCell::new(
            CellType::Wall,
            CellModificator::None,
            CellPresence::None
        )
    }
    pub fn pathway() -> Self {
        MatrixCell::new(
            CellType::Pathway,
            CellModificator::None,
            CellPresence::None
        )
    }
    pub fn point() -> Self {
        MatrixCell::new(
            CellType::Pathway,
            CellModificator::Point,
            CellPresence::None
        )
    }
    pub fn bonus() -> Self {
        MatrixCell::new(
            CellType::Pathway,
            CellModificator::Bonus,
            CellPresence::None
        )
    }
    pub fn pacman() -> Self {
        MatrixCell::new(
            CellType::Pathway,
            CellModificator::None,
            CellPresence::Pacman
        )
    }
    pub fn ghost() -> Self {
        MatrixCell::new(
            CellType::Pathway,
            CellModificator::None,
            CellPresence::Ghost
        )
    }
    pub fn undef() -> Self {
        MatrixCell::new(
            CellType::None,
            CellModificator::None,
            CellPresence::None
        )
    }
}