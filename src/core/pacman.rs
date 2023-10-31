use crate::core::GameStatus;
use super::cell::Cell;

#[derive(Default)]
pub struct Pacman {
    // TODO: remove steps to timestamps
    pub whole_step: u32,
    pub actual_step: u32,
    pub curr_cell: usize,
    pub points: u64,
}


impl Pacman {
    pub fn new(whole_step: u32, start_cell: usize) -> Self {
        Self {
            whole_step,
            actual_step: 0,
            curr_cell: start_cell,
            points: 0,
        }
    }
    pub fn update_state(&mut self, way: &mut [Cell]) -> GameStatus {
        // TODO: change speed by measuring timestamps
        // self.actual_step += 1;
        // if self.actual_step != self.whole_step {
        //     return GameStatus::Running;
        // }
        // self.actual_step = 0;
        if way.get(self.curr_cell).unwrap().next_cells.is_empty() {
            // means that pacman has nowhere to go
            return GameStatus::Finished;
        }
        let mut indexes_of_possible_cells = Vec::<usize>::default();
        let next_cells = way.get(self.curr_cell).unwrap().next_cells.clone();
        for next_cell in next_cells.iter() {
            if way.get(*next_cell).unwrap().ghost_presence {
                // means that there is a ghost
                continue;
            }
            if way.get(*next_cell).unwrap().point_presence {
                let new_cell = way.get_mut(*next_cell).unwrap();
                new_cell.point_presence = false;
                new_cell.pacman_presence = true;

                way.get_mut(self.curr_cell).unwrap().pacman_presence = false;
                self.curr_cell = *next_cell;
                self.points += 1;
                return GameStatus::Running;
            }
            indexes_of_possible_cells.push(*next_cell);
        }
        if indexes_of_possible_cells.is_empty() {
            return GameStatus::Finished;
        }
        // TODO: calculate the distance to the nearest point
        let new_cell = way.get_mut(*indexes_of_possible_cells.first().unwrap()).unwrap();
        new_cell.pacman_presence = true;
        way.get_mut(self.curr_cell).unwrap().pacman_presence = false;
        self.curr_cell = *indexes_of_possible_cells.first().unwrap();

        GameStatus::Running
    }

}