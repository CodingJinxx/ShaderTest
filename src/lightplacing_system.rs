use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
use bevy_prototype_lyon::{prelude::{ShapeBundle, GeometryBuilder, Fill, Stroke}, shapes};

use crate::{actions, GameState, components::Deleteable, lighting::LightSource};

pub struct LightPlaceSystem;

impl Plugin for LightPlaceSystem {
    fn build(&self, app: &mut App) {
        app.add_system(handle_place_lights.in_set(OnUpdate(GameState::Playing)));
    }
}

pub fn handle_place_lights(actions: Res<actions::Actions>, mut commands: Commands) {
    let curs = actions.world_cursor_position;

    if let Some(curs) = curs {
        if actions.left_click && actions.current_tool() == Some(actions::Tool::PlaceLight) {
            commands.spawn((ShapeBundle {
                 path: GeometryBuilder::build_as(&shapes::Rectangle{
                     extents: Vec2::new(10.0, 10.0),
                     origin: shapes::RectangleOrigin::Center,
                     ..default()
                 }),
                 transform: Transform::from_translation(Vec3::new(curs.x, curs.y, 0.0)),
                 ..default()
             },
             Fill::color(Color::WHITE),
             Stroke::new(Color::WHITE, 1.0),
             PickableBundle::default(),
             Deleteable,
             LightSource {
                position: Vec2::new(curs.x, curs.y),
                color: Vec4::new(1.0, 1.0, 1.0, 1.0),
                intensity: 2.0,
                radius: 1000.0,
                is_active: 1
             }
            ));
         }
    }
}
