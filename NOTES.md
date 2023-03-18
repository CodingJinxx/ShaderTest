# Renderpipeline

1. Create a texture with all occluders

2. Fragment Shader for lighting
- sampler2d OccluderTexture
- vec<Lightsources> LightSources

2.1 Raymarch to Occluders, if blocked then disqualify light source
2.2 Calculate light level on pixel