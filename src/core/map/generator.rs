use super::map::GameMap;

pub trait MapGenerator {
    fn generate_map() -> GameMap;
}

pub struct DefaultMapGenerator {}

impl MapGenerator for DefaultMapGenerator {
    fn generate_map() -> GameMap {
        todo!()
    }
}