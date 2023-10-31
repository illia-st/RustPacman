use crate::core::GameStatus;
use super::cell::Cell;

#[derive(Default)]
pub struct Ghost {
    pub whole_step: u32,
    pub actual_step: u32,
    pub curr_cell: Cell,
}

impl Ghost {
    pub fn new(whole_step: u32, start_cell: Cell) -> Self {
        Self {
            whole_step,
            actual_step: 0,
            curr_cell: start_cell,
        }
    }
    pub fn update_state(&mut self) -> GameStatus {
        self.actual_step += 1;
        if self.actual_step != self.whole_step {
            return GameStatus::Running;
        }
        self.actual_step = 0;
        todo!("implement ghost state updating");
    }
}