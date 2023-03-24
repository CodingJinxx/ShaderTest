use bevy::prelude::Vec2;

#[derive(Clone)]
pub struct WallObject {
    pub position: Vec2,
    pub width: i32,
    pub height: i32
}