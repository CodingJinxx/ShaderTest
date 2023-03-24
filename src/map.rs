use bevy::{
    prelude::*,
    render::{render_resource::Extent3d, view::RenderLayers},
    sprite::MaterialMesh2dBundle,
};
use bevy_prototype_lyon::prelude::ShapePlugin;

use crate::{
    lighting::{LightingMaterial, WrappedBool, WrappedF32, WrappedVec2, WrappedVec4},
    loading::TextureAssets,
    GameState, events::MapSwitchEvent, shared_state::SharedState,
};

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct MapMarker {
    pub width: u32,
    pub height: u32,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_map.in_schedule(OnEnter(GameState::Playing)))
        .add_system(display_map_on_switch.in_set(OnUpdate(GameState::Playing)));
    }
}

fn setup_map(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<Assets<Image>>,
    mut post_processing_materials: ResMut<Assets<LightingMaterial>>,
) {
    let img_handle = textures.dungeon_map.clone();
    commands.spawn(SpriteBundle {
        texture: img_handle.clone(),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..Default::default()
    });

    let image = assets.get(&img_handle).unwrap();

    let width = image.texture_descriptor.size.width;
    let height = image.texture_descriptor.size.height;

    let size = Extent3d {
        width: width,
        height: height,
        ..default()
    };

    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        size.width as f32,
        size.height as f32,
    ))));

    let material_handle = post_processing_materials.add(LightingMaterial {
        colors: [WrappedVec4 { value: Vec4::ZERO }; 64],
        positions: [WrappedVec2 { value: Vec2::ZERO }; 64],
        intensities: [WrappedF32 { value: 0.0 }; 64],
        radiuses: [WrappedF32 { value: 0.0 }; 64],
        is_active: [WrappedBool { value: 0 }; 64],
        occluders: [WrappedVec4 { value: Vec4::ZERO }; 64],
        exists: [WrappedBool { value: 0 }; 64],
        source_image: img_handle.clone(),
    });

    commands.spawn(
        (MaterialMesh2dBundle {
            mesh: quad_handle.into(),
            material: material_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                ..default()
            },
            ..default()
        }, MapMarker {
            width,
            height,
        }),
    );
}

fn display_map_on_switch(mut events: EventReader<MapSwitchEvent>, mut map_query: Query<(&MapMarker, &mut Handle<Image>)>, shared_state: Res<SharedState>, asset_server: Res<AssetServer>) {
    if !events.is_empty() {
        // Event was fired update map

        // Delete all existing

        // // Display Map
        let (_, mut map_texture) = map_query.get_single_mut().expect("Couldnt find map");

        *map_texture = asset_server.load(shared_state.current_map.as_ref().unwrap().url.as_str());
        
        events.clear();
    }
}
