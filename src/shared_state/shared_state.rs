use bevy::prelude::{Resource};

use crate::model::map::Map;

#[derive(Resource)]
pub struct SharedState {
    pub current_map: Option<Map>,
    pub all_maps: Vec<Map>
}

impl SharedState {
    pub fn new() -> Self {
        Self {
            current_map: None,
            all_maps: Vec::new()
        }
    }
}
