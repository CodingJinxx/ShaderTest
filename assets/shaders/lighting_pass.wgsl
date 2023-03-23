// This shader computes the chromatic aberration effect

#import bevy_pbr::utils

// Since post processing is a fullscreen effect, we use the fullscreen vertex shader provided by bevy.
// This will import a vertex shader that renders a single fullscreen triangle.
//
// A fullscreen triangle is a single triangle that covers the entire screen.
// The box in the top left in that diagram is the screen. The 4 x are the corner of the screen
//
// Y axis
//  1 |  x-----x......
//  0 |  |  s  |  . ´
// -1 |  x_____x´
// -2 |  :  .´
// -3 |  :´
//    +---------------  X axis
//      -1  0  1  2  3
//
// As you can see, the triangle ends up bigger than the screen.
//
// You don't need to worry about this too much since bevy will compute the correct UVs for you.
#import bevy_core_pipeline::fullscreen_vertex_shader

@group(0) @binding(0)
var screen_texture: texture_2d<f32>;
@group(0) @binding(1)
var texture_sampler: sampler;

const MAX_LIGHTS: u32 = 32u;

struct LightSource {
    position: vec2<f32>,
    color: vec4<f32>,
    intensity: f32,
    radius: f32,
};

struct LightSourceArray {
    values: array<LightSource, MAX_LIGHTS>
};

@group(0) @binding(2)
var<uniform> light_sources: LightSourceArray;



@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    var color: vec4<f32> = textureSample(screen_texture, texture_sampler, in.uv);



    // Calculate the screen position of the fragment
    var screen_position: vec2<f32> = in.uv * vec2<f32>(textureDimensions(screen_texture));

    var light_intensity: f32 = 0.0;
    for (var i: u32 = 0u; i < MAX_LIGHTS; i = i + 1u) {
        let light: LightSource = light_sources.values[i];

        // Calculate the distance from the light source to the fragment
        let distance: f32 = length(position.xy - light.position);

        // Calculate the light influence based on the distance and light radius
        let influence: f32 = max(0.0, 1.0 - distance / light.radius);

        // Apply the light color, intensity, and influence to the fragment
        light_intensity = light.intensity * influence;
    }

    // Combine the original color and lit color, and preserve the alpha value
    return color * light_intensity;
}
