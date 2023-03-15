use bevy::prelude::*;
use bevy_pancam::*;

use crate::GameState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PanCamPlugin::default())
        .add_system(setup_camera.in_schedule(OnEnter(GameState::Playing)));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default())
    .insert(PanCam {
        grab_buttons: vec![MouseButton::Left, MouseButton::Middle], // which buttons should drag the camera
        enabled: true, // when false, controls are disabled. See toggle example.
        zoom_to_cursor: true, // whether to zoom towards the mouse or the center of the screen
        min_scale: 1., // prevent the camera from zooming too far in
        max_scale: Some(40.), // prevent the camera from zooming too far out
        ..Default::default()
    });
}