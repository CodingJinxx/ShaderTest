use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy_mod_picking::PickingCameraBundle;
use bevy_mod_raycast::DefaultRaycastingPlugin;
use bevy_mod_raycast::RaycastMesh;
use bevy_mod_raycast::RaycastSource;
use bevy_pancam::*;
use crate::components::*;

use crate::GameState;
use crate::lighting::CameraSet;

#[derive(Component)]
pub struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PanCamPlugin::default())
        .add_plugin(DefaultRaycastingPlugin::<RaycastSet>::default());
        // .add_startup_system(setup_camera.in_set(CameraSet::CameraSetup).before(CameraSet::LightingSetup));
    }
}

pub fn setup_camera(mut commands: &mut Commands,target: RenderTarget) {
    commands.spawn(Camera2dBundle {
        camera: Camera{ 
            ..default()
        },
        ..Default::default()
    })
    .insert(PickingCameraBundle::default())
    .insert(MainCamera)
    // .insert(UiCameraConfig { show_ui: false })
    .insert(    PanCam {
        grab_buttons: vec![MouseButton::Left, MouseButton::Middle], // which buttons should drag the camera
        enabled: true, // when false, controls are disabled. See toggle example.
        zoom_to_cursor: true, // whether to zoom towards the mouse or the center of the screen
        min_scale: 1., // prevent the camera from zooming too far in
        max_scale: Some(40.), // prevent the camera from zooming too far out
        ..Default::default()
    });
}