use chrono::{DateTime, Duration, Utc};
use crate::core::GameStatus;
use super::map::graph::cell::GraphCell;

#[derive(Debug)]
pub struct Pacman {
    // TODO: remove steps to timestamps
    pub curr_cell: usize,
    pub points: u64,
    pub update_delta: Duration,
    pub last_event_capture: DateTime<Utc>,
}


impl Pacman {
    pub fn new(start_cell: usize, update_delta: Duration, last_event_capture: DateTime<Utc>) -> Self {
        Self {
            curr_cell: start_cell,
            points: 0,
            update_delta,
            last_event_capture,
        }
    }
    pub fn update_state(&mut self, way: &mut [GraphCell]) -> GameStatus {
        // TODO: change speed by measuring timestamps
        let event_capture = Utc::now();
        if event_capture.signed_duration_since(self.last_event_capture) < self.update_delta {
            return GameStatus::Running;
        }
        self.last_event_capture = event_capture;
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