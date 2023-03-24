use bevy::prelude::{App, Plugin};

pub struct MapSwitchEvent();

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<MapSwitchEvent>();
    }
}