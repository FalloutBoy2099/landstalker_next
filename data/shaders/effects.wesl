// The graphic effects over the finished picture (src/effects.rs): the light of the flames, the glow that Bevy's bloom takes up, a colour grade and a vignette.
// The maths are done in the screen's own (gamma) colours, which is what the pixel art was drawn in.

import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;

struct Effects {
    // flame light, glow (for the bloom), vignette, colour grade (1 on, 0 off)
    toggles: vec4<f32>,
    // vignette strength, brightness from which a flame glows, how much over 1.0 it is taken, time in seconds
    params: vec4<f32>,
    // the game area in pixels: x, y, width, height
    area: vec4<f32>,
    // colour of the flames' light, and its strength
    light_color: vec4<f32>,
    // number of lights
    info: vec4<f32>,
    // the dark of the room: red, green, blue, and how much of it the unlit parts take
    night: vec4<f32>,
    // the ambient light: its colour, and how much of it is left (1: all)
    ambient: vec4<f32>,
    // colour grade: shadows pushed to blue, warmth of the highlights, contrast, saturation
    grade: vec4<f32>,
    // each light: x, y in pixels, radius in pixels, brightness (intensity times flicker)
    lights: array<vec4<f32>, 48>,
    // each light: its colour
    colors: array<vec4<f32>, 48>,
};

@group(0) @binding(0) var screen: texture_2d<f32>;
@group(0) @binding(1) var samp: sampler;
@group(0) @binding(2) var<uniform> fx: Effects;

fn to_gamma(c: vec3<f32>) -> vec3<f32> {
    return pow(max(c, vec3<f32>(0.0)), vec3<f32>(1.0 / 2.2));
}

fn to_linear(c: vec3<f32>) -> vec3<f32> {
    return pow(max(c, vec3<f32>(0.0)), vec3<f32>(2.2));
}

fn luma(c: vec3<f32>) -> f32 {
    return dot(c, vec3<f32>(0.299, 0.587, 0.114));
}

// A different value of noise at each pixel (interleaved gradient noise): it breaks the bands of the gradients.
fn noise(p: vec2<f32>) -> f32 {
    return fract(52.9829189 * fract(dot(p, vec2<f32>(0.06711056, 0.00583715))));
}

// How much a colour is a flame: bright, and saturated (the white of a wall is bright and not saturated, so it does not glow).
fn glow_weight(c: vec3<f32>) -> f32 {
    let top = max(c.r, max(c.g, c.b));
    let low = min(c.r, min(c.g, c.b));
    let sat = (top - low) / max(top, 0.0001);
    let t = fx.params.y;
    return smoothstep(t - 0.06, t + 0.04, top) * smoothstep(0.3, 0.6, sat);
}

// The top of the colour range is rolled off softly instead of being cut: the strong lights do not burn the picture into flat yellow.
fn shoulder(c: vec3<f32>) -> vec3<f32> {
    let over = max(c - vec3<f32>(0.78), vec3<f32>(0.0));
    return min(c, vec3<f32>(0.78)) + over / (1.0 + over * 3.2);
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let px = in.position.xy;
    let n = noise(px);
    let source = textureSample(screen, samp, in.uv).rgb;
    let base = to_gamma(source);
    var c = base;
    // how near the pixel is to a flame (0 far, 1 at its heart): only there something can glow
    var near = 0.0;
    for (var i = 0; i < i32(fx.info.x); i = i + 1) {
        let l = fx.lights[i];
        near = max(near, 1.0 - distance(px, l.xy) / (l.z * 0.22));
    }
    near = clamp(near, 0.0, 1.0);

    // the light of the flames: the room is darker and tinted by the kind of place, and each flame lights what is near it, in its own colour
    if (fx.toggles.x > 0.5 && fx.info.x > 0.5) {
        var light = vec3<f32>(0.0);
        var glow = vec3<f32>(0.0);
        let count = i32(fx.info.x);
        for (var i = 0; i < count; i = i + 1) {
            let l = fx.lights[i];
            let off = px - l.xy;
            // on the floor a light makes an oval (the floor is seen at a slant: twice as wide as high), on the walls it is round and tighter: both together
            let d_floor = length(vec2<f32>(off.x, off.y * 1.9)) / l.z;
            let d_wall = length(off) / (l.z * 0.75);
            let d = min(d_floor, d_wall);
            let tone_here = fx.colors[i].rgb;
            if (d < 1.0) {
                // a soft fall-off that reaches zero at the edge of the radius
                let f = (1.0 - d) * (1.0 - d) / (1.0 + 6.0 * d);
                // yellow-white at the heart, the colour of the flame further out
                let tone = mix(vec3<f32>(1.0, 0.76, 0.5), tone_here, clamp(d * 1.6, 0.0, 1.0));
                light = light + tone * f * l.w;
            }
            // the heart of the flame glows over what is there (a small disc, much tighter than the light)
            let heart = max(0.0, 1.0 - distance(px, l.xy) / (l.z * 0.16));
            glow = glow + tone_here * heart * heart * l.w;
        }
        let ambient = fx.ambient.rgb * fx.ambient.w;
        let strength = fx.light_color.w * 2.0;
        c = base * (ambient + light * strength);
        // the dark of the room: where no light reaches, the picture takes a dark bluish tone
        let lit = clamp(max(light.r, max(light.g, light.b)) * strength * 2.0, 0.0, 1.0);
        c = mix(c, fx.night.rgb + c * 0.55, fx.night.w * (1.0 - lit));
        // the air around a flame glows a little over what is dark
        c = c + glow * 0.07 * min(fx.light_color.w * 3.0, 1.0);
    } else if (fx.toggles.x > 0.5) {
        c = base * fx.ambient.rgb * fx.ambient.w;
    }

    // the colour grade: the shadows go to blue / cyan, the highlights stay warm, a little more contrast and colour
    if (fx.toggles.w > 0.5) {
        let l = luma(c);
        let dark = 1.0 - smoothstep(0.0, 0.55, l);
        let bright = smoothstep(0.45, 0.95, l);
        c = c + dark * vec3<f32>(-0.03, 0.02, 0.075) * fx.grade.x;
        c = c * mix(vec3<f32>(1.0), vec3<f32>(1.0 + fx.grade.y, 1.0, 1.0 - fx.grade.y * 1.6), bright);
        c = (c - vec3<f32>(0.5)) * fx.grade.z + vec3<f32>(0.5);
        let g = luma(c);
        c = mix(vec3<f32>(g), c, fx.grade.w);
    }

    // the vignette: the corners of the game area are darker, the middle is left alone
    if (fx.toggles.z > 0.5) {
        let a = (px - fx.area.xy) / fx.area.zw;
        if (a.x >= 0.0 && a.x <= 1.0 && a.y >= 0.0 && a.y <= 1.0) {
            let p = (a - vec2<f32>(0.5)) * 2.0;
            let v = smoothstep(0.6, 1.5, length(p));
            c = c * (1.0 - fx.params.x * v);
        }
    }

    c = shoulder(c);
    // a little noise before the colours are cut to 8 bits: the soft gradients of the light would show bands
    c = c + vec3<f32>((n - 0.5) / 255.0);
    var out_color = to_linear(clamp(c, vec3<f32>(0.0), vec3<f32>(1.0)));

    // the glow: the flames of the original picture are taken over 1.0 (the camera is HDR); Bevy's bloom takes up what is over its threshold, and only that
    if (fx.toggles.y > 0.5) {
        out_color = out_color + source * glow_weight(base) * near * fx.params.z;
    }
    return vec4<f32>(out_color, 1.0);
}
