use crate::core::GameStatus;
use super::cell::Cell;
use std::collections::HashSet;

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
    fn find_way_to_pacman(curr_way: Vec<usize>, tracker: HashSet<usize>, pacman_pos: usize, way: &mut [Cell]) -> Vec<usize> {
        let curr_pos = *curr_way.last().unwrap();
        if curr_pos == pacman_pos {
            return curr_way;
        }
        let next_cells = way.get(curr_pos).unwrap().next_cells.clone();
        let mut ans: Vec<usize> = Vec::default();
        for cell in next_cells.iter() {
            if tracker.contains(cell) {
                continue;
            }
            let mut new_way = curr_way.clone();
            let mut new_tracker = tracker.clone();
            new_way.push(*cell);
            new_tracker.insert(*cell);

            let res = Self::find_way_to_pacman(new_way, new_tracker, pacman_pos, way);
            if ans.is_empty() || res.len() < ans.len() {
                ans = res;
            }
        }
        match ans.is_empty() {
            true => curr_way,
            false => ans
        }
    }
    pub fn update_state(&mut self, way: &mut [Cell], current_pacman_pos: usize) -> GameStatus {
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
                let mut tracker = HashSet::new();
                tracker.insert(cell);
                let ans = Self::find_way_to_pacman(vec![cell], tracker, current_pacman_pos, way);
                if self.computed_way.len() > ans.len() {
                    self.computed_way = ans;
                }
            }
            self.computed_way.reverse();
            self.pacman_pos = current_pacman_pos;
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