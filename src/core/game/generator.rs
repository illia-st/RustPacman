use super::game::Game;

pub trait GameGenerator {
    fn generate_game() -> Game;
}

pub struct DefaultGameGenerator {}

impl GameGenerator for DefaultGameGenerator {
    fn generate_game() -> Game {
        todo!()
    }
}