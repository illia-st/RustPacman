use std::cmp::{max, min};
use crate::core::GameStatus;
use super::map::graph::cell::GraphCell;
use std::collections::HashMap;
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

#[derive(Clone)]
pub struct AStarCell {
    pub g_cost: usize,          // distance from the starting node
    pub h_cost: usize,          // distance from the end node (can be heuristic)
    pub f_cost: usize,          // g_cost + h_cost
    pub parent: Option<usize>,  // parent cell for tracing the route when the end cell is found
}

impl AStarCell {
    fn manhattan_distance(x_min: usize, y_min: usize, x_max: usize, y_max: usize) -> usize {
        x_max - x_min + y_max - y_min
    }
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

    fn find_way_to_pacman(&mut self, way: &mut [GraphCell], current_pacman_pos: usize) -> Vec<usize>{
        const G_COST_STEP: usize = 10;
        // I have decided to use hashmap here because I need to store astar distances by usize of the cell
        let mut opened = HashMap::<usize, AStarCell>::default();
        // And I have decided to use HashSet here because I don't need to store the distances of the closed cell
        let mut closed = HashMap::<usize, AStarCell>::default();

        // need to calculate the distances for the root cell
        let root_x = way.get(self.curr_cell).unwrap().x;
        let root_y = way.get(self.curr_cell).unwrap().y;

        let target_x = way.get(current_pacman_pos).unwrap().x;
        let target_y = way.get(current_pacman_pos).unwrap().y;

        // g_cost fot the root is 0 because it's a starting point
        let g_cost: usize = 0;
        // calculate h_cost by using manhattan_distance
        let h_cost = AStarCell::manhattan_distance(
            min(root_x, target_x),
            min(root_y, target_y),
            max(root_x, target_x),
            max(root_y, target_y),
        );

        let f_cost = g_cost + h_cost;
        // add the start cell to open
        opened.insert(self.curr_cell, AStarCell {
            g_cost,
            h_cost,
            f_cost,
            parent: None,   // as it is a root cell there is no parent
        });
        // start looping
        loop {
            if opened.is_empty() {
                return vec![];
            }
            // find the cell with the min f_cost
            let cell= *opened
                .iter()
                .min_by(|a, b| a.1.f_cost.cmp(&b.1.f_cost))
                .map(|(k, _v)| k)
                .unwrap();
            // remove from the open list
            let astar = opened.remove(&cell).unwrap();
            // add it to the closed one
            closed.insert(cell, astar.clone());

            // check if the current cell is the target cell (the position of pacman)
            if cell == current_pacman_pos {
                // means that we have found the way
                break;
            }
            // get it's neighbours
            let neighbours = way.get(cell).unwrap().next_cells.clone();
            for neighbour in neighbours {
                if closed.contains_key(&neighbour) {
                    continue;
                }
                let neighbour_cell = way.get(neighbour).unwrap();
                let neighbour_x = way.get(neighbour).unwrap().x;
                let neighbour_y = way.get(neighbour).unwrap().y;

                // calculate a new g_cost
                let new_neighbour_g_cost = astar.g_cost + G_COST_STEP;

                let old_neighbour_star = opened.get_mut(&neighbour);

                if old_neighbour_star.is_none() && astar.g_cost == 0 && neighbour_cell.ghost_presence {
                    closed.insert(neighbour, AStarCell { g_cost: 0, h_cost: 0, f_cost: 0, parent: None });
                    continue;
                }

                if old_neighbour_star.is_none() {
                    let neighbour_h_cost = AStarCell::manhattan_distance(
                        min(neighbour_x, target_x),
                        min(neighbour_y, target_y),
                        max(neighbour_x, target_x),
                        max(neighbour_y, target_y),
                    );
                    let neighbour_astar = AStarCell {
                        g_cost: new_neighbour_g_cost,
                        h_cost: neighbour_h_cost,
                        f_cost: new_neighbour_g_cost + neighbour_h_cost,
                        parent: Some(cell),
                    };
                    opened.insert(neighbour, neighbour_astar);
                    continue;
                }
                let old_neighbour_star = old_neighbour_star.unwrap();

                if new_neighbour_g_cost < old_neighbour_star.g_cost {
                    old_neighbour_star.g_cost = new_neighbour_g_cost;
                    old_neighbour_star.f_cost = new_neighbour_g_cost + old_neighbour_star.h_cost;
                    old_neighbour_star.parent = Some(cell);
                }
            }
        }
        let mut curr_cell = current_pacman_pos;
        let mut route = vec![curr_cell];
        while closed.get(&curr_cell).unwrap().parent.is_some() {
            let curr_astar = closed.get(&curr_cell).unwrap();
            if curr_astar.parent.unwrap() != self.curr_cell {
                route.push(curr_astar.parent.unwrap());    
            }
            curr_cell = curr_astar.parent.unwrap();
        }
        route
    }
    pub fn update_state(&mut self, way: &mut [GraphCell], matrix: &mut Vec<Vec<MatrixCell>>, current_pacman_pos: usize) -> GameStatus {
        let event_capture = Utc::now();
        if event_capture.signed_duration_since(self.last_event_capture) < self.update_delta {
            return GameStatus::Running;
        }
        self.last_event_capture = event_capture;

        if self.pacman_pos != current_pacman_pos || self.computed_way.is_empty() {
            self.computed_way = self.find_way_to_pacman(way, current_pacman_pos);
            self.pacman_pos = current_pacman_pos;
        }

        let mut x = way.get(self.curr_cell).unwrap().x;
        let mut y = way.get(self.curr_cell).unwrap().y;

        match !self.computed_way.is_empty() {
            true => {
                let next_cell = *self.computed_way.last().unwrap();
                if way.get(next_cell).unwrap().ghost_presence {
                    return GameStatus::Running;
                }

                // TODO: check ghost presence here, we can't go to the cell where ghost is present
                way.get_mut(self.curr_cell).unwrap().ghost_presence = false;
                matrix.get_mut(x).unwrap().get_mut(y).unwrap().cell_presence = CellPresence::None;

                self.computed_way.pop();
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