use bevy::{prelude::*, render::{render_resource::{AsBindGroup, ShaderType}, extract_component::ExtractComponent}};

#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType, Debug)]
pub struct LightSource {
    pub position: Vec2,
    pub color: Vec4,
    pub intensity: f32,
    pub radius: f32,
    pub is_active: u32
}

