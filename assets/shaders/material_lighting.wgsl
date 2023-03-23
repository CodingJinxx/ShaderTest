#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::utils

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

@group(1) @binding(2)
var<uniform> light_sources: LightSources;


const MAX_LIGHTS = 64u;

struct WrappedF32 {
    @size(16)
    value: f32,
};

struct WrappedVec2 {
    @size(16)
    value: vec2<f32>,
};

struct WrappedVec4 {
    @size(16)
    value: vec4<f32>,
};

struct WrappedBool{
    @size(16)
    value: u32,
}

struct LightSources {
    positions: array<WrappedVec2, MAX_LIGHTS>,
    colors: array<WrappedVec4, MAX_LIGHTS>,
    intensities: array<WrappedF32, MAX_LIGHTS>,
    radiuses: array<WrappedF32, MAX_LIGHTS>,
    is_active: array<WrappedBool, MAX_LIGHTS>,
};

fn sdCircle(p: vec2<f32>, r: f32) -> f32 {
  return length(p) - r;
}

fn sdf(p: vec2<f32>, q: vec2<f32>) -> f32 {
    let a = pow((p.x - q.x) ,2.0);
    let b = pow((p.y - q.y) ,2.0);
    return sqrt(a + b);
}

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    // Get screen position with coordinates from 0 to 1
    let uv = coords_to_viewport_uv(position.xy, view.viewport);

    var color: vec4<f32> = textureSample(texture, our_sampler, uv);


    var final_color: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    // Iterate through all the light sources
    for (var i = 0u; i < MAX_LIGHTS; i = i + 1u) {
        let light_position: vec2<f32> = light_sources.positions[i].value;
        let light_color: vec4<f32> = light_sources.colors[i].value;
        let light_intensity: f32 = light_sources.intensities[i].value;
        let light_radius: f32 = light_sources.radiuses[i].value;
        let light_active: u32 = light_sources.is_active[i].value;

        if(light_active == 0u) {
            continue;
        }

        let worldToLight = light_position - position.xy;

        let lightDistance = length(worldToLight);
        let dir = normalize(worldToLight);

        let radiance = light_color * (1.0 / pow(lightDistance, 2.0));

        final_color += radiance * color;
    }



    return vec4<f32>((final_color.rgb + (color.rgb * 0.1)), color.a);
}
