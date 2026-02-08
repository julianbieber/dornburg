struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(2) uv: vec2<f32>,
}

@group(2) @binding(0) var height_texture: texture_2d<f32>;
@group(2) @binding(1) var height_texture_sampler: sampler;


fn cos_s(x: vec3f) -> vec3f {
    return (cos(x) * 0.5) + 0.5;
}

fn color(base: vec3f, scale: vec3f, freq: vec3f, freq_o: vec3f, x: f32) -> vec3f {
    return base + scale * cos_s(freq * x + freq_o);
}

fn rot(r: vec3<f32>) -> mat3x3<f32> {
    let rx = mat3x3<f32>(
        1.0, 0.0, 0.0,
        0.0, cos(r.x), -sin(r.x),
        0.0, sin(r.x), cos(r.x)
    );
    let ry = mat3x3<f32>(
        cos(r.y), 0.0, -sin(r.y),
        0.0, 1.0, 0.0,
        sin(r.y), 0.0, cos(r.y)
    );
    let rz = mat3x3<f32>(
        cos(r.z), -sin(r.z), 0.0,
        sin(r.z), cos(r.z), 0.0,
        0.0, 0.0, 1.0
    );
    return rx * ry * rz;
}

fn dotnoise(x: vec3f) -> f32 {
    var a = 0.0;
    var p = x;
    for (var i: i32 = 0; i < 4; i = i + 1) {
        p = p * rot(vec3f(0.2,0.3,0.4));
        a += dot(cos(p), cos(p.yzx));
    }

    return a;
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let is_set = textureSample(height_texture, height_texture_sampler, mesh.uv.yx).r > 0.5;

    if is_set {
        return vec4<f32>(color(
            vec3f(
                0.0568133,
                0.015539987,
                0.0,
            ),
            vec3f(
                0.0,
                0.0,
                0.21633771,
            ),
            vec3f(
                0.9478618,
                0.43700445,
                1.0,
            ),
            vec3f(
                0.0
            ),
            dotnoise(vec3f(mesh.world_position.x, mesh.world_position.y, 0.0)*0.002)
        ), 1.0);
    } else {
        return vec4<f32>(0.0);
    }
}
