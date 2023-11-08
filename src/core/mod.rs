// need to remember about the speed
// let measure speed in ticks per cells
// initially let pacman have 1 tick per cell
// let ghost have 5 ticks per cell
// so, ghosts will be 5 times slower than pacman
#[derive(PartialEq)]
pub enum GameStatus {
    Running,
    Finished,
}
pub mod pacman;
pub mod ghost;
pub mod map;
pub mod game;