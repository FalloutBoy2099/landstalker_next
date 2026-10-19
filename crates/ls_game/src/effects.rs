//! The graphic effects over the picture of the game (landstalker.ini, [effects]; data/effects.yaml): the light of the flames, a bloom, a vignette and a colour
//! grade. The shadows under the objects are in shadows.rs.
//!
//! * The camera: HDR, a tone mapper and Bevy's bloom (`Bloom`, `Tonemapping`, `Hdr`), set by `drive_effects`; only while an effect is on.
//! * The lights: the objects of the layer `lights` of the room's `map.tmx` (written by tools/detect_effects.py, editable in Tiled): each one becomes an entity
//!   with a `PointLight2d` (type, colour, intensity) and a `Flicker`, which `flicker_lights` makes live. A room whose map has none uses the flame tiles of
//!   data/effects.yaml instead.
//! * One pass over the finished picture (data/shaders/effects.wesl) lights the room with them, makes the flames glow (over 1.0, which the bloom takes up),
//!   grades the colours and darkens the corners. The look depends on the kind of place (outdoor, interior, cave: `environments`, `rules` and `rooms` of
//!   data/effects.yaml).
//!
//! Each effect is turned off by its line of [effects]; with all of them off the pass is not even run, and the picture is the original's.

use crate::config::Config;
use crate::scripting::Scripting;
use crate::{RoomRes, SCALE};
use bevy::core_pipeline::fullscreen_material::{FullscreenMaterial, FullscreenMaterialPlugin};
use bevy::core_pipeline::tonemapping::{tonemapping, Tonemapping};
use bevy::core_pipeline::{Core2d, Core2dSystems};
use bevy::ecs::schedule::{ScheduleConfigs, ScheduleLabel};
use bevy::ecs::system::BoxedSystem;
use bevy::post_process::bloom::{bloom, Bloom, BloomCompositeMode, BloomPrefilter};
use bevy::prelude::*;
use bevy::render::{extract_component::ExtractComponent, render_resource::ShaderType, RenderApp};
use bevy::shader::ShaderRef;
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// How many lights the shader takes (the nearest to the middle of the screen are kept).
pub const MAX_LIGHTS: usize = 48;

/// How a kind of place is lit and graded (an entry of `environments` in data/effects.yaml).
#[derive(Clone, Debug)]
pub struct Environment {
    /// light left in the room (1: all of it) and its colour
    pub ambient: f32,
    pub tint: [f32; 3],
    /// what a flame does: strength of its light and how far it reaches (in cells)
    pub light_strength: f32,
    pub light_radius: f32,
    pub vignette: f32,
    pub bloom: f32,
    /// how far the dark parts are pushed to blue, how warm the bright parts are, contrast, saturation
    pub shadows: f32,
    pub warmth: f32,
    pub contrast: f32,
    pub saturation: f32,
    /// the dark of the room: its colour and how much of it the unlit parts take (0: none)
    pub night: [f32; 3],
    pub night_amount: f32,
}

impl Default for Environment {
    fn default() -> Self {
        Environment { ambient: 0.86, tint: [1.0, 0.93, 0.84], light_strength: 0.55, light_radius: 3.8, vignette: 0.28, bloom: 0.5, shadows: 0.2, warmth: 0.06, contrast: 1.04, saturation: 1.06, night: [0.063, 0.078, 0.125], night_amount: 0.0 }
    }
}

/// What data/effects.yaml says.
#[derive(Resource, Default)]
pub struct EffectsData {
    /// room -> the blocks whose picture is a flame, with where the flame is in the 16x16 tile (x, y) and how big it is (pixels): the lights of a room whose
    /// map has no `lights` layer
    tiles: HashMap<i32, HashMap<u32, [f32; 3]>>,
    /// the entity types that are flames
    entities: HashSet<i32>,
    /// colour of the light of a flame, how much it flickers (0..1) and how fast (Hz)
    color: [f32; 3],
    flicker: f32,
    speed: f32,
    /// the kinds of place by name, the rules that tell which one a room is (first match wins), and the rooms that are told by hand
    environments: HashMap<String, Environment>,
    rules: Vec<(String, Vec<String>)>,
    rooms: HashMap<i32, String>,
}

impl EffectsData {
    pub fn load(data: &Path) -> EffectsData {
        let tree = crate::yamldata::read(&data.join("effects.yaml"));
        let flame = tree.get("flame").cloned().unwrap_or_default();
        let mut out = EffectsData { color: [1.0, 0.54, 0.26], flicker: 0.3, speed: 7.0, ..Default::default() };
        if let Some(tiles) = flame.get("tiles") {
            for (room, list) in crate::yamldata::int_keys(tiles) {
                let blocks = crate::yamldata::int_keys(list)
                    .into_iter()
                    .map(|(b, v)| {
                        let n = |i: usize, d: f64| v.get(i).and_then(|x| x.as_f64()).unwrap_or(d) as f32;
                        (b as u32, [n(0, 8.0), n(1, 6.0), n(2, 30.0)])
                    })
                    .collect();
                out.tiles.insert(room, blocks);
            }
        }
        if let Some(entities) = flame.get("entities") {
            out.entities = crate::yamldata::int_keys(entities).into_iter().map(|(k, _)| k).collect();
        }
        if let Some(light) = flame.get("light") {
            let num = |k: &str, d: f32| light.get(k).and_then(|v| v.as_f64()).map_or(d, |v| v as f32);
            if let Some(c) = light.get("color").and_then(|c| c.as_array()) {
                for (i, v) in c.iter().take(3).enumerate() {
                    out.color[i] = (v.as_f64().unwrap_or(255.0) / 255.0) as f32;
                }
            }
            out.flicker = num("flicker", out.flicker);
            out.speed = num("speed", out.speed);
        }
        if let Some(envs) = tree.get("environments").and_then(|e| e.as_object()) {
            for (name, e) in envs {
                let d = Environment::default();
                let num = |k: &str, d: f32| e.get(k).and_then(|v| v.as_f64()).map_or(d, |v| v as f32);
                let mut tint = d.tint;
                if let Some(t) = e.get("tint").and_then(|t| t.as_array()) {
                    for (i, v) in t.iter().take(3).enumerate() {
                        tint[i] = v.as_f64().unwrap_or(1.0) as f32;
                    }
                }
                out.environments.insert(
                    name.clone(),
                    Environment {
                        ambient: num("ambient", d.ambient),
                        tint,
                        light_strength: num("light_strength", d.light_strength),
                        light_radius: num("light_radius", d.light_radius),
                        vignette: num("vignette", d.vignette),
                        bloom: num("bloom", d.bloom),
                        shadows: num("shadows", d.shadows),
                        warmth: num("warmth", d.warmth),
                        contrast: num("contrast", d.contrast),
                        saturation: num("saturation", d.saturation),
                        night: {
                            let mut c = d.night;
                            if let Some(v) = e.get("night").and_then(|t| t.as_array()) {
                                for (i, x) in v.iter().take(3).enumerate() {
                                    c[i] = (x.as_f64().unwrap_or(0.0) / 255.0) as f32;
                                }
                            }
                            c
                        },
                        night_amount: num("night_amount", d.night_amount),
                    },
                );
            }
        }
        if let Some(rules) = tree.get("rules").and_then(|r| r.as_array()) {
            for r in rules {
                let env = r.get("env").and_then(|e| e.as_str()).unwrap_or("interior").to_string();
                let words = r.get("path").and_then(|p| p.as_array()).map(|a| a.iter().filter_map(|w| w.as_str().map(|w| w.to_lowercase())).collect()).unwrap_or_default();
                out.rules.push((env, words));
            }
        }
        if let Some(rooms) = tree.get("rooms") {
            for (room, env) in crate::yamldata::int_keys(rooms) {
                if let Some(e) = env.as_str() {
                    out.rooms.insert(room, e.to_string());
                }
            }
        }
        out
    }

    /// The name of the kind of place of a room, by number and path in the game's tree.
    pub fn environment_name(&self, room: i32, path: &str) -> String {
        self.rooms.get(&room).cloned().unwrap_or_else(|| {
            let p = path.to_lowercase();
            self.rules.iter().find(|(_, words)| words.iter().any(|w| p.contains(w.as_str()))).map(|(e, _)| e.clone()).unwrap_or_else(|| "interior".to_string())
        })
    }

    /// The kind of place of a room, by number and path in the game's tree.
    pub fn environment_of(&self, room: i32, path: &str) -> Environment {
        self.environments.get(&self.environment_name(room, path)).cloned().unwrap_or_default()
    }
}

// ---------------------------------------------------------------------------------------------------------------
// the lights of a room: entities
// ---------------------------------------------------------------------------------------------------------------

/// A point light of a room (the position is the `Transform`: the pixels of the room, x right and y up like the sprites of the room). The shader lights the
/// picture with it; its brightness at this moment is `intensity * level`.
#[derive(Component, Clone, Debug)]
pub struct PointLight2d {
    /// `flame`...
    pub light_type: String,
    /// red, green, blue in 0..1
    pub color: [f32; 3],
    /// 1 = an ordinary flame
    pub intensity: f32,
    /// what the flicker makes of it now (about 1 - flicker .. 1)
    pub level: f32,
}

/// What makes a light live: how much it dims (0..1), how fast (radians a second) and where in its cycle it is.
#[derive(Component, Clone, Copy, Debug)]
pub struct Flicker {
    pub amount: f32,
    pub speed: f32,
    pub phase: f32,
}

/// The lights that belong to the room that is shown (they go with it).
#[derive(Component)]
pub struct RoomLight;

/// A flame never burns evenly: two quick waves and a slow one.
pub fn flicker_level(t: f32, f: &Flicker) -> f32 {
    let w = 0.5 * (t * f.speed + f.phase * 6.1).sin() + 0.3 * (t * f.speed * 2.3 + f.phase * 3.7).sin() + 0.2 * (t * f.speed * 0.37 + f.phase * 11.0).sin();
    1.0 - f.amount * (0.5 + 0.5 * w)
}

/// Makes the lights flicker.
pub fn flicker_lights(time: Res<Time>, mut lights: Query<(&mut PointLight2d, &Flicker)>) {
    let t = time.elapsed_secs();
    for (mut light, flicker) in &mut lights {
        light.level = flicker_level(t, flicker);
    }
}

/// The place of the point of the block grid (fractions allowed) of a room, in the pixels of the room: what `spawn_room` does for the top left of a block.
fn grid_to_pixels(room: &RoomRes, bx: f32, by: f32) -> Vec2 {
    Vec2::new(((bx - by + (room.block_h - 1) as f32) * 2.0 + room.hm_left as f32) * 8.0, (bx + by + room.hm_top as f32) * 8.0)
}

/// The lights of a room as (pixels of the room, light, flicker): the objects `type = "light"` of the layer `lights` of its map; a room without any gets the
/// flame tiles of data/effects.yaml (the tiles of one flame made one light).
fn lights_of(room: &RoomRes, data: &EffectsData) -> Vec<(Vec2, PointLight2d, Flicker)> {
    let mut out = Vec::new();
    for (n, o) in room.room.tmx.objects.iter().filter(|o| o.layer == "lights" && (o.kind == "light" || o.kind == "torch" || o.kind.is_empty())).enumerate() {
        let local = grid_to_pixels(room, o.x / 16.0, o.y / 16.0);
        let light = PointLight2d { light_type: o.properties.get("light_type").cloned().unwrap_or_else(|| "flame".to_string()), color: o.color("color", data.color), intensity: o.number("intensity", 1.0), level: 1.0 };
        out.push((local, light, Flicker { amount: o.number("flicker", data.flicker).clamp(0.0, 1.0), speed: data.speed * std::f32::consts::TAU, phase: n as f32 * 1.7 + o.x * 0.013 }));
    }
    if !out.is_empty() {
        return out;
    }
    let Some(set) = data.tiles.get(&room.room.file.id) else { return out };
    let size = room.room.file.map.blocks.size;
    let mut found: Vec<(Vec2, f32)> = Vec::new();
    for (layer, shift) in [("background", 16.0f32), ("foreground", 0.0)] {
        for by in 0..size[1] {
            for bx in 0..size[0] {
                if let Some(&[hx, hy, pixels]) = room.room.block(layer, bx, by).and_then(|b| set.get(&b)) {
                    found.push((grid_to_pixels(room, bx as f32, by as f32) + Vec2::new(shift + hx, hy), pixels));
                }
            }
        }
    }
    found.sort_by(|a, b| b.1.total_cmp(&a.1));
    let mut merged: Vec<(Vec2, f32)> = Vec::new();
    for (at, pixels) in found {
        if let Some(m) = merged.iter_mut().find(|m| m.0.distance(at) < 11.0) {
            let total = m.1 + pixels;
            m.0 = (m.0 * m.1 + at * pixels) / total;
            m.1 = total;
        } else {
            merged.push((at, pixels));
        }
    }
    for (n, (at, pixels)) in merged.into_iter().enumerate() {
        let light = PointLight2d { light_type: "flame".to_string(), color: data.color, intensity: (pixels / 40.0).sqrt().clamp(0.6, 1.5), level: 1.0 };
        out.push((at, light, Flicker { amount: data.flicker, speed: data.speed * std::f32::consts::TAU, phase: n as f32 * 1.7 + at.x * 0.013 }));
    }
    out
}

/// Builds the lights of the room when it changes: the old ones go, the new ones are made from the map (see `lights_of`).
pub fn spawn_room_lights(mut commands: Commands, room: Res<RoomRes>, data: Res<EffectsData>, old: Query<Entity, With<RoomLight>>, mut shown: Local<i32>) {
    let id = room.room.file.id;
    if *shown == id && !room.is_changed() {
        return;
    }
    *shown = id;
    for e in &old {
        commands.entity(e).despawn();
    }
    for (local, light, flicker) in lights_of(&room, &data) {
        commands.spawn((RoomLight, light, flicker, Transform::from_xyz(local.x, -local.y, 0.0)));
    }
}

// ---------------------------------------------------------------------------------------------------------------
// the pass over the picture
// ---------------------------------------------------------------------------------------------------------------

/// The settings of the pass, on the camera: what the shader reads (see effects.wesl).
#[derive(Component, ExtractComponent, Clone, Copy, ShaderType)]
#[extract_app(RenderApp)]
pub struct SceneEffects {
    /// flame light, glow (for the bloom), vignette, colour grade (1 on, 0 off)
    toggles: Vec4,
    /// vignette strength, brightness from which a flame glows, how much, time
    params: Vec4,
    /// the game area on the screen, in pixels: x, y, width, height
    area: Vec4,
    /// colour of the light of the flames, and its strength
    light_color: Vec4,
    /// number of lights
    info: Vec4,
    /// the dark of the room (red, green, blue) and how much of it the unlit parts take
    night: Vec4,
    /// the ambient light: its colour times how much is left of it
    ambient: Vec4,
    /// colour grade: how far the shadows are pushed to blue, warmth of the highlights, contrast, saturation
    grade: Vec4,
    /// each light: x and y in pixels on the screen, radius in pixels, brightness (intensity times flicker)
    lights: [Vec4; MAX_LIGHTS],
    /// each light: its colour
    colors: [Vec4; MAX_LIGHTS],
}

impl Default for SceneEffects {
    fn default() -> Self {
        SceneEffects { toggles: Vec4::ZERO, params: Vec4::ZERO, area: Vec4::ZERO, light_color: Vec4::ZERO, info: Vec4::ZERO, night: Vec4::ZERO, ambient: Vec4::ONE, grade: Vec4::new(0.0, 0.0, 1.0, 1.0), lights: [Vec4::ZERO; MAX_LIGHTS], colors: [Vec4::ONE; MAX_LIGHTS] }
    }
}

impl FullscreenMaterial for SceneEffects {
    fn fragment_shader() -> ShaderRef {
        "shaders/effects.wesl".into()
    }

    fn schedule() -> impl ScheduleLabel + Clone {
        Core2d
    }

    /// Before the bloom (the flames are made to glow first: over 1.0, which is what the bloom takes up) and the tone mapper.
    fn schedule_configs(system: ScheduleConfigs<BoxedSystem>) -> ScheduleConfigs<BoxedSystem> {
        system.in_set(Core2dSystems::PostProcess).before(bloom).before(tonemapping)
    }
}

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FullscreenMaterialPlugin::<SceneEffects>::default()).add_systems(Update, (spawn_room_lights, flicker_lights.after(spawn_room_lights))).add_systems(PostUpdate, drive_effects);
    }
}

/// The room being lit: its number and the kind of place.
#[derive(Default)]
pub struct Place {
    room: i32,
    environment: Environment,
}

/// The tone mapper of [effects] `tonemapping`.
fn tonemapper(name: &str) -> Tonemapping {
    match name {
        "reinhard" => Tonemapping::Reinhard,
        "reinhard_luminance" => Tonemapping::ReinhardLuminance,
        "tony" | "tonymcmapface" => Tonemapping::TonyMcMapface,
        _ => Tonemapping::None,
    }
}

#[allow(clippy::too_many_arguments)]
fn drive_effects(
    mut commands: Commands,
    config: Res<Config>,
    data: Res<EffectsData>,
    room: Res<RoomRes>,
    script: NonSend<Scripting>,
    title: Res<crate::title::Title>,
    time: Res<Time>,
    windows: Query<&Window>,
    lights: Query<(&PointLight2d, &Flicker, &Transform), With<RoomLight>>,
    cameras: Query<(Entity, &Transform, Option<&SceneEffects>, Option<&Bloom>, Option<&Tonemapping>), (With<Camera2d>, Without<RoomLight>)>,
    mut place: Local<Place>,
) {
    let Ok((camera, cam_tf, current, bloom_on, tone_now)) = cameras.single() else { return };
    let fx = &config.effects;
    let any = fx.flame_light || fx.bloom || fx.vignette || fx.color_grade;
    if !any || title.active {
        if current.is_some() {
            commands.entity(camera).remove::<(SceneEffects, Bloom)>();
        }
        return;
    }
    let Ok(window) = windows.single() else { return };
    let k = window.scale_factor();
    let (win_w, win_h) = (window.width(), window.height());
    // the game area: the window without the panel of the play test
    let area_w = win_w - if config.playtest_panel { crate::playtest::PANEL_W } else { 0.0 };
    // the kind of place is looked for once a room
    let id = room.room.file.id;
    // (read at every frame: the debug panel changes it while the game runs)
    place.room = id;
    place.environment = data.environment_of(id, &room.room.file.path);
    let env = place.environment.clone();
    // the world to the screen: the root of the room is scaled by SCALE, the camera is at `cam_tf`, the screen is the window
    let on_screen = |local: Vec2| -> Vec2 {
        let world = Vec2::new(local.x * SCALE, -local.y * SCALE);
        Vec2::new(win_w / 2.0 + (world.x - cam_tf.translation.x), win_h / 2.0 - (world.y - cam_tf.translation.y)) * k
    };
    let reach = env.light_radius * 24.0 * SCALE * k;
    let centre = Vec2::new(area_w, win_h) * 0.5 * k;
    // the lights on the screen: (distance to the middle, place, brightness, colour)
    let mut found: Vec<(f32, Vec2, f32, [f32; 3], f32)> = Vec::new();
    if fx.flame_light {
        // the flicker moves the opacity (the brightness) and the size (+- 8 %) of a light together
        let wave = |level: f32, amount: f32| ((level - (1.0 - amount * 0.5)) / (amount * 0.5).max(0.01)).clamp(-1.0, 1.0);
        let mut points: Vec<(Vec2, f32, [f32; 3], f32)> = lights.iter().map(|(l, f, t)| (Vec2::new(t.translation.x, -t.translation.y), l.intensity * l.level, l.color, 1.0 + 0.08 * wave(l.level, f.amount))).collect();
        // the flames that move (a fireball): entities of the room
        let t = time.elapsed_secs();
        for s in script.entities() {
            if s.visible && data.entities.contains(&s.kind) {
                let p = room.to_screen(s.x, s.y, s.z + 0.5);
                let level = flicker_level(t, &Flicker { amount: data.flicker, speed: data.speed * std::f32::consts::TAU, phase: s.index as f32 * 2.3 });
                points.push((Vec2::new(p.x, p.y), level, data.color, 1.0 + 0.08 * wave(level, data.flicker)));
            }
        }
        for (local, bright, color, size) in points {
            let at = on_screen(local);
            if at.x < -reach || at.y < -reach || at.x > area_w * k + reach || at.y > win_h * k + reach {
                continue;
            }
            found.push((at.distance(centre), at, bright, color, size));
        }
        found.sort_by(|a, b| a.0.total_cmp(&b.0));
        found.truncate(MAX_LIGHTS);
    }
    // [effects] intensity scales what the kind of place says
    let it = fx.intensity;
    let mut effect = SceneEffects {
        toggles: Vec4::new(fx.flame_light as u32 as f32, fx.bloom as u32 as f32, fx.vignette as u32 as f32, fx.color_grade as u32 as f32),
        params: Vec4::new((env.vignette * it).min(0.9), GLOW_FROM, GLOW_GAIN * (env.bloom * it).clamp(0.0, 2.0), time.elapsed_secs()),
        area: Vec4::new(0.0, 0.0, area_w * k, win_h * k),
        light_color: Vec4::new(data.color[0], data.color[1], data.color[2], env.light_strength * it),
        info: Vec4::new(found.len() as f32, 0.0, 0.0, 0.0),
        night: Vec4::new(env.night[0], env.night[1], env.night[2], (env.night_amount * it).clamp(0.0, 1.0)),
        ambient: Vec4::new(env.tint[0], env.tint[1], env.tint[2], 1.0 - (1.0 - env.ambient) * it),
        grade: Vec4::new(env.shadows * it, env.warmth * it, 1.0 + (env.contrast - 1.0) * it, 1.0 + (env.saturation - 1.0) * it),
        ..Default::default()
    };
    for (i, (_, at, bright, color, size)) in found.iter().enumerate() {
        effect.lights[i] = Vec4::new(at.x, at.y, reach * (0.75 + 0.25 * bright.min(1.5)) * size, *bright);
        effect.colors[i] = Vec4::new(color[0], color[1], color[2], 1.0);
    }
    let mut cam = commands.entity(camera);
    cam.insert(effect);
    // the camera: HDR (what `Bloom` asks for), the tone mapper, and the bloom of Bevy, which takes up what the pass made glow
    let tone = tonemapper(&fx.tonemapping);
    if tone_now != Some(&tone) {
        cam.insert(tone);
    }
    if fx.bloom {
        cam.insert(Bloom {
            intensity: (fx.bloom_intensity * it).clamp(0.0, 1.0),
            low_frequency_boost: 0.2,
            prefilter: BloomPrefilter { threshold: fx.bloom_threshold, threshold_softness: 0.2 },
            composite_mode: BloomCompositeMode::Additive,
            ..Bloom::OLD_SCHOOL
        });
    } else if bloom_on.is_some() {
        cam.remove::<Bloom>();
    }
}

/// The brightness (of the colours as they are on the screen) from which a pixel is a flame that glows, and how much over 1.0 the glow takes it.
const GLOW_FROM: f32 = 0.78;
const GLOW_GAIN: f32 = 3.0;

// ---------------------------------------------------------------------------------------------------------------
// the debug panel: the effects, set while the game runs
// ---------------------------------------------------------------------------------------------------------------

/// The effects and the cheats, in the debug panel: every effect with its switch and its strengths, and what the kind of place of the room says (the colours, the
/// light, the vignette...). What is changed shows at once. The buttons write it to landstalker.ini and to data/effects.yaml.
pub fn effects_ui(ui: &mut bevy_egui::egui::Ui, config: &mut Config, data: &mut EffectsData, room_id: i32, room_path: &str, status: &mut String) {
    use bevy_egui::egui;
    let name = data.environment_name(room_id, room_path);
    ui.label(egui::RichText::new("Effects and cheats").strong());
    let fx = &mut config.effects;
    ui.horizontal_wrapped(|ui| {
        ui.checkbox(&mut fx.object_shadows, "shadows");
        ui.checkbox(&mut fx.flame_light, "flame light");
        ui.checkbox(&mut fx.bloom, "bloom");
        ui.checkbox(&mut fx.vignette, "vignette");
        ui.checkbox(&mut fx.color_grade, "grade");
    });
    ui.add(egui::Slider::new(&mut fx.intensity, 0.0..=2.0).text("intensity"));
    ui.add(egui::Slider::new(&mut fx.bloom_threshold, 0.2..=1.5).text("bloom threshold"));
    ui.add(egui::Slider::new(&mut fx.bloom_intensity, 0.0..=1.0).text("bloom intensity"));
    egui::ComboBox::from_label("tone mapping").selected_text(fx.tonemapping.clone()).show_ui(ui, |ui| {
        for t in ["none", "reinhard", "reinhard_luminance", "tony"] {
            ui.selectable_value(&mut fx.tonemapping, t.to_string(), t);
        }
    });
    let mut one_hit = crate::combat::ONE_HIT_KILL.load(std::sync::atomic::Ordering::Relaxed);
    if ui.checkbox(&mut one_hit, "one hit kill (the sword kills what it hits)").changed() {
        crate::combat::ONE_HIT_KILL.store(one_hit, std::sync::atomic::Ordering::Relaxed);
    }
    ui.separator();
    ui.label(format!("this room: {} ({})", name, room_id));
    if let Some(env) = data.environments.get_mut(&name) {
        ui.add(egui::Slider::new(&mut env.ambient, 0.2..=1.0).text("ambient"));
        ui.horizontal(|ui| {
            ui.label("tint");
            ui.color_edit_button_rgb(&mut env.tint);
        });
        ui.add(egui::Slider::new(&mut env.light_strength, 0.0..=1.5).text("flame light"));
        ui.add(egui::Slider::new(&mut env.light_radius, 1.0..=9.0).text("flame reach"));
        ui.add(egui::Slider::new(&mut env.vignette, 0.0..=0.9).text("vignette"));
        ui.add(egui::Slider::new(&mut env.bloom, 0.0..=2.0).text("glow"));
        ui.add(egui::Slider::new(&mut env.shadows, 0.0..=1.5).text("blue shadows"));
        ui.add(egui::Slider::new(&mut env.warmth, 0.0..=0.4).text("warmth"));
        ui.add(egui::Slider::new(&mut env.contrast, 0.8..=1.4).text("contrast"));
        ui.add(egui::Slider::new(&mut env.saturation, 0.0..=1.6).text("saturation"));
        ui.horizontal(|ui| {
            ui.label("dark");
            ui.color_edit_button_rgb(&mut env.night);
        });
        ui.add(egui::Slider::new(&mut env.night_amount, 0.0..=0.8).text("dark amount"));
    }
    ui.add(egui::Slider::new(&mut data.flicker, 0.0..=0.5).text("flicker"));
    ui.add(egui::Slider::new(&mut data.speed, 1.0..=14.0).text("flicker Hz"));
    ui.horizontal(|ui| {
        ui.label("flame colour");
        ui.color_edit_button_rgb(&mut data.color);
    });
    ui.separator();
    ui.horizontal(|ui| {
        if ui.button("Save switches (ini)").clicked() {
            *status = if config.save_effects() { "landstalker.ini written".to_string() } else { "could not write landstalker.ini".to_string() };
        }
        if ui.button("Save looks (effects.yaml)").clicked() {
            *status = match data.save(Path::new(&format!("{}/data", std::env::var("BEVY_ASSET_ROOT").unwrap_or_default()))) {
                Ok(()) => "effects.yaml written".to_string(),
                Err(e) => format!("effects.yaml: {}", e),
            };
        }
    });
    if !status.is_empty() {
        ui.label(status.as_str());
    }
}

impl EffectsData {
    /// Writes the looks (the kinds of place, the colour, the flicker) back into data/effects.yaml, keeping the rest of the file as it is.
    pub fn save(&self, data: &Path) -> Result<(), String> {
        let path = data.join("effects.yaml");
        let text = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let header: String = text.lines().take_while(|l| l.starts_with('#')).map(|l| format!("{}\n", l)).collect();
        let mut tree: serde_yaml::Value = serde_yaml::from_str(&text).map_err(|e| e.to_string())?;
        let num = |v: f32| serde_yaml::Value::from((v as f64 * 1000.0).round() / 1000.0);
        if let Some(envs) = tree.get_mut("environments").and_then(|e| e.as_mapping_mut()) {
            for (name, e) in &self.environments {
                let mut m = serde_yaml::Mapping::new();
                for (k, v) in [("ambient", e.ambient), ("light_strength", e.light_strength), ("light_radius", e.light_radius), ("vignette", e.vignette), ("bloom", e.bloom), ("shadows", e.shadows), ("warmth", e.warmth), ("contrast", e.contrast), ("saturation", e.saturation), ("night_amount", e.night_amount)] {
                    m.insert(k.into(), num(v));
                }
                m.insert("tint".into(), serde_yaml::Value::Sequence(e.tint.iter().map(|v| num(*v)).collect()));
                m.insert("night".into(), serde_yaml::Value::Sequence(e.night.iter().map(|v| serde_yaml::Value::from((v * 255.0).round() as i64)).collect()));
                envs.insert(name.clone().into(), serde_yaml::Value::Mapping(m));
            }
        }
        if let Some(light) = tree.get_mut("flame").and_then(|f| f.get_mut("light")).and_then(|l| l.as_mapping_mut()) {
            light.insert("color".into(), serde_yaml::Value::Sequence(self.color.iter().map(|v| serde_yaml::Value::from((v * 255.0).round() as i64)).collect()));
            light.insert("flicker".into(), num(self.flicker));
            light.insert("speed".into(), num(self.speed));
        }
        let body = serde_yaml::to_string(&tree).map_err(|e| e.to_string())?;
        std::fs::write(&path, header + &body).map_err(|e| e.to_string())
    }
}
