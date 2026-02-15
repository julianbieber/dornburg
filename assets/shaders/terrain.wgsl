struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(2) uv: vec2<f32>,
}

struct CustomMaterial {
    player: vec4f,
}
struct CustomMaterialI {
    v: vec4i,
}

@group(2) @binding(0) var height_texture: texture_2d<f32>;
@group(2) @binding(1) var height_texture_sampler: sampler;
@group(2) @binding(2) var time_texture: texture_2d<f32>;
@group(2) @binding(3) var time_texture_sampler: sampler;
@group(2) @binding(4) var kill_texture: texture_2d<f32>;
@group(2) @binding(5) var kill_texture_sampler: sampler;
@group(2) @binding(6) var<uniform> player_position: CustomMaterial;
@group(2) @binding(7) var<uniform> f1_position: CustomMaterial;
@group(2) @binding(8) var<uniform> level: CustomMaterialI;


fn cos_s(x: vec3f) -> vec3f {
    return (cos(x) * 0.5) + 0.5;
}

fn color(base: vec3f, scale: vec3f, freq: vec3f, freq_o: vec3f, x: f32) -> vec3f {
    return base + scale * cos(freq * x + freq_o);
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
        p = p * rot(vec3(time, time, 1.4 * time * 1.0 / f32(i + 1)));
        p += 1.0;
        a += dot(sin(p), sin(p.yzx));
        a *= 0.5;
    }

    return a;
}

fn dotnoise_static(x: vec3f) -> f32 {
    var a = 0.0;
    var p = x;
    for (var i: i32 = 0; i < 4; i = i + 1) {
        p = p * rot(vec3(0.2, 0.3, 1.4 * 0.3 * 1.0 / f32(i + 1)));
        a += dot(sin(p), sin(p.yzx));
    }

    return a / 4.0;
}

fn billow(v: vec3f, time: f32) -> f32 {
    let lun = 2.0;
    let dmp = 0.5;
    var a = 1.0;
    var t = 0.0;

    for (var i = 0; i < 3; i = i + 1) {
        t += abs(a * dotnoise(v * lun, time));
        a *= dmp;
    }
    return t;
}

fn density_color(d: f32, p: vec3f) -> vec3f {
    let c = pow(2.3, -d);

    var out_c = vec3f(0.0);
    for (var i = 0; i < level.v.x+1; i = i + 1) {
        out_c += color(vec3f(0.0), vec3f(0.1, 0.4, 0.3), vec3f(1.0, 2.0, 3.0), vec3f(0.0), billow(out_c + c, 0.0));
        out_c *= rot(p);
    }

    return abs(tanh(out_c * c));
}

fn fold(p: vec3f, n: vec3f, d:f32) -> vec3f {
    let dist = dot(p, n)+d;
    if dist < 0.0 {
        return -2.0 * dist * n + p;
    } else {
        return p;
    }
}

fn map(p: vec3f, time: f32) -> f32 {
    let s = 30.0;
    let id = round(p/s);
    var r = p - s*id;
    r = fold(r, normalize(r), 0.1);

    var m = 10000.0; //length(r) - 1.0 - sin(length(id))*3.0;


    var kif_p  = r;
    for (var i = 0; i< level.v.x + 1; i = i + 1) {
        kif_p = fold(kif_p, normalize(r), 0.2);
        kif_p *= rot(vec3f(0.1, 0.2, 0.3)*p+time);
        kif_p += 0.5;
        let l = length(kif_p) - 0.5;
        m = min(l, m);
    }


    
    return m*0.9;
}

fn march(ro: vec3f, rd: vec3f, time: f32) -> vec3f {
    var t: f32 = 0.0;
    var acc: vec3f = vec3f(0.0);

    for (var i: i32 = 0; i < 100; i = i + 1) {
        let p: vec3f = ro + rd * t;
        let d: f32 = map(p+player_position.player.xyz*0.02, time);

        if d < 0.001 {
            acc += density_color(abs(d), p+time+player_position.player.xyz*0.001);
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
    let ro: vec3f = vec3f(0.0, 0.0, -10.0);

    var rd: vec3f = vec3f(uv.x, uv.y, 0.0);
    rd = rd - vec3f(0.5, 0.5, 0.0);
    rd = rd * 2.0;
    rd.y = rd.y * -1.0;
    rd.x = rd.x * (iResolution.x / iResolution.y);
    rd.z = 1.0;
    rd = normalize(rd);

    return march(ro, rd, time);
}


fn line(coord: f32, spacing: f32, thickness: f32) -> bool {
    return step((abs(coord)) % spacing, thickness) == 0.0;
}

// the closer we are to a finish, the more brown the wall; furhter away more grey
// we need parallel horizontal lines and offset vertical lines 
fn wall(
    finish_distance: f32,
    time: f32,
    player_distance: f32,
    world_position: vec2f
) -> vec3f {

    let s = vec2f(34.0, 15.0);
    let offset_block_id = round(world_position / s);
    let x = world_position - s * offset_block_id;
    var modifier = 0.0;
    if line(world_position.y, 15.0, 12.0) || line(world_position.x + x.x - offset_block_id.y, 15.0, 11.0) {
        modifier = 0.3;
    } else {
        modifier = 1.0;
    }

    let earth_color = color(
        vec3f(
            0.0,
            0.0,
            0.0,
        ),
        vec3f(
            0.49555808,
            0.226957,
            0.0,
        ),
        vec3f(
            0.71472687,
            0.7101762,
            0.34810132,
        ),
        vec3f(
            0.52731574,
            0.4745929,
            0.030067481,
        ),
        0.0
    );

    let wall_color = color(
        vec3f(
            0.0,
            0.0,
            0.0,
        ),
        vec3f(
            0.21543397,
            0.21561927,
            0.21857274,
        ),
        vec3f(
            0.7583676,
            0.7664614,
            0.76142216,
        ),
        vec3f(
            0.0,
            0.0,
            0.0,
        ),
        0.0
    );

    return mix(earth_color, wall_color, 1.0 - (finish_distance)) * modifier;
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


    let f = min(f3_d, min(f1_d, f2_d));
    let f_d = 1.0 - smoothstep(0.0, 1280.0, f);

    if distance_to_player > 350.0 {
        return vec4f(0.0, 0.0, 0.0, 1.0);
    }
    var uv = mesh.uv;
    let time = textureSample(time_texture, time_texture_sampler, mesh.uv.yx).r;
    let r = distance_to_player / 200.0-1.0;
    uv += (sin(vec2(dotnoise(vec3(time), 0.0), dotnoise(vec3(time), 1.0))) * 0.013) * clamp(r, 0.0, 1.0);
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
            dotnoise(vec3f(mesh.world_position.x * 0.02, mesh.world_position.y * 0.02, time * 10.0), time)
        ) * (1.0 - (distance_to_player / 400.0)), 1.0);
    }

        // return vec4<f32>(wall(f_d, time, distance_to_player, mesh.world_position.xy), 1.0);
    if is_set {
        return vec4<f32>(wall(f_d, time, distance_to_player, mesh.world_position.xy), 1.0);
    } else {
        return vec4f(background(uv, vec2f(1280.0, 1280.0), time * 0.2), 1.0) * (1.0 - (distance_to_player / 400.0));
    }
}
