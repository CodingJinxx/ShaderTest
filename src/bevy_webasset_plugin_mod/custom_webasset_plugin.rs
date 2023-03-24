use std::path::PathBuf;

use bevy::{prelude::*};

use super::custom_webasset_io::CustomWebAssetIo;

#[derive(Default)]
pub struct CustomWebAssetPlugin {
    /// Settings for the underlying (regular) AssetPlugin
    pub asset_plugin: AssetPlugin,
}

impl Plugin for CustomWebAssetPlugin {
    fn build(&self, app: &mut App) {
        // First, configure the underlying plugin
        // We use out own watcher, so `watch_for_changes` is always false
        let asset_plugin = AssetPlugin {
            asset_folder: self.asset_plugin.asset_folder.clone(),
            watch_for_changes: false,
        };

        // Create the `FileAssetIo` wrapper
        let asset_io = {
            // This makes calling `WebAssetIo::watch_for_changes` redundant
       

            // Create the `FileAssetIo`
            let default_io = asset_plugin.create_platform_default_asset_io();
            let mut root_path = PathBuf::new();
            #[cfg(not(target_arch = "wasm32"))]
            {
                use bevy::asset::{FileAssetIo};
                root_path = FileAssetIo::get_base_path().join(&self.asset_plugin.asset_folder);
            }
            #[cfg(target_arch = "wasm32")]
            {
                root_path = root_path.join(&self.asset_plugin.asset_folder);
            }
            // The method doesn't change, so we just use `FileAssetIo`'s

            CustomWebAssetIo {
                root_path,
                default_io,
            }
        };

        // Add the asset server with our `WebAssetIo` wrapping `FileAssetIo`
        app.insert_resource(AssetServer::new(asset_io));

        // Add the asset plugin
        app.add_plugin(asset_plugin);
    }
}

impl CustomWebAssetPlugin {
    pub fn new(asset_folder: &str) -> Self {
        let mut plugin = AssetPlugin::default();
        plugin.asset_folder = asset_folder.to_owned();

        Self {
            asset_plugin: plugin
        }
    }
}