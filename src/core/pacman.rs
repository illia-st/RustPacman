use std::collections::HashSet;
use chrono::{DateTime, Duration, Utc};
use rand::{random, Rng};
use crate::core::GameStatus;
use crate::core::map::matrix::cell::{CellModificator, CellPresence, MatrixCell};
use super::map::graph::cell::GraphCell;

#[derive(Debug)]
pub struct Pacman {
    // TODO: remove steps to timestamps
    pub curr_cell: usize,
    pub points: u64,
    pub update_delta: Duration,
    pub last_event_capture: DateTime<Utc>,
    computed_way: Vec<usize>,
}


impl Pacman {
    pub fn new(start_cell: usize, update_delta: Duration, last_event_capture: DateTime<Utc>) -> Self {
        Self {
            curr_cell: start_cell,
            points: 0,
            update_delta,
            last_event_capture,
            computed_way: Vec::default(),
        }
    }
    fn find_point(curr_way: Vec<usize>, mut min_len: usize, tracker: HashSet<usize>, way: &mut [GraphCell]) -> (Vec<usize>, bool) {
        if curr_way.len() >= min_len {
            return (curr_way, false);
        }
        let curr_pos = *curr_way.last().unwrap();
        // we don't check here for ghost because we can't call this method with the cell with the ghost
        if way.get(curr_pos).unwrap().point_presence || way.get(curr_pos).unwrap().bonus_presence {
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
            let (res_way, found) = Self::find_point(new_way, min_len, new_tracker, way);
            if found && res_way.len() < min_len {
                min_len = res_way.len();
                ans = res_way;
                found_ans = true;
            }
        }
        (ans, found_ans)
    }
    fn start_finding_point(&mut self, possible_cells: Vec<usize>, way: &mut [GraphCell]) {
        // let next_cells = way.get(self.curr_cell).unwrap().next_cells.clone();
        self.computed_way.clear();
        for cell in possible_cells {
            let mut tracker = HashSet::new();
            tracker.insert(cell);
            let (ans_way, found) = Self::find_point(vec![cell], usize::MAX, tracker, way);
            if found && (self.computed_way.is_empty() || self.computed_way.len() > ans_way.len()) {
                self.computed_way = ans_way;
            }
        }
        self.computed_way.reverse();
    }

    fn go_by_computed_way(&mut self, way: &mut [GraphCell], matrix: &mut Vec<Vec<MatrixCell>>, x: &mut usize, y: &mut usize) -> bool {
        if self.computed_way.is_empty() {
            // println!("computed way is empty");
            return false;
        }

        let next_cell = *self.computed_way.last().unwrap();
        self.computed_way.pop();
        if way.get(next_cell).unwrap().ghost_presence {
            return false;
        }
        way.get_mut(self.curr_cell).unwrap().pacman_presence = false;
        matrix.get_mut(*x).unwrap().get_mut(*y).unwrap().cell_presence = CellPresence::None;
        self.curr_cell = next_cell;
        *x = way.get(self.curr_cell).unwrap().x;
        *y = way.get(self.curr_cell).unwrap().y;
        let new_cell = way.get_mut(self.curr_cell).unwrap();
        matrix.get_mut(*x).unwrap().get_mut(*y).unwrap().cell_presence = CellPresence::Pacman;
        new_cell.pacman_presence = true;
        if new_cell.point_presence {
            new_cell.point_presence = false;
            matrix.get_mut(*x).unwrap().get_mut(*y).unwrap().cell_modificator = CellModificator::None;
            self.points += 1;
        }
        true
    }
    fn go_to_cell_with_modificator(&mut self, cells_with_modificators: Vec<usize>, way: &mut [GraphCell], matrix: &mut Vec<Vec<MatrixCell>>, x: &mut usize, y: &mut usize) {
        let cell: usize = rand::thread_rng().gen_range(0..cells_with_modificators.len());
        let next_cell = cells_with_modificators[cell];
        let new_cell = way.get_mut(next_cell).unwrap();
        if new_cell.point_presence {
            self.points += 1;
            new_cell.point_presence = false;
        }
        new_cell.bonus_presence = false;
        new_cell.pacman_presence = true;
        matrix.get_mut(*x).unwrap().get_mut(*y).unwrap().cell_presence = CellPresence::None;

        way.get_mut(self.curr_cell).unwrap().pacman_presence = false;
        self.curr_cell = next_cell;
        *x = way.get(self.curr_cell).unwrap().x;
        *y = way.get(self.curr_cell).unwrap().y;
        matrix.get_mut(*x).unwrap().get_mut(*y).unwrap().cell_modificator = CellModificator::None;

        matrix.get_mut(*x).unwrap().get_mut(*y).unwrap().cell_presence = CellPresence::Pacman;
    }

    pub fn update_state(&mut self, way: &mut [GraphCell], matrix: &mut Vec<Vec<MatrixCell>>) -> GameStatus {
        // TODO: change speed by measuring timestamps
        let event_capture = Utc::now();
        if event_capture.signed_duration_since(self.last_event_capture) < self.update_delta {
            return GameStatus::Running;
        }
        self.last_event_capture = event_capture;

        let mut x = way.get(self.curr_cell).unwrap().x;
        let mut y = way.get(self.curr_cell).unwrap().y;

        if way.get(self.curr_cell).unwrap().next_cells.is_empty() {
            // means that pacman has nowhere to go
            return GameStatus::Finished;
        }

        if self.go_by_computed_way(way, matrix, &mut x, &mut y) {
            return GameStatus::Running;
        }

        let mut indexes_of_possible_cells = Vec::<usize>::default();
        let mut cells_with_modificators = Vec::<usize>::default();
        let next_cells = way.get(self.curr_cell).unwrap().next_cells.clone();
        for next_cell in next_cells.iter() {
            if way.get(*next_cell).unwrap().ghost_presence {
                // means that there is a ghost
                continue;
            }
            if way.get(*next_cell).unwrap().point_presence || way.get(*next_cell).unwrap().bonus_presence {
                cells_with_modificators.push(*next_cell);
                continue;
            }
            indexes_of_possible_cells.push(*next_cell);
        }
        if !cells_with_modificators.is_empty() {
            self.go_to_cell_with_modificator(cells_with_modificators, way, matrix, &mut x, &mut y);
            return GameStatus::Running;
        }
        if indexes_of_possible_cells.is_empty() {
            // wait till ghost eat us :)
            return GameStatus::Running;
        }
        self.start_finding_point(indexes_of_possible_cells, way);
        match self.go_by_computed_way(way, matrix, &mut x, &mut y) {
            true => GameStatus::Running,
            // false means that we have eaten all the points and game is finished
            false => GameStatus::Finished
        }
    }

}