//! A custom post processing effect, using two cameras, with one reusing the render texture of the first one.
//! Here a chromatic aberration is applied to a 3d scene containing a rotating cube.
//! This example is useful to implement your own post-processing effect such as
//! edge detection, blur, pixelization, vignette... and countless others.

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::RenderTarget,
        render_resource::{
            AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureDimension, TextureFormat,
            TextureUsages, ShaderType, OwnedBindingResource, encase,
        },
        texture::BevyDefault,
        view::RenderLayers, RenderApp, RenderSet, Extract, renderer::RenderQueue,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, RenderMaterials2d}, transform, 
};
use bevy_pancam::PanCam;

use crate::{camera::{MainCamera, setup_camera}, map::MapMarker};

use super::LightSource;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CameraSet {
    CameraSetup,
    LightingSetup
}

pub struct LightingPostprocessPlugin;

impl Plugin for LightingPostprocessPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<LightingMaterial>::default())
            .add_startup_system(setup.in_set(CameraSet::LightingSetup).after(CameraSet::CameraSetup));

        app.sub_app_mut(RenderApp)
            .add_system(extract_lights.in_schedule(ExtractSchedule).in_set(RenderSet::ExtractCommands))
            .add_system(prepare_light_material.in_set(RenderSet::Prepare));
    }
}

#[derive(Component)]
pub struct OriginalCamera;

fn extract_lights(
    mut commands: Commands,
    mut light_q: Extract<Query<(Entity, &LightSource)>>, 
    mut materials_q: Extract<Query<(Entity, &Handle<LightingMaterial>)>>,
    mut cam_info: Extract<Query<(Entity, &PanCam, &Transform, &GlobalTransform, &OrthographicProjection, &Camera)>>,
    mut map_q: Extract<Query<(Entity, &MapMarker, &GlobalTransform)>>
) {
    for (entity, light_source) in &light_q {
        commands.get_or_spawn(entity)
            .insert(*light_source);
    }

    for (entity, material) in &materials_q {
        commands.get_or_spawn(entity)
            .insert(material.clone());
    }

    for (entity, pan_cam, trans, global_trans, proj, camera) in &cam_info {
        commands.get_or_spawn(entity)
            .insert(OriginalCamera)
            .insert(*trans)
            .insert(proj.clone())
            .insert(*global_trans)
            .insert(camera.clone());
    }

    for (entity, map_marker, global) in &map_q {
        commands.get_or_spawn(entity)
            .insert(*map_marker)
            .insert(*global);
    }
}

fn prepare_light_material(
    materials: Res<RenderMaterials2d<LightingMaterial>>,
    mut light_sources: Query<(&LightSource)>,
    mut material_q: Query<(&Handle<LightingMaterial>)>,
    mut camera_q: Query<(&OriginalCamera, &Transform, &GlobalTransform, &OrthographicProjection, &Camera)>,
    mut map_q: Query<(&MapMarker, &GlobalTransform)>,
    render_queue: Res<RenderQueue>,
) {
    if let Ok(map) = map_q.get_single() {
        let (marker, map_trans) = map;
    if let Ok(cam) = camera_q.get_single_mut() {
        let (p , cam_trans, global_trans, cam_proj, camera) = cam;

        for mat in material_q.iter_mut() {
            if let Some(mut light_material) = materials.get(mat) {
                let mut lighting_uniform_data = LightingMaterialUniformData {
                    colors: [WrappedVec4 {
                        value: Vec4::ZERO,
                    }; 64],
                    positions: [WrappedVec2 {
                        value: Vec2::ZERO,
                    }; 64],
                    intensities: [WrappedF32 {
                        value: 0.0,
                    }; 64],
                    radius: [WrappedF32 {
                        value: 0.0,
                    }; 64],
                    is_active: [WrappedBool {
                        value: 0,
                    }; 64]
                };
            
                let mut i = 0;
                for (light_source) in light_sources.iter_mut() {
                    let position_3d = Vec3::new(light_source.position.x * cam_proj.scale, light_source.position.y * cam_proj.scale  , 0.0);
                    let projection = Mat4::orthographic_rh(
                        cam_proj.area.min.x, 
                        cam_proj.area.max.x, 
                        cam_proj.area.min.y, 
                        cam_proj.area.max.y, 
                        cam_proj.far, 
                        cam_proj.near);

                    // let projection = camera.projection_matrix() * global_trans.compute_matrix().inverse();
                    // let mut position_on_screen = projection.project_point3(position_3d);
                    // let localMapPoint = (map_trans.compute_matrix().inverse() * projection.inverse()) * Vec4::new(position_on_screen.x, position_on_screen.y, 0.8, 1.0);
                    // let mesh_size = Vec2::new(marker.width as f32, marker.height as f32);
                    // let new_pos = camera.world_to_ndc(  global_trans, Vec3::new(light_source.position.x, light_source.position.y , 0.0)).unwrap();
                    // let local_map_space = (map_trans.compute_matrix().inverse() * projection.inverse()) * Vec4::new(new_pos.x, new_pos.y, 0.0, 1.0);
                    // let normalized_map_space = Vec2::new(new_pos.x, new_pos.y) / mesh_size;
                    
                    // // info!("light source position: {:?}", light_source.position);
                    // // info!("mesh size: {:?}", mesh_size);
                    // // info!("Light position: {:?}", new_pos);

                    // // info!("Light position 2: {:?}", position_on_screen);
                    // // info!("Light position 3: {:?}", normalized_map_space);

                    // let ndc_2d_pos = new_pos.truncate();
                    // let ndc_undone_cam = camera.projection_matrix().inverse() * Vec4::new(ndc_2d_pos.x, ndc_2d_pos.y, 0.0, 1.0);
                    // let ndc_undone_cam_point = Vec2::new(ndc_undone_cam.x, ndc_undone_cam.y) / ndc_undone_cam.w;
                    // let ndc_mesh_undone = map_trans.compute_matrix().inverse() * Vec4::new(ndc_undone_cam_point.x, ndc_undone_cam_point.y, 0.0, 1.0);
                    // let ndc_mesh_undone_point = Vec2::new(ndc_mesh_undone.x, ndc_mesh_undone.y) / ndc_mesh_undone.w;
                    // let normalized_local_point = (ndc_mesh_undone_point + mesh_size / 2.0) / mesh_size;

                    // info!("Light position 3: {:?}", normalized_local_point);

                    let light_ndc = camera.world_to_ndc(  global_trans, Vec3::new(light_source.position.x, light_source.position.y , 0.0)).unwrap().truncate();
                    let light_clip_space = Vec4::new(light_ndc.x, light_ndc.y, 0.0, 1.0);
                    let undo_camera_projection_matrix = camera.projection_matrix().inverse();
                    let undo_camera_projection = undo_camera_projection_matrix * light_clip_space;
                    let undo_map_model_matrix = map_trans.compute_matrix().inverse();
                    let undo_map_model = undo_map_model_matrix * undo_camera_projection;
                    let undo_map_model_point = Vec2::new(undo_map_model.x, undo_map_model.y) / undo_map_model.w;
                    let local_space_point = undo_map_model_point / Vec2::new(marker.width as f32, marker.height as f32);

                    info!("NDC: {:?}", light_ndc);
                    info!("Undone Camera Projection: {:?}", undo_camera_projection);
                    info!("Undone Map Projection: {:?}", undo_map_model);
                    info!("Undone Map Projection Point: {:?}", undo_map_model_point);
                    info!("Local Space Point: {:?}", local_space_point);

                    lighting_uniform_data.colors[i] = WrappedVec4 {
                        value: light_source.color,
                    };
                    lighting_uniform_data.positions[i] = WrappedVec2 {
                        value: local_space_point
                    };
                    lighting_uniform_data.intensities[i] = WrappedF32 {
                        value: light_source.intensity,
                    }; 
                    lighting_uniform_data.is_active[i] = WrappedBool {
                        value: light_source.is_active,
                    }; 
                    lighting_uniform_data.radius[i] = WrappedF32 {
                        value: light_source.radius,
                    }; 
                    
                    i += 1;
                }
                for binding in (&light_material.bindings).iter() {
                    if let OwnedBindingResource::Buffer(cur_buffer) = binding {
                        let mut buffer = encase::UniformBuffer::new(Vec::new());
                        buffer.write(&lighting_uniform_data).unwrap();
                        render_queue.write_buffer(cur_buffer, 0, buffer.as_ref());
                    }
                }
            }
        }
       
    }
}
}

fn setup(
    mut commands: Commands,
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut post_processing_materials: ResMut<Assets<LightingMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    // This assumes we only have a single window
    let window = windows.single();

    let size = Extent3d {
        width: window.resolution.physical_width(),
        height: window.resolution.physical_height(),
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    // // Main camera, first to render
    // commands.spawn((
    //     Camera3dBundle {
    //         camera_3d: Camera3d {
    //             clear_color: ClearColorConfig::Custom(Color::WHITE),
    //             ..default()
    //         },
    //         camera: Camera {
    //             target: RenderTarget::Image(image_handle.clone()),
    //             ..default()
    //         },
    //         transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
    //             .looking_at(Vec3::default(), Vec3::Y),
    //         ..default()
    //     },
    //     // Disable UI rendering for the first pass camera. This prevents double rendering of UI at
    //     // the cost of rendering the UI without any post processing effects.
    //     UiCameraConfig { show_ui: false },
    // ));

    // Setup Camera passing RenderTarget
    setup_camera(&mut commands, RenderTarget::Image(image_handle.clone()));




    // // This specifies the layer used for the post processing camera, which will be attached to the post processing camera and 2d quad.
    // let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

    // let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
    //     size.width as f32,
    //     size.height as f32,
    // ))));

    // // This material has the texture that has been rendered.
    // let material_handle = post_processing_materials.add(LightingMaterial {
    //     colors: [WrappedVec4 {
    //         value: Vec4::ZERO,
    //     }; 64],
    //     positions: [WrappedVec2 {
    //         value: Vec2::ZERO,
    //     }; 64],
    //     intensities: [WrappedF32 {
    //         value: 0.0,
    //     }; 64],
    //     radiuses: [WrappedF32 {
    //         value: 0.0,
    //     }; 64],
    //     is_active: [WrappedBool {
    //         value: 0,
    //     }; 64],
    //     source_image: image_handle,
    // });

    // // Post processing 2d quad, with material using the render texture done by the main camera, with a custom shader.
    // commands.spawn((
    //     MaterialMesh2dBundle {
    //         mesh: quad_handle.into(),
    //         material: material_handle,
    //         transform: Transform {
    //             translation: Vec3::new(0.0, 0.0, 1.5),
    //             ..default()
    //         },
    //         ..default()
    //     },
    //     post_processing_pass_layer,
    // ));

    // The post-processing pass camera.
    // commands.spawn((
    //     Camera2dBundle {
    //         camera: Camera {
    //             // renders after the first main camera which has default value: 0.
    //             order: 1,
    //             ..default()
    //         },
    //         ..Camera2dBundle::default()
    //     },
    //     post_processing_pass_layer,
    //     ));
}


/// Our custom post processing material
#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
pub struct LightingMaterial {
    /// In this example, this image will be the result of the main camera.

    #[texture(0)]
    #[sampler(1)]
    pub source_image: Handle<Image>,

    #[uniform(2)]
    pub positions: [WrappedVec2; 64],
    #[uniform(2)]
    pub colors: [WrappedVec4; 64],
    #[uniform(2)]
    pub intensities: [WrappedF32; 64],
    #[uniform(2)]
    pub radiuses: [WrappedF32; 64],
    #[uniform(2)]
    pub is_active: [WrappedBool; 64],
}

impl Material2d for LightingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/material_lighting.wgsl".into()
    }
}

#[derive(Clone, Copy, ShaderType)]
pub struct WrappedF32 {
    #[size(16)]
    pub value: f32,
}

#[derive(Clone, Copy, ShaderType)]
pub struct WrappedVec2 {
    #[size(16)]
    pub value: Vec2,
}

#[derive(Clone, Copy, ShaderType)]
pub struct WrappedVec4 {
    #[size(16)]
    pub value: Vec4,
}

#[derive(Clone, Copy, ShaderType)]
pub struct WrappedBool {
    #[size(16)]
    pub value: u32,
}

#[derive(Clone, ShaderType)]
pub struct LightingMaterialUniformData {
    pub positions: [WrappedVec2; 64],
    pub colors: [WrappedVec4; 64],
    pub intensities: [WrappedF32; 64],
    pub radius: [WrappedF32; 64],
    pub is_active: [WrappedBool; 64]
}