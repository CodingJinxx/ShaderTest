use bevy::prelude::Component;

use bevy::prelude::*;

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct LightOccluder {
    pub width: f32,
    pub height: f32,
}