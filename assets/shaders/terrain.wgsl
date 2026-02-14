struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(2) uv: vec2<f32>,
}

struct CustomMaterial {
    player: vec4f,
}

@group(2) @binding(0) var height_texture: texture_2d<f32>;
@group(2) @binding(1) var height_texture_sampler: sampler;
@group(2) @binding(2) var time_texture: texture_2d<f32>;
@group(2) @binding(3) var time_texture_sampler: sampler;
@group(2) @binding(4) var kill_texture: texture_2d<f32>;
@group(2) @binding(5) var kill_texture_sampler: sampler;
@group(2) @binding(6) var<uniform> player_position: CustomMaterial;
@group(2) @binding(7) var<uniform> f1_position: CustomMaterial;


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

fn dotnoise(x: vec3f, time: f32) -> f32 {
    var a = 0.0;
    var p = x;
    for (var i: i32 = 0; i < 4; i = i + 1) {
        p = p * rot(vec3(time, time, 1.4*time * 1.0/f32(i+1)));
        p+=1.0;
        a += dot(sin(p), sin(p.yzx));
        a *= 0.5;
    }

    return a;
}

fn dotnoise_static(x: vec3f ) -> f32 {
    var a = 0.0;
    var p = x;
    for (var i: i32 = 0; i < 4; i = i + 1) {
        p = p * rot(vec3(0.2, 0.3, 1.4*0.3 * 1.0/f32(i+1)));
        a += dot(sin(p), sin(p.yzx));
    }

    return a / 4.0;
}

fn billow(v: vec3f, time: f32) -> f32 {
    let lun = 2.0;
    let dmp = 0.5;
    var a = 1.0;
    var t = 0.0;

    for (var i = 0; i < 3; i = i+1) {
        t += abs(a*dotnoise(v*lun, time));
        a *= dmp;
        
    }
    return t;
}

fn density_color(d: f32) -> vec3f {
    let c = pow(2.3, -d);

    var out_c = vec3f(0.0);
    for (var i = 0; i < 4; i = i+ 1) {
        out_c += color(vec3f(0.0), vec3f(0.1, 0.4, 0.3), vec3f(1.0, 2.0, 3.0), vec3f(0.0), billow(out_c+c, 0.0));

    }

    return abs(tanh(out_c * c));
    
}

fn march(ro: vec3f, rd: vec3f, time: f32) -> vec3f {
    var t: f32 = 0.0;
    var acc: vec3f= vec3f(0.0);

    for (var i: i32 = 0; i < 100; i = i + 1) {
        let p: vec3f = ro + rd * t;
        let d: f32 = billow(p, time);

        if (d < 1.01) {
            acc += density_color(abs(d));
            acc *= length(tanh(acc)) * 0.3;
        }

        t = t + abs(d);
    }


    return acc;
}

fn background(
    uv: vec2f,
    iResolution: vec2f,
    time: f32,
) -> vec3f {
    let ro: vec3f = vec3f(time*time, time, -10.0);

    var rd: vec3f = vec3f(uv.x, uv.y, 0.0);
    rd = rd - vec3f(0.5, 0.5, 0.0);
    rd = rd * 2.0;
    rd.y = rd.y * -1.0;
    rd.x = rd.x * (iResolution.x / iResolution.y);
    rd.z = 1.0;
    rd = normalize(rd);

    return march(ro, rd, time);
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let distance_to_player = (length(player_position.player.xy - mesh.world_position.xy));

    let f1 = player_position.player.zw;
    let f2 = f1_position.player.xy;
    let f3 = f1_position.player.zw;

    let f1_d = length(mesh.world_position.xy - f1);
    let f2_d = length(mesh.world_position.xy - f2);
    let f3_d = length(mesh.world_position.xy - f3);


    let f_d = ((1280.0*2.0) - min(f3_d, min(f1_d, f2_d)))/1280.0;
    
    if distance_to_player > 350.0 {
        return vec4f(0.0, 0.0, 0.0, 1.0);
    }
    var uv = mesh.uv;
    let time = textureSample(time_texture, time_texture_sampler, mesh.uv.yx).r;
    let r = distance_to_player / 200.0-1.0;
    uv += (sin(vec2(dotnoise(vec3(time), 0.0), dotnoise(vec3(time), 1.0)))*0.013 )* clamp(r, 0.0, 1.0);
    let is_set = textureSample(height_texture, height_texture_sampler, uv.yx).r > 0.5;
    let kill = textureSample(kill_texture, kill_texture_sampler, uv.yx).r > 0.5;

    if kill {
        return vec4<f32>(color(
            vec3f(
                0.26518637,
                0.0,
                0.0,
            ),
            vec3f(
                0.95800865,
                0.30069998,
                0.0,
            ),
            vec3f(
                0.8457962,
                0.83376676,
                0.11107275,
            ),
            vec3f(
                0.0
            ),
            dotnoise(vec3f(mesh.world_position.x * 0.02, mesh.world_position.y * 0.02, time*10.0), time)
        )* (1.0 - (distance_to_player / 400.0)), 1.0);
    }

    if is_set {
        return vec4<f32>(color(
            vec3f(
                0.0568133,
                0.015539987,
                0.0,
            ),
            vec3f(
                0.2,
                0.0,
                0.21633771,
            ),
            vec3f(
                0.9478618,
                0.43700445,
                1.0,
            ),
            vec3f(
            0.0,0.0,f_d
            ),
            dotnoise(vec3f(mesh.world_position.x * 0.002, mesh.world_position.y * 0.002, time), time)
        )*(1.0 - (distance_to_player / 400.0)), 1.0);
    } else {
        return vec4f(background(mesh.uv, vec2f(1280.0, 1280.0), time*0.2), 1.0) * (1.0 - (distance_to_player / 400.0));
    }
}
