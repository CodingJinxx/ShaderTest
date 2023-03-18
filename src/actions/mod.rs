use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::PrimaryWindow;

use crate::actions::game_control::{get_movement, GameControl};
use crate::GameState;

pub use self::tools::Tool;

mod game_control;
mod tools;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>()
            // .add_system(set_movement_actions.in_set(OnUpdate(GameState::Playing)));
            .add_system(set_cursor_position.in_set(OnUpdate(GameState::Playing)))
            .add_system(update_mouse_click.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    // pub player_movement: Option<Vec2>,
    current_tool: Option<Tool>,
    previous_tool: Option<Tool>,
    pub cursor_position_raw: Option<Vec2>,
    pub world_cursor_position: Option<Vec2>,
    pub left_click: bool,
    pub ui_just_clicked: bool,
}

// pub fn set_movement_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
//     let player_movement = Vec2::new(
//         get_movement(GameControl::Right, &keyboard_input)
//             - get_movement(GameControl::Left, &keyboard_input),
//         get_movement(GameControl::Up, &keyboard_input)
//             - get_movement(GameControl::Down, &keyboard_input),
//     );

//     if player_movement != Vec2::ZERO {
//         actions.player_movement = Some(player_movement.normalize());
//     } else {
//         actions.player_movement = None;
//     }
// }

impl Actions {
    pub fn current_tool(&self) -> Option<Tool> {
        self.current_tool
    }

    pub fn update_tool(&mut self, tool: Tool) {
        self.previous_tool = self.current_tool;
        self.current_tool = Some(tool);
    }

    pub fn revert_to_previous_tool(&mut self) {
        std::mem::swap(&mut self.previous_tool, &mut self.current_tool)
    }
}

pub fn set_cursor_position(mut actions: ResMut<Actions>, window_q: Query<&Window, With<PrimaryWindow>>, camera_q: Query<(&Camera, &GlobalTransform)>) {
    let (camera, camera_transform) = camera_q.single();

    let primary_window = window_q.single();

    primary_window.cursor_position().map(|position| {
        actions.cursor_position_raw = Some(position);
        actions.world_cursor_position = Some(camera.viewport_to_world_2d(camera_transform, position).unwrap());
    });
}

pub fn update_mouse_click(mut actions: ResMut<Actions>, mouse_button_input: Res<Input<MouseButton>>) {
    if(!actions.ui_just_clicked) {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            actions.left_click = true;
        } else {
            actions.left_click = false;
        }
    }
}
