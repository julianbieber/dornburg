struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(2) uv: vec2<f32>,
}

@group(2) @binding(0) var height_texture: texture_2d<f32>;
@group(2) @binding(1) var height_texture_sampler: sampler;



@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    return vec4<f32>( textureSample(height_texture, height_texture_sampler, mesh.uv.yx).r);
}
