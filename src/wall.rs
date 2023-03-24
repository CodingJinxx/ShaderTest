use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
use bevy_mod_raycast::RaycastMesh;
use bevy_prototype_lyon::prelude::*;
use bevy_pancam::*;

use crate::{GameState, actions::{Actions, Tool, update_mouse_click}, components::{self, Deleteable, RaycastSet}, lighting::LightOccluder};

pub struct WallBuildingPlugin;

impl Plugin for WallBuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_wall_building.after(update_mouse_click).in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Component)]
struct PreliminaryWall;
// Builds a wall, also disables pancam and enables a preliminary wall
fn handle_wall_building(mut actions: ResMut<Actions>, mut commands: Commands, mut preliminary_q: Query<(&mut PreliminaryWall, Entity, &mut Path, &Transform, &mut LightOccluder)>, mut pancam_q: Query<&mut PanCam>) {
    // Create Preliminary Wall 
    let cursor = actions.world_cursor_position.unwrap();
    let mut pancam = pancam_q.single_mut();

    if preliminary_q.iter().len() == 1{
        let (mut preliminary_wall, entity, mut path, transform, mut occluder) = preliminary_q.single_mut();
        let transf = transform.translation.truncate();
        *path = GeometryBuilder::build_as(&shapes::Rectangle{
            extents: Vec2::new(cursor.x, -cursor.y) - Vec2::new(transf.x, -transf.y),
            origin: shapes::RectangleOrigin::TopLeft,
            ..default()
        });

        occluder.width = cursor.x - transf.x;
        occluder.height = transf.y - cursor.y;

        if(actions.left_click) {
            commands.entity(entity).remove::<PreliminaryWall>();
            commands.entity(entity).insert(Deleteable);
            actions.revert_to_previous_tool();
            pancam.enabled = true;
        }
        
    } 
    else if actions.left_click {
        if let Some(Tool::BuildWall) = actions.current_tool() {
            let entity = commands.spawn(
            (ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle{
                    extents: Vec2::new(0.0, 0.0),
                    origin: shapes::RectangleOrigin::TopLeft,
                    ..default()
                }),
                transform: Transform::from_translation(Vec3::new(cursor.x, cursor.y, 1.0)),
                ..default()
            },
            Fill::color(Color::BLACK),
            Stroke::new(Color::BLACK, 1.0),
            PreliminaryWall,
            PickableBundle::default(), 
            LightOccluder {
                width: 0.0,
                height: 0.0,
            }
        )).id();

        info!("Created entity {:?}", entity);
        pancam.enabled = false;
        }
    }
}