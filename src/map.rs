use bevy::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;

use crate::{loading::TextureAssets, GameState};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_map.in_schedule(OnEnter(GameState::Playing)));
    }
}

fn setup_map(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn(SpriteBundle {
        texture: textures.dungeon_map.clone(),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..Default::default()
    });
}

fn handle_input(mut commands: Commands) {
    
}