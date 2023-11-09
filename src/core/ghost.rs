use crate::core::GameStatus;
use super::map::graph::cell::GraphCell;
use std::collections::HashSet;
use chrono::{DateTime, Duration, Utc};
use crate::core::map::matrix::cell::{CellPresence, MatrixCell};

#[derive(Debug)]
pub struct Ghost {
    // TODO: change to usize
    pub curr_cell: usize,
    pub pacman_pos: usize,
    computed_way: Vec<usize>,
    pub update_delta: Duration,
    pub last_event_capture: DateTime<Utc>,
}

impl Ghost {
    pub fn new(start_cell: usize, pacman_pos: usize, update_delta: Duration, last_event_capture: DateTime<Utc>) -> Self {
        Self {
            curr_cell: start_cell,
            pacman_pos,
            computed_way: Vec::default(),
            update_delta,
            last_event_capture,
        }
    }
    fn find_way_to_pacman(curr_way: Vec<usize>, mut min_len: usize, tracker: HashSet<usize>, pacman_pos: usize, way: &mut [GraphCell]) -> (Vec<usize>, bool) {
        if curr_way.len() >= min_len {
            return (curr_way, false);
        }
        let curr_pos = *curr_way.last().unwrap();
        if curr_pos == pacman_pos {
            return (curr_way, true);
        }
        let next_cells = way.get(curr_pos).unwrap().next_cells.clone();
        let mut ans: Vec<usize> = Vec::default();
        let mut found_ans = false;
        for cell in next_cells.iter() {
            if tracker.contains(cell) {
                continue;
            }
            let mut new_way = curr_way.clone();
            let mut new_tracker = tracker.clone();
            new_way.push(*cell);
            new_tracker.insert(*cell);

            let (res_way, found) = Self::find_way_to_pacman(new_way, min_len, new_tracker, pacman_pos, way);
            if found && res_way.len() < min_len {
                min_len = res_way.len();
                ans = res_way;
                found_ans = true;
            }
        }
        (ans, found_ans)
    }
    fn start_finding_pacman(&mut self, way: &mut [GraphCell], current_pacman_pos: usize) {
        let next_cells = way.get(self.curr_cell).unwrap().next_cells.clone();
        self.computed_way.clear();
        for cell in next_cells {
            if way.get(cell).unwrap().ghost_presence {
                continue;
            }
            // TODO: think about changing hashset to vector of bool
            let mut tracker = HashSet::new();
            tracker.insert(cell);
            let (ans_way, found) = Self::find_way_to_pacman(vec![cell], usize::MAX, tracker, current_pacman_pos, way);
            if found && (self.computed_way.is_empty() || self.computed_way.len() > ans_way.len()) {
                self.computed_way = ans_way;
            }
        }
        self.pacman_pos = current_pacman_pos;
    }
    pub fn update_state(&mut self, way: &mut [GraphCell], matrix: &mut Vec<Vec<MatrixCell>>, current_pacman_pos: usize) -> GameStatus {
        if self.pacman_pos != current_pacman_pos && self.computed_way.is_empty() {
            self.start_finding_pacman(way, current_pacman_pos);
        } else if self.pacman_pos != current_pacman_pos {
            // if we have found pacman in our way, it means that he is closer to us in one cell
            // so we can delete the first one
            // TODO: probably change Vec to Deque
            let found = self.computed_way.iter().rfind(|cell| **cell == current_pacman_pos);
            match found {
                None => {
                    self.computed_way.push(current_pacman_pos);
                },
                Some(_) => {
                    self.computed_way.pop();
                }
            };
        }

        let event_capture = Utc::now();
        if event_capture.signed_duration_since(self.last_event_capture) < self.update_delta {
            return GameStatus::Running;
        }
        self.last_event_capture = event_capture;

        // if self.pacman_pos != current_pacman_pos {
        //     self.start_finding_pacman(way, current_pacman_pos);
        // }

        let mut x = way.get(self.curr_cell).unwrap().x;
        let mut y = way.get(self.curr_cell).unwrap().y;

        match !self.computed_way.is_empty() {
            true => {
                let next_cell = *self.computed_way.first().unwrap();
                if way.get(next_cell).unwrap().ghost_presence {
                    return GameStatus::Running;
                }

                // TODO: check ghost presence here, we can't go to the cell where ghost is present
                way.get_mut(self.curr_cell).unwrap().ghost_presence = false;
                matrix.get_mut(x).unwrap().get_mut(y).unwrap().cell_presence = CellPresence::None;

                self.computed_way.remove(0);
                self.curr_cell = next_cell;

                way.get_mut(self.curr_cell).unwrap().ghost_presence = true;
                x = way.get(self.curr_cell).unwrap().x;
                y = way.get(self.curr_cell).unwrap().y;
                matrix.get_mut(x).unwrap().get_mut(y).unwrap().cell_presence = CellPresence::Ghost;

                if way.get_mut(next_cell).unwrap().pacman_presence {
                    return GameStatus::Finished;
                }
                GameStatus::Running
            }
            false => GameStatus::Running
        }
    }
}