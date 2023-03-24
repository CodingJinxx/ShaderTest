use super::{wall::WallObject};

#[derive(Clone)]
pub struct Map {
    pub walls: Vec<WallObject>,
    // pub game_objects: Vec<GameObject>,
    pub url: String,
    pub id: String
}

impl Map {
    pub fn new(id: String, url: String) -> Self {
        Self {
            walls: Vec::new(),
            // game_objects: Vec::new(),
            url: url,
            id: id
        }
    }
}