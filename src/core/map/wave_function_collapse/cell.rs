#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum CellType {
    W,
    HWL,
    HWR,
    HWU,
    HWD,
    RWLU,
    RWRU,
    RWLD,
    RWRD,
    EWLU,
    EWRU,
    EWLD,
    EWRD,
    POINT,
    BONUS,
    GHOST,
    PATHWAY,
    NONE,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct WFCCell {
    pub cell_type: CellType,
    pub possible_states: Vec<CellType>
}

impl WFCCell {
    pub fn new() -> Self {
        WFCCell {
            cell_type: CellType::NONE,
            possible_states: vec![
                CellType::W,
                CellType::HWL,
                CellType::HWR,
                CellType::HWU,
                CellType::HWD,
                CellType::RWLU,
                CellType::RWRU,
                CellType::RWLD,
                CellType::RWRD,
                CellType::EWLU,
                CellType::EWRU,
                CellType::EWLD,
                CellType::EWRD,
                CellType::POINT,
                CellType::BONUS,
                CellType::GHOST,
                CellType::PATHWAY,
            ]
        }
    }

    pub fn get_entrophy(&self) -> usize {
        self.possible_states.len()
    }

    pub fn get_left_neighbour(&self) -> Vec<CellType> {
        todo!()
    }

    pub fn get_right_neighbour(&self) ->  Vec<CellType> {
        todo!()
    }

    pub fn get_top_neighbour(&self) ->  Vec<CellType> {
        todo!()
    }

    pub fn get_bottom_neighbour(&self) ->  Vec<CellType> {
        todo!()
    }
}