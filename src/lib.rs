mod actions;
mod audio;
mod loading;
mod menu;
mod player;
mod lighting;
mod camera;
mod map;
mod ui;
mod wall;
mod components;
mod delete_system;
mod lightplacing_system;
mod communication;
mod bevy_webasset_plugin_mod;
mod events;
mod model;
mod shared_state;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::wall::WallBuildingPlugin;
pub use crate::bevy_webasset_plugin_mod::custom_webasset_plugin::CustomWebAssetPlugin;

use crate::map::MapPlugin;
use crate::ui::UiPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;
use camera::CameraPlugin;
use communication::plugin::InteropCommunicationPlugin;
use delete_system::DeleteSystemPlugin;
use events::EventPlugin;
use lighting::LightingPostprocessPlugin;
use lightplacing_system::LightPlaceSystem;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(LightingPostprocessPlugin)
            .add_plugin(DeleteSystemPlugin)
            .add_plugin(LightPlaceSystem)
            .add_plugin(UiPlugin)
            .add_plugin(ShapePlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(WallBuildingPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(InteropCommunicationPlugin)
            .add_plugin(EventPlugin);
            // .add_plugin(PlayerPlugin);

        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //         .add_plugin(LogDiagnosticsPlugin::default());
        // }
    }
}
