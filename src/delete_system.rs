use crate::{components::{Deleteable, RaycastSet}, GameState, actions::{Actions, Tool}};
use bevy::{prelude::*, transform::{self, commands}, sprite::Mesh2dHandle};
use bevy_mod_raycast::{
    DefaultRaycastingPlugin, Intersection, RaycastMesh, RaycastMethod, RaycastSource, RaycastSystem,
};

pub struct DeleteSystemPlugin;

impl Plugin for DeleteSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(delete_system.in_set(OnUpdate(GameState::Playing)))
        .add_system(update_raycast_with_cursor.before(RaycastSystem::BuildRays::<RaycastSet>).in_base_set(CoreSet::First));
    }
}

fn delete_system(delete_q: Query<(Entity, &Transform, &Mesh2dHandle, &Deleteable)>, intersection_q: Query<(&Intersection<RaycastSet>,  Entity)>, entities_with_int: Query<(&RaycastMesh<RaycastSet>, Entity), With<Intersection<RaycastSet>>>, actions: Res<Actions>, mut commands: Commands) {
    if(actions.left_click && actions.current_tool() == Some(Tool::Delete)) {
        for (intersection, entity) in &intersection_q{
            info!(
                "Distance {:?}, Position {:?}, Entity {:?}",
                intersection.distance(),
                intersection.position(),
                entity
            );
        }
        
        for (mesh, entity) in entities_with_int.iter() {
           info!("Entity {:?}", entity);
        }
    }
}
fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RaycastSource<RaycastSet>>,
) {
    // Grab the most recent cursor event if it exists:
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };

    for mut pick_source in &mut query {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_position);
    }
}