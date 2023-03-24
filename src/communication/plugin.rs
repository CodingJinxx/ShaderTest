use bevy::prelude::*;

use crate::{shared_state::SharedState, communication::{CommunicationBridge, messages::Message}, events::MapSwitchEvent};
use crate::{model::map::Map};
use lazy_static::lazy_static;

use super::communication_bridge::Channel;

pub struct InteropCommunicationPlugin;

lazy_static! {
    pub static ref COMMUNICATION_BRIDGE: CommunicationBridge<Message> = CommunicationBridge::new();
}

impl Plugin for InteropCommunicationPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(SharedState::new())
        .add_system(handle_incoming_messages);
    }
}


// Messages from C# come in here
// Can manipulate the SharedState
fn handle_incoming_messages(mut state: ResMut<SharedState>, mut ev_map_switch: EventWriter<MapSwitchEvent>) {
    if let Ok(x) = COMMUNICATION_BRIDGE.receive() {
        match x {
            Message::DisplayMap { id} => {
                // If we have current map
                if state.current_map.is_some()
                {   
                    if state.current_map.as_ref().unwrap().id == id {
                        return;
                    }
                }

                let new_map : Map;
                // Check if we have this map
                if let Some(select_map) = state.all_maps.iter().find(|m| m.id == id) {
                    // Save new map for later
                    new_map = select_map.clone();
                }
                else {
                    error!("Map with {} doesnt exist", id);
                    return;
                }
                
                // If we have a current_map
                if state.current_map.is_some() {
                    // Update current map in the list of all maps
                    if let Some(old_map_index) = state.all_maps.iter().position(|x| x.id == state.current_map.as_ref().unwrap().id) {
                        state.as_mut().all_maps[old_map_index] = state.current_map.clone().unwrap();
                    }
                    else {
                        // If its not in there for some weird reason add it
                        warn!("Current map wasnt in all maps");
                        let current_map = state.current_map.clone().unwrap();
                        state.as_mut().all_maps.push(current_map);
                    }
                }
               
                // Finally set the current map to the new map now that old changes have been saved
                state.current_map = Some(new_map);
            
                // Notify all systems that map has changed
                ev_map_switch.send(MapSwitchEvent());
            }
            Message::UploadMapInformation { id, url, overwrite} => {
                // Check if we already have the map, if overwrite is false we will not do anything if it is contained
                let map_already_exists = state.all_maps.iter().any(|m| m.id == id);
                if !overwrite {
                    // Check if we have any maps with matching id
                    if map_already_exists {
                        return;
                    }
                }

                // If we dont have the map add it
                if !map_already_exists {
                    state.all_maps.push(Map::new(id, url))
                }
                else {
                    // If we do have the map update it
                    if let Some(old_map_index) = state.all_maps.iter().position(|x| x.id == state.current_map.as_ref().unwrap().id) {
                        state.all_maps[old_map_index] = Map::new(id, url);
                    }
                    else {
                        error!("Map should exists");
                    }
                }
            }
            _ => {}
        }
    }
}