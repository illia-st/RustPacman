use crate::core::GameStatus;
use super::cell::Cell;

#[derive(Default)]
pub struct Ghost {
    // TODO: change to usize
    pub curr_cell: usize,
    pub pacman_pos: usize,
    computed_way: Vec<usize>,
}

impl Ghost {
    pub fn new(start_cell: usize, pacman_pos: usize) -> Self {
        Self {
            curr_cell: start_cell,
            pacman_pos,
            computed_way: Vec::default(),
        }
    }
    fn find_way_to_pacman(curr_way: Vec<usize>, way: &mut [Cell]) -> Vec<usize> {
        todo!()
    }
    pub fn update_state(&mut self, way: &mut [Cell], current_pacman_pos: usize) -> GameStatus {
        // so, need to define what is goind to happen here
        // 1) we have a current pacman position
        // 2) we have the way
        // 3) need to compute the minimal distance to current pacman position
        // and update position of the ghost
        // we can do so by using dfs
        if self.pacman_pos == current_pacman_pos && !self.computed_way.is_empty() {
            way.get_mut(self.curr_cell).unwrap().ghost_presence = false;
            let next_cell = *self.computed_way.last().unwrap();
            self.computed_way.pop();
            if way.get_mut(next_cell).unwrap().pacman_presence {
                return GameStatus::Finished;
            }
            way.get_mut(next_cell).unwrap().ghost_presence = true;
            self.curr_cell = next_cell;
            return GameStatus::Running;
        }
        if self.pacman_pos != current_pacman_pos {
            let next_cells = way.get(self.curr_cell).unwrap().next_cells.clone();
            for cell in next_cells {
                let ans = Self::find_way_to_pacman(vec![cell], way);
                if self.computed_way.len() > ans.len() {
                    self.computed_way = ans;
                }
            }
        }
        match self.computed_way.is_empty() {
            true => {
                way.get_mut(self.curr_cell).unwrap().ghost_presence = false;
                let next_cell = *self.computed_way.last().unwrap();
                self.computed_way.pop();
                if way.get_mut(next_cell).unwrap().pacman_presence {
                    return GameStatus::Finished;
                }
                way.get_mut(next_cell).unwrap().ghost_presence = true;
                self.curr_cell = next_cell;
                GameStatus::Running
            }
            false => GameStatus::Finished
        }
    }
}