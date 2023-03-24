#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::utils


@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

@group(1) @binding(2)
var<uniform> light_sources: LightSources;

@group(1) @binding(3)
var<uniform> occluders: Occluders;  


const MAX_LIGHTS = 64u;
const MAX_OCCLUDERS = 64u;

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

struct Occluders {
    occluders: array<WrappedVec4, MAX_OCCLUDERS>,
    exists: array<WrappedBool, MAX_OCCLUDERS>,
}

fn sdCircle(p: vec2<f32>, r: f32) -> f32 {
  return length(p) - r;
}

fn sdf(p: vec2<f32>, q: vec2<f32>) -> f32 {
    let a = pow((p.x - q.x) ,2.0);
    let b = pow((p.y - q.y) ,2.0);
    return sqrt(a + b);
}



fn intersects(A: vec2<f32>, B: vec2<f32>, C: vec2<f32>, D: vec2<f32>) -> bool {
// calculate the direction of the lines
  var uA = ((D.x-C.x)*(A.y-C.y) - (D.y-C.y)*(A.x-C.x)) / ((D.y-C.y)*(B.x-A.x) - (D.x-C.x)*(B.y-A.y));
  var uB = ((B.x-A.x)*(A.y-C.y) - (B.y-A.y)*(A.x-C.x)) / ((D.y-C.y)*(B.x-A.x) - (D.x-C.x)*(B.y-A.y));

  // if uA and uB are between 0-1, lines are colliding
  if (uA >= 0.0 && uA <= 1.0 && uB >= 0.0 && uB <= 1.0) {

    // optionally, draw a circle where the lines meet
    // float intersectionX = A.x + (uA * (B.x-A.x));
    // float intersectionY = A.y + (uA * (B.y-A.y));
  

    return true;
  }
  return false;
}

// x1, y1, x2, y2
fn line_intersects_rect(a: vec2<f32>, b: vec2<f32>, rect: vec4<f32>) -> bool {
    
    if(intersects(a, b, vec2<f32>(rect.x, rect.z), vec2<f32>(rect.x, rect.w))) {
        return true;
    }
    if(intersects(a, b, vec2<f32>(rect.x, rect.w), vec2<f32>(rect.y, rect.w))) {
        return true;
    }
    if(intersects(a, b, vec2<f32>(rect.y, rect.w), vec2<f32>(rect.y, rect.z))) {
        return true;
    }
    if(intersects(a, b, vec2<f32>(rect.y, rect.z), vec2<f32>(rect.x, rect.z))) {
        return true;
    }

    return false;
}



@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    // Get screen position with coordinates from 0 to 1
    
    var color: vec4<f32> = textureSample(texture, our_sampler, uv);

    let position : vec2<f32> = world_position.xy;
    // x1, x2, y1, y2
    let occluder : vec4<f32> = vec4<f32>(-50.0, 50.0, -50.0, 50.0);

    //    if world_position.x > occluder.x && world_position.x < occluder.y &&
    //    world_position.y > occluder.z && world_position.y < occluder.w {
    //     // If it is, color it black
    //     return vec4<f32>(0.0, 0.0, 0.0, 0.9);
    // }


    var final_color: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 0.9);
    // Iterate through all the light sources
    for (var i = 0u; i < MAX_LIGHTS; i = i + 1u) {
        let light_position: vec2<f32> = light_sources.positions[i].value;
        let light_color: vec4<f32> = light_sources.colors[i].value;
        let light_intensity: f32 = light_sources.intensities[i].value;
        let light_radius: f32 = light_sources.radiuses[i].value;
        var light_active: u32 = light_sources.is_active[i].value;

        if(light_active != 0u) {
            let size = textureDimensions(texture);
            // let scaled_point = light_position * view.projection
            let projected_point = vec4<f32>(light_position.x / f32(size.x), light_position.y / f32(size.y), 0.0, 1.0);
            let world_point = projected_point;
            let worldToLight = light_position.xy - world_position.xy;

            let lightDistance = length(worldToLight);
            let dir = normalize(worldToLight);
            let attenuation = 1.0 / (lightDistance * lightDistance);

            // Check if light's line of sight intersects the occluder's rectangle
            var is_light_occluded = false;
            for(var j = 0u; j < MAX_OCCLUDERS; j = j + 1u) {
                let occluder = occluders.occluders[j].value;
                let occluder_exists = occluders.exists[j].value;
                if(occluder_exists != 0u) {
                    if(line_intersects_rect(world_position.xy, light_position.xy, occluder)) {
                        is_light_occluded = true;
                        break;
                    }
                }
            }

            if(!is_light_occluded) {
                   final_color = vec4<f32>(color.rgb * attenuation, 0.0);
              }
        }
    }
    return final_color;
}
