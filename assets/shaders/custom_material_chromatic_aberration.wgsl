#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::utils

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

// let MAX_LIGHTS: u32 = 32u;

// struct LightSource {
//     position: vec2<f32>,
//     color: vec4<f32>,
//     intensity: f32,
//     radius: f32,
// };

// struct LightSourceArray {
//     values: array<LightSource, MAX_LIGHTS>
// };

// @group(0) @binding(2)
// var<uniform> light_sources: LightSourceArray;


@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    // Get screen position with coordinates from 0 to 1
    let uv = coords_to_viewport_uv(position.xy, view.viewport);
    let offset_strength = 0.02;

    // Sample each color channel with an arbitrary shift
    var output_color = vec4<f32>(
        textureSample(texture, our_sampler, uv + vec2<f32>(offset_strength, -offset_strength)).r,
        textureSample(texture, our_sampler, uv + vec2<f32>(-offset_strength, 0.0)).g,
        textureSample(texture, our_sampler, uv + vec2<f32>(0.0, offset_strength)).b,
        1.0
    );

    return output_color;
}
