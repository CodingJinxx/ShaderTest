use bevy::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::communication::{messages::Message, communication_bridge::Channel};

use super::plugin::COMMUNICATION_BRIDGE;

#[wasm_bindgen]
pub fn send_map_information(url: String, id: String) {
    info!("Map Information Received");
    COMMUNICATION_BRIDGE.send(Message::UploadMapInformation {
        id,
        url,
        overwrite: false
    });
}

#[wasm_bindgen]
pub fn display_map(id: String) {
    info!("Display Map");
    COMMUNICATION_BRIDGE.send(Message::DisplayMap {
        id
    });
}

