//! landstalker_next: step 1 — a room of the converted game files (data/rooms/room_NNN) drawn in isometric view with
//! Nigel walking on the heightmap. Usage: landstalker [room] [x y]   (default: room 18, at its first door)
//!
//! Arrow keys: Up = NW (-x), Down = SE (+x), Left = SW (+y), Right = NE (-y) like the C++ engine. Esc: the pause menu.

mod assets;
mod bot;
mod audio;
mod combat;
mod cutscene;
mod dialogue;
mod effects;
mod config;
mod game;
mod i18n;
mod menu;
mod physics;
mod playtest;
mod save;
mod scripting;
mod shadows;
mod shops;
mod swaps;
mod tas;
mod title;
mod track;
mod versions;
mod yamldata;

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::asset::AssetPlugin;
use bevy::window::WindowResolution;
use bevy_egui::{egui, input::egui_wants_any_keyboard_input, EguiInput, EguiPlugin, EguiPreUpdateSet, EguiPrimaryContextPass};
use ls_data::sprites::{Dir, Directions, Pic, Poses, SpriteDef};
use ls_data::{room_dir, Entity as RoomEntity, Room};
use physics::{Body, Solid, Tuning, World};
use assets::DataFiles;
use audio::{Pcm, SfxQueue, Sound};
use bevy::audio::AddAudioSource;
use combat::Combat;
use config::{read_controls, Config, Controls};
use game::{ChestItem, ChestOpen, ChestSeq, GameState, Message, CHEST_OPEN_TIME, CHEST_REVEAL_TIME};
use scripting::{KindInfo, Scripting};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

/// Pixels of the original picture are drawn this many times bigger.
const SCALE: f32 = 4.0;
/// where the original's screen puts Nigel's torso, in pixels of the picture from the top of the screen (measured on the ROM running in EmuHawk)
const NIGEL_SCREEN_Y: f32 = 85.0;

/// Where the original's 320x224 picture is drawn in the game area: its top-left corner, and the screen pixels per pixel of the picture (the
/// map's scale): Nigel is in the middle of the area and 85 pixels of the picture from the top. The boxes, bars and captions of the story
/// use it, so they sit where the original has them whatever the size of the window.
pub fn picture_origin(area: bevy_egui::egui::Rect) -> (bevy_egui::egui::Pos2, f32) {
    (bevy_egui::egui::pos2(area.center().x - 160.0 * SCALE, area.top()), SCALE)
}
const HEIGHT_UNIT_PX: f32 = 16.0;

// ---------------------------------------------------------------------------------------------------------------
// data files of the sprites
// ---------------------------------------------------------------------------------------------------------------

/// What `sprites/entities/entity_NNN/entity.yaml` holds (docs/sprites_format.md): the stats and the poses.
#[derive(Deserialize, Default, Clone)]
struct EntityDef {
    hitbox_base: Option<i32>,
    hitbox_height: Option<i32>,
    enemy: Option<bool>,
    health: Option<i32>,
    defence: Option<i32>,
    attack: Option<i32>,
    gold_drop: Option<i32>,
    #[serde(default)]
    poses: Poses,
    #[serde(default)]
    directions: Directions,
}

/// The entity types of data/sprites/entities.
struct EntitiesFile {
    entities: HashMap<u32, EntityDef>,
}

/// One drawable picture: asset path (under data/), size, the pixel that sits on the entity position, and whether it is drawn mirrored.
#[derive(Clone)]
struct Picture {
    path: String,
    w: i32,
    h: i32,
    ax: i32,
    ay: i32,
    mirror: bool,
}

fn picture_of(base: &str, p: &Pic, mirror: bool) -> Picture {
    Picture { path: format!("{}/{}", base, p.file), w: p.w, h: p.h, ax: p.ax, ay: p.ay, mirror }
}

// ---------------------------------------------------------------------------------------------------------------
// resources and components
// ---------------------------------------------------------------------------------------------------------------

#[derive(Resource)]
struct RoomRes {
    room: Room,
    /// height of the block grid (the engine's `height`), left / top of the heightmap in 8 px units
    block_h: i32,
    hm_left: i32,
    hm_top: i32,
}

impl RoomRes {
    /// Screen pixel (y down) of a point of the world at a height, like RoomView::ToScreen of the C++ engine.
    fn to_screen(&self, wx: f32, wy: f32, z: f32) -> Vec2 {
        let xx = wx + 0.5 - self.hm_left as f32;
        let yy = wy - 0.5 - self.hm_top as f32;
        Vec2::new(((xx - yy + (self.block_h - 1) as f32) * 2.0 + self.hm_left as f32) * 8.0, (xx + yy + self.hm_top as f32) * 8.0 - z * HEIGHT_UNIT_PX)
    }

    fn floor_height(&self, cx: i32, cy: i32) -> f32 {
        self.room.cell_at_world(cx, cy).map(|c| c.height as f32).unwrap_or(0.0)
    }
}

/// Solid entities of the room (still ones for now).
#[derive(Resource)]
struct Solids(Vec<Solid>);

/// The entities of the room as the room file has them (their pictures); their live state is in the scripts' `ents`.
#[derive(Resource)]
struct RoomEnts(Vec<RoomEntity>);

/// Pictures read from disk once, as pixels.
#[derive(Resource, Default)]
struct CpuImages(HashMap<String, Option<image::RgbaImage>>);

/// A sprite of a room entity: its texture is rebuilt when its picture, place or hiding changes.
#[derive(Component)]
struct RoomEnt {
    index: usize,
    key: Option<(String, bool, i32, i32, bool)>,
    /// walking: where it was, seconds since it last moved, seconds it has been walking
    last: (f32, f32),
    since_moved: f32,
    walk_time: f32,
}

/// Water, lava, waterfalls: blocks whose picture changes every `delay` frames of the console (60 Hz); the pictures of a
/// room's animations are in anim_N.png (one strip of blocks, one row of pictures per frame).
#[derive(Resource)]
struct TileAnims {
    blocks: Handle<Image>,
    groups: Vec<AnimGroup>,
    clock: f32,
}

struct AnimGroup {
    atlas: image::RgbaImage,
    blocks: Vec<usize>,
    frames: u32,
    delay: f32,
    shown: i32,
}

/// The map as loaded and the tile swaps of the room (see swaps.rs).
#[derive(Resource)]
struct SwapState {
    base: swaps::Base,
    swaps: Vec<swaps::Swap>,
    flags: Vec<swaps::SwapFlag>,
}

/// A block of the map drawn on the screen (layer 0 background, 1 foreground).
#[derive(Component)]
struct BlockSprite {
    layer: usize,
    bx: i32,
    by: i32,
}

/// The game stands still (choosing a screenshot area).
#[derive(Resource, Default)]
struct GameFrozen(bool);

/// Why the player cannot move ("scene", "dialogue"...), shown by the panel; None = he is in control.
#[derive(Resource, Default)]
struct ControlLock(Option<String>);

/// Ids of the rooms of data/index.yaml, in order.
#[derive(Resource)]
struct RoomIndex(Vec<i32>);

impl RoomIndex {
    fn contains(&self, id: i32) -> bool {
        self.0.contains(&id)
    }
    /// The next (dir = 1) or previous (-1) room of the index.
    fn neighbour(&self, id: i32, dir: i32) -> Option<i32> {
        let i = self.0.iter().position(|r| *r == id)? as i32 + dir;
        self.0.get(usize::try_from(i).ok()?).copied()
    }
}

/// A room to go to at the next frame.
#[derive(Resource, Default)]
struct RoomRequest(Option<i32>);

/// Where Nigel appears in the room asked for (the target cell of a warp); the room's first door when None.
#[derive(Resource, Default)]
struct Arrival(Option<Vec2>);

/// The cell of the door Nigel came in by when a raft was waiting for him: the raft may carry him over that door without it taking him back through it.
#[derive(Resource, Default)]
struct RaftArrival(Option<(i32, i32)>);

/// How Nigel arrives: 0 as given, 1 falling (the nearest floor if there is none there), 2 by a ladder, 3 by an elevator.
#[derive(Resource, Default)]
struct ArrivalMode(u8);

#[derive(Resource)]
struct DataDir(PathBuf);

/// data/physics.yaml (gravity, jump height, speeds); R reloads it while playing.
#[derive(Resource)]
struct TuningRes(Tuning);

fn load_tuning() -> Tuning {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/physics.yaml");
    match std::fs::read_to_string(&path) {
        Ok(text) => serde_yaml::from_str(&text).unwrap_or_else(|e| {
            println!("physics.yaml: {}", e);
            Tuning::default()
        }),
        Err(_) => Tuning::default(),
    }
}

#[derive(Resource)]
struct Sprites {
    entities: HashMap<u32, EntityDef>,
    player: SpriteDef,
    /// the chest's pictures (closed, opening, open)
    chest: SpriteDef,
    /// Nigel's pictures as pixels (cut where blocks hide him) and the texture the sprite shows
    player_cpu: HashMap<String, image::RgbaImage>,
    player_texture: Handle<Image>,
}

#[derive(Component)]
struct WorldRoot;

#[derive(Component)]
struct Player {
    x: f32,
    y: f32,
    z: f32,
    /// 0 NW, 1 NE, 2 SE, 3 SW (the engine's player facing)
    vz: f32,
    grounded: bool,
    air_peak: f32,
    facing: u8,
    anim_time: f32,
    moving: bool,
    /// index of the arrow key that decides the direction (the one pressed last)
    last_key: Option<usize>,
    /// the room entity held over his head, and whether he is lifting it (1) or setting it down (2)
    carrying: Option<usize>,
    carried_solid: bool,
    lift_phase: u8,
    lift_t: f32,
    drop: Vec3,
    /// on a ladder (floor type 0x0B / 0x0C)
    climbing: bool,
}

/// Things thrown by Nigel, in flight.
#[derive(Resource, Default)]
struct Thrown(Vec<ThrownItem>);

struct ThrownItem {
    index: usize,
    vx: f32,
    vy: f32,
    vz: f32,
    t: f32,
}

const LIFT_TIME: f32 = 0.4;
const CARRY_HEIGHT: f32 = 2.0;
// crates, vases, ... and the books of the churches (their shelves: shops::church_books)
const LIFTABLE: [i32; 10] = [22, 17, 14, 15, 227, 225, 226, 209, 228, 238];

#[derive(Component)]
struct PlayerSprite;

/// What a block can hide of a sprite standing behind it (like the C++ engine's priority blocks): the cell it shows must be in
/// front (a larger x + y), rise above the sprite's feet; per pixel of the room, see `CoverMap`.
#[derive(Clone, Copy)]
struct HideAttr {
    depth: f32,
    z: f32,
    wall: bool,
    /// the pixel is a priority pixel of blocks_priority.png (it goes over sprites by itself) and not a closed cell's
    priority: bool,
}

/// Which occlusion rule. `Original` (default) is the rule of the Mega Drive game (code/gamelogic/gamelogic3.asm `sub_44C4` in
/// landstalker_disasm): a sprite is "low priority" (hidden by the priority tiles of the map) when a cell in front of it,
/// along the diagonal line of sight, is higher than the line; otherwise it is drawn over everything. The tiles' priority
/// pixels are those of blocks_priority.png. `Cpp` is the C++ engine's rule (compare with tools/compare_cpp.py), `Objects`
/// and `Mine` are my earlier experiments.
#[derive(PartialEq, Clone, Copy)]
enum Rule {
    Original,
    Cpp,
    Objects,
    Mine,
}

fn rule() -> Rule {
    match std::env::var("LS_RULE").as_deref() {
        Ok("cpp") => Rule::Cpp,
        Ok("objects") => Rule::Objects,
        Ok("mine") => Rule::Mine,
        _ => Rule::Original,
    }
}

impl HideAttr {
    fn hides(&self, sprite_depth: f32, sprite_z: f32) -> bool {
        let top = self.z + if self.wall { 1.0 } else { 0.0 };
        if self.priority && rule() == Rule::Mine {
            self.depth > sprite_depth - 1.0 && top > sprite_z - 0.01
        } else {
            self.depth > sprite_depth && top > sprite_z + 0.01
        }
    }
}

/// For each pixel of the room (original pixels): the block pixel on top that may hide a sprite behind it (0 = none).
/// Nigel's picture is cut where such a pixel is in front of him, so no block is ever drawn twice or changes its look.
#[derive(Resource, Clone)]
struct CoverMap {
    x0: i32,
    y0: i32,
    w: i32,
    h: i32,
    /// priority pixels (top opaque pixel of the blocks)
    cells: Vec<u16>,
    /// silhouettes of the closed cells (walls, pillars, statues, stones) on the opaque pixels
    sil: Vec<u16>,
    /// union of the priority pixels of both layers (the console's priority tiles)
    pri: Vec<bool>,
    attrs: Vec<HideAttr>,
}

impl CoverMap {
    /// Is this pixel of the room covered by a tile with the priority bit (it goes over low priority sprites)?
    fn priority_at(&self, x: i32, y: i32) -> bool {
        let (cx, cy) = (x - self.x0, y - self.y0);
        cx >= 0 && cy >= 0 && cx < self.w && cy < self.h && self.pri[(cy * self.w + cx) as usize]
    }

    /// Does something of the room hide a sprite whose feet are in the cell of x + y = `depth`, at height `z`, at this pixel?
    fn hides(&self, x: i32, y: i32, depth: f32, z: f32) -> bool {
        let (cx, cy) = (x - self.x0, y - self.y0);
        if cx < 0 || cy < 0 || cx >= self.w || cy >= self.h {
            return false;
        }
        let i = (cy * self.w + cx) as usize;
        [self.cells[i], self.sil[i]].iter().any(|&id| id != 0 && self.attrs.get(id as usize).map_or(false, |a| a.hides(depth, z)))
    }
}

// ---------------------------------------------------------------------------------------------------------------

/// Where the player appears when nothing else is said: on the first door of the room; a door that is a staircase is a
/// hole in the floor level, so he is put on the nearest plain floor next to it.
fn start_of(room: &Room) -> Vec2 {
    // a room where a raft carries him (163, 165...): he starts on it, the raft goes as soon as the room opens
    if let Some(r) = room.file.entities.iter().find(|e| e.kind == 53 && (e.behaviour == 35 || e.behaviour == 36)) {
        return Vec2::new(r.position[0] as f32, r.position[1] as f32);
    }
    let Some(w) = room.file.warps.first() else { return Vec2::new(30.0, 30.0) };
    let (wx, wy) = (w.area[0], w.area[1]);
    let on_stairs = room.cell_at_world(wx, wy).map_or(false, |c| c.floor_type == 5);
    if on_stairs {
        let mut best: Option<(i32, i32, i32)> = None;
        for dy in -3..=3 {
            for dx in -3..=3 {
                let Some(c) = room.cell_at_world(wx + dx, wy + dy) else { continue };
                if c.blocked() || c.floor_type != 0 {
                    continue;
                }
                let d = dx * dx + dy * dy;
                if best.map_or(true, |b| d < b.2) {
                    best = Some((wx + dx, wy + dy, d));
                }
            }
        }
        if let Some((x, y, _)) = best {
            return Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
        }
    }
    Vec2::new(wx as f32 + 0.5, wy as f32 + 0.5)
}

/// The area of a door or a staircase is never stood in on arrival (the original does not let Nigel reach the last step: he
/// would leave the room at once). He appears in front of it instead, on the side of the room (the side with more floor
/// beyond it), centred on the area.
fn outside_warps(room: &Room, at: Vec2) -> Vec2 {
    let (ax, ay) = (at.x.floor() as i32, at.y.floor() as i32);
    let Some(w) = room.file.warps.iter().find(|w| ax >= w.area[0] && ax < w.area[0] + w.area[2] && ay >= w.area[1] && ay < w.area[1] + w.area[3]) else { return at };
    let [x0, y0, w_, h_] = w.area;
    // a door is arrived in, centred on its area; only a staircase is never stood on (its last step leaves the room at once)
    if w.kind == "normal" {
        return Vec2::new(x0 as f32 + w_ as f32 / 2.0, y0 as f32 + h_ as f32 / 2.0);
    }
    let walkable = |x: i32, y: i32| room.cell_at_world(x, y).map_or(false, |c| !c.blocked());
    let in_area = |x: i32, y: i32| x >= x0 && x < x0 + w_ && y >= y0 && y < y0 + h_;
    // the door is in a wall along the longer side of its area: one step out of it along the other axis
    let along_x = h_ >= w_;
    let sides: [i32; 2] = [-1, 1];
    let mut best: Option<(i32, Vec2)> = None;
    for side in sides {
        let (first_x, first_y) = if along_x { (if side < 0 { x0 - 1 } else { x0 + w_ }, y0) } else { (x0, if side < 0 { y0 - 1 } else { y0 + h_ }) };
        // the cell in front of the door, and how much floor lies beyond it
        let mut count = 0;
        for depth in 0..4 {
            for span in 0..(if along_x { h_ } else { w_ }) {
                let (x, y) = if along_x { (first_x + side * depth, y0 + span) } else { (x0 + span, first_y + side * depth) };
                if !in_area(x, y) && walkable(x, y) {
                    count += 1;
                }
            }
        }
        if !walkable(first_x, first_y) {
            continue;
        }
        let pos = if along_x { Vec2::new(first_x as f32 + 0.5, y0 as f32 + h_ as f32 / 2.0) } else { Vec2::new(x0 as f32 + w_ as f32 / 2.0, first_y as f32 + 0.5) };
        if best.map_or(true, |b| count > b.0) {
            best = Some((count, pos));
        }
    }
    best.map_or(at, |b| b.1)
}

/// Where Nigel lands after a fall (the same place, or the nearest floor) or arrives by a ladder / an elevator (the nearest
/// ladder / elevator cell of the room above).
fn arrival_cell(room: &Room, at: Vec2, mode: u8) -> Vec2 {
    let (ax, ay) = (at.x.floor() as i32, at.y.floor() as i32);
    let mut best: Option<(i32, i32, i32)> = None;
    for dy in -40..=40 {
        for dx in -40..=40 {
            let Some(c) = room.cell_at_world(ax + dx, ay + dy) else { continue };
            if c.blocked() {
                continue;
            }
            let ok = match mode {
                2 => c.floor_type == 0x0B || c.floor_type == 0x0C,
                3 => c.floor_type == 0x0F,
                _ => true,
            };
            let d = dx * dx + dy * dy;
            if ok && best.map_or(true, |b| d < b.2) {
                best = Some((ax + dx, ay + dy, d));
            }
        }
    }
    best.map_or(at, |(x, y, _)| Vec2::new(x as f32 + 0.5, y as f32 + 0.5))
}

/// A pit (floor type 7) makes whoever stands on it fall to the room below; a ladder (0x0B / 0x0C) or an elevator (0x0F) takes
/// him out of the room once he is higher than its highest point.
fn floor_effects(
    room: Res<RoomRes>,
    index: Res<RoomIndex>,
    player: Query<&Player>,
    mut request: ResMut<RoomRequest>,
    mut arrival: ResMut<Arrival>,
    mut mode: ResMut<ArrivalMode>,
    mut sfx: ResMut<SfxQueue>,
) {
    let Ok(p) = player.single() else { return };
    if request.0.is_some() {
        return;
    }
    let (cx, cy) = (p.x.floor() as i32, p.y.floor() as i32);
    let Some(cell) = room.room.cell_at_world(cx, cy) else { return };
    let at = Vec2::new(cx as f32 + 0.5, cy as f32 + 0.5);
    let file = &room.room.file;
    if let Some(dest) = file.climb_destination.filter(|d| index.contains(*d)) {
        if p.z + 0.0625 > file.heights.z_end as f32 && (cell.floor_type == 0x0B || cell.floor_type == 0x0C) {
            request.0 = Some(dest);
            arrival.0 = Some(at);
            mode.0 = 2;
            return;
        }
    }
    if let Some(dest) = file.fall_destination.filter(|d| index.contains(*d)) {
        if p.grounded && cell.floor_type == 7 && (p.z - cell.height as f32).abs() <= 0.5 {
            sfx.play(0x54);
            request.0 = Some(dest);
            arrival.0 = Some(at);
            mode.0 = 1;
        }
    }
}

/// Test aid: LS_DUMP_CELLS=x0,y0,x1,y1 prints the heights of the cells (## blocked, s stairs) of the room that opens.
fn dump_cells(room: &Room) {
    let Some(v) = std::env::var("LS_DUMP_CELLS").ok().map(|v| v.split(',').filter_map(|n| n.trim().parse::<i32>().ok()).collect::<Vec<_>>()).filter(|v| v.len() == 4) else { return };
    println!("[cells] room {} x {}..{} (columns), y {}..{} (rows)", room.file.id, v[0], v[2], v[1], v[3]);
    for y in v[1]..=v[3] {
        let row: Vec<String> = (v[0]..=v[2]).map(|x| room.cell_at_world(x, y).map_or("  .".to_string(), |c| if c.blocked() { " ##".to_string() } else { format!("{:3}{}", c.height, if c.floor_type == 5 { "s" } else if c.floor_type == 0x0E { "c" } else if matches!(c.floor_type, 0x11..=0x18 | 0x1E..=0x25) { "!" } else { "" }) })).collect();
        println!("[cells] {:3} {}", y, row.join(" "));
    }
}

/// The doors of a room, read from its room.yaml (what a replacing version uses).
fn original_warps(data: &std::path::Path, room: i32) -> Vec<ls_data::Warp> {
    #[derive(serde::Deserialize)]
    struct WarpsOnly {
        #[serde(default)]
        warps: Vec<ls_data::Warp>,
    }
    std::fs::read_to_string(room_dir(data, room).join("room.yaml")).ok().and_then(|t| serde_yaml::from_str::<WarpsOnly>(&t).ok()).map(|w| w.warps).unwrap_or_default()
}

fn room_res(room: Room) -> RoomRes {
    let block_h = room.file.map.blocks.size[1];
    let (hm_left, hm_top) = (room.file.map.legacy.heightmap_left, room.file.map.legacy.heightmap_top);
    RoomRes { room, block_h, hm_left, hm_top }
}

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("..");
    std::env::set_var("BEVY_ASSET_ROOT", &root);
    let mut config = Config::load(&root);
    if std::env::var("LS_MUTE").is_ok() {
        config.audio.enabled = false; // (tests run silent)
    }
    // options of the ini file that the code reads from the environment (an environment variable still wins)
    if std::env::var("LS_RULE").is_err() && config.occlusion_rule != "original" {
        std::env::set_var("LS_RULE", &config.occlusion_rule);
    }
    if config.log_script {
        std::env::set_var("LS_LOG_SCRIPT", "1");
    }
    combat::ONE_HIT_KILL.store(config.one_hit_kill, std::sync::atomic::Ordering::Relaxed);
    let args: Vec<String> = std::env::args().skip(1).collect();
    // LS_NEWGAME=1: the adventure from its start, the intro scenes (room 139) run by the story
    let new_game = std::env::var("LS_NEWGAME").is_ok();
    // LS_NOINTRO=1 with it: the adventure starts in room 148 (the intro cannot be skipped, so tests that do not need it start here)
    let intro = new_game && std::env::var("LS_NOINTRO").is_err();
    // the TAS the game starts from (landstalker.ini `tas_start_room`): its room, its place, its flags
    let tas_from = tas::start_file(&root, &config);
    let tas_start_given = std::env::var("LS_TAS_START").ok();
    let room_id: i32 = args.first().and_then(|a| a.parse().ok()).or(tas_from.as_ref().map(|t| t.room)).unwrap_or(if intro { 139 } else if new_game { 148 } else if config.start_room > 0 { config.start_room } else { 18 });
    let data = root.join("data");
    let room = Room::load(&room_dir(&data, room_id)).unwrap_or_else(|e| panic!("room {}: {:#}", room_id, e));
    dump_cells(&room);
    let start = match (args.get(1), args.get(2)) {
        (Some(x), Some(y)) => Vec2::new(x.parse().unwrap_or(30.0), y.parse().unwrap_or(30.0)),
        _ if tas_from.as_ref().is_some_and(|t| t.room == room_id) => tas_from.as_ref().map_or(Vec2::ZERO, |t| Vec2::new(t.start.x, t.start.y)),
        _ if intro => Vec2::new(62.5, 24.5),
        _ if new_game => Vec2::new(32.5, 38.5),
        _ => start_of(&room),
    };
    let index = ls_data::load_index(&data).expect("index.yaml");
    i18n::init(&data, &std::env::var("LS_LANG").unwrap_or_else(|_| config.language.clone()));
    let files = DataFiles::open(data.clone(), &config.sprites_mode);
    let mut table = HashMap::new();
    for (id, text) in files.entity_definitions() {
        match serde_yaml::from_str::<EntityDef>(&text) {
            Ok(d) => {
                table.insert(id, d);
            }
            Err(e) => println!("sprites: entity {}: {}", id, e),
        }
    }
    assert!(!table.is_empty(), "no entity in data/sprites/entities (entity_NNN/entity.yaml or entity_NNN.lsa)");
    let entities = EntitiesFile { entities: table };
    let player: SpriteDef = serde_yaml::from_str(&files.read_string("sprites/sprite_000/sprite.yaml").expect("sprites/sprite_000/sprite.yaml")).expect("sprite_000");
    let chest: SpriteDef = serde_yaml::from_str(&files.read_string("sprites/sprite_036/sprite.yaml").expect("sprites/sprite_036/sprite.yaml")).expect("sprite_036");
    let mut combat = Combat::default();
    for (k, d) in &entities.entities {
        if d.enemy.unwrap_or(false) {
            combat.stats.insert(*k as i32, combat::EnemyStats { health: d.health.unwrap_or(1), defence: d.defence.unwrap_or(0), attack: d.attack.unwrap_or(1), gold: d.gold_drop.unwrap_or(0) });
        }
    }
    println!("room {} \"{}\": {} entities, start at {:?}", room.file.id, room.file.name, room.file.entities.len(), start);
    let mut script = Scripting::new(&data).unwrap_or_else(|e| panic!("scripts: {:#}", e));
    script.set_kinds(
        entities
            .entities
            .iter()
            .map(|(k, d)| (*k as i32, KindInfo { hbase: d.hitbox_base.unwrap_or(8) as f32 / 16.0, hheight: d.hitbox_height.unwrap_or(16) as f32 / 16.0, known: true, killable: d.enemy.unwrap_or(false) && d.health != Some(255), enemy: d.enemy.unwrap_or(false), attack: d.attack.unwrap_or(0) }))
            .collect(),
    );

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window { title: format!("Landstalker next - room {}", room_id), resolution: {
                        // (LS_WINDOW=1920x1080: test aid for other window sizes)
                        let (w, h) = std::env::var("LS_WINDOW").ok().and_then(|v| v.split_once('x').and_then(|(a, b)| Some((a.parse().ok()?, b.parse().ok()?)))).unwrap_or(config.window);
                        WindowResolution::new(w, h).with_scale_factor_override(1.0)
                    }, position: if std::env::var("LS_SHOT").is_ok() { WindowPosition::At(IVec2::new(-4000, -4000)) } else { WindowPosition::Centered(MonitorSelection::Primary) }, focused: std::env::var("LS_SHOT").is_err(), ..default() }),
                    ..default()
                })
                .set(AssetPlugin { file_path: "data".into(), ..default() })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(EguiPlugin::default())
        .add_plugins(effects::EffectsPlugin)
        .insert_resource(effects::EffectsData::load(&root.join("data")))
        .add_systems(Update, shadows::object_shadows)
        .add_systems(PreUpdate, tab_is_for_the_game.after(EguiPreUpdateSet::ProcessInput).before(EguiPreUpdateSet::BeginPass))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(room_res(room))
        .insert_resource(Sprites { entities: entities.entities, player, chest, player_cpu: HashMap::new(), player_texture: Handle::default() })
        .insert_resource(StartPos(start))
        .insert_resource(tas::TasPlayer::default())
        .insert_resource(tas::TasStartFrom(tas_from))
        .add_systems(Startup, tas::apply_start.after(setup))
        .add_systems(Update, tas::tas_record.after(read_controls))
        .insert_resource(Solids(Vec::new()))
        .insert_resource(CpuImages::default())
        .insert_resource(config.clone())
        .insert_resource({
            let mut g = GameState::with_names(&yamldata::read(&data.join("items.yaml")));
            if let Some(n) = std::env::var("LS_HP").ok().and_then(|v| v.parse().ok()) {
                g.set_max_hp(n); // test aid: more health
            }
            if let Some(n) = std::env::var("LS_HURT").ok().and_then(|v| v.parse::<f32>().ok()) {
                g.damage(n); // test aid: that many points of health lost
            }
            // test aids: LS_ITEMS="id:count,..." gives items, LS_STATUS="poison,confusion,paralysis,curse" the ailments
            for part in std::env::var("LS_ITEMS").unwrap_or_default().split(',') {
                if let Some((id, n)) = part.split_once(':').and_then(|(a, b)| Some((a.trim().parse::<i32>().ok()?, b.trim().parse::<i32>().ok()?))) {
                    g.items.insert(id, n);
                }
            }
            if let Some(n) = std::env::var("LS_GOLD").ok().and_then(|v| v.parse::<i32>().ok()) {
                g.gold = n; // test aid: some gold
            }
            if let Some(n) = std::env::var("LS_SWORD").ok().and_then(|v| v.parse::<i32>().ok()) {
                g.add_item(n); // test aid: wearing this sword
                g.sword = n;
            }
            for name in std::env::var("LS_STATUS").unwrap_or_default().split(',') {
                match name.trim() {
                    "poison" => g.add_status(game::STATUS_POISON),
                    "confusion" => g.add_status(game::STATUS_CONFUSION),
                    "paralysis" => g.add_status(game::STATUS_PARALYSIS),
                    "curse" => g.add_status(game::STATUS_CURSE),
                    _ => {}
                }
            }
            g
        })
        .insert_resource(Message::default())
        .insert_resource(ChestOpen::default())
        .insert_resource(save::SaveRequest::default())
        .insert_resource(files)
        .insert_resource(config.audio.clone())
        .insert_resource(Controls::default())
        .insert_resource(Sound::new(&data))
        .insert_resource(SfxQueue::default())
        .insert_resource(Thrown::default())
        .insert_non_send(script)
        .insert_resource(TuningRes(load_tuning()))
        .insert_resource(CameraFollow(true))
        .insert_resource(combat)
        .insert_resource(menu::Menu::default())
        .insert_resource(shops::Shops::load(&data))
        .insert_resource(shops::Dialogue::default())
        .insert_resource({
            let mut c = cutscene::Cutscene::default();
            // test aid: LS_CREDITS=seconds shows the credits of the ending, that far into their scroll
            if let Some(t) = std::env::var("LS_CREDITS").ok().and_then(|v| v.parse::<f32>().ok()) {
                c.credits = Some(t);
            }
            if intro {
                match std::fs::read_to_string(data.join("scripts/intro/intro.track")) {
                    Ok(text) => c.begin_intro_track(track::Track::parse(&text), 139),
                    Err(_) => c.begin_intro(),
                }
            }
            c
        })
        .insert_resource(cutscene::Cutscenes::load(&data))
        // no argument: the title screen (LS_NOTITLE=1 goes straight to the game, LS_TITLE=1 shows the title with an argument)
        .insert_resource(title::Title::new(std::env::var("LS_TITLE").is_ok() || (args.is_empty() && !new_game && std::env::var("LS_NOTITLE").is_err() && config.start_room <= 0 && config.tas_start_room <= 0 && tas_start_given.is_none())))
        .insert_resource(dialogue::Dialogues::load(&data.join("dialogues")))
        .insert_resource(GameFrozen::default())
        .insert_resource(ControlLock::default())
        .init_resource::<bot::Recorder>()
        .insert_resource(RoomRequest::default())
        .insert_resource(Arrival::default())
        .init_resource::<RaftArrival>()
        .insert_resource(versions::RoomVersions::load(&data))
        .insert_resource(ArrivalMode::default())
        .insert_resource(RoomIndex(index.rooms.iter().map(|r| r.id).collect()))
        .insert_resource(DataDir(data.clone()))
        .insert_resource(playtest::Playtest::new())
        .add_audio_source::<Pcm>()
        .add_systems(Startup, setup)
        .add_systems(Update, apply_room_request)
        .add_systems(
            Update,
            (walk, animate_player.after(walk), toggle_camera, pause_key, room_keys, warps.after(walk), follow_camera.after(walk), log_entities).run_if(not(egui_wants_any_keyboard_input)).after(apply_room_request),
        )
        .add_systems(Update, (interact.run_if(not(egui_wants_any_keyboard_input)), update_carry.after(interact), update_chest.after(interact), script_tick.after(update_carry), sync_entities.after(script_tick)).after(apply_room_request).after(walk))
        .add_systems(Update, title::title_input.after(read_controls).before(shops::dialogue_input).before(shops::talk).before(menu::menu_input).before(combat::attack_input).before(walk).before(interact).before(save::save_load))
        .add_systems(Update, shops::dialogue_input.after(read_controls).before(shops::talk).before(menu::menu_input).before(combat::attack_input).before(walk).before(interact).run_if(not(egui_wants_any_keyboard_input)))
        .add_systems(Update, shops::church_books.after(shops::dialogue_input))
        .add_systems(Update, shops::talk.after(read_controls).before(menu::menu_input).before(combat::attack_input).before(walk).before(interact).run_if(not(egui_wants_any_keyboard_input)))
        .add_systems(Update, menu::menu_input.after(read_controls).before(combat::attack_input).before(walk).before(interact).run_if(not(egui_wants_any_keyboard_input)))
        .add_systems(Update, save::save_load.after(read_controls).before(combat::attack_input).run_if(not(egui_wants_any_keyboard_input)))
        .add_systems(Update, (cutscene::track_update.before(walk).before(script_tick), cutscene::cutscene_update.after(shops::dialogue_input).before(script_tick), cutscene::cutscene_arrived.after(apply_room_request), cutscene::friday_sprite))
        .add_systems(Update, combat::attack_input.after(read_controls).before(walk).before(interact).run_if(not(egui_wants_any_keyboard_input)))
        .add_systems(Update, combat::combat_update.after(update_carry).before(script_tick).after(apply_room_request))
        .add_systems(Update, (combat::pickups.after(script_tick), combat::spawn_sparks, combat::animate_sparks))
        .add_systems(Update, playtest::capture_hotkey.run_if(|c: Res<Config>| c.playtest_panel))
        .add_systems(Update, debug_shot)
        .add_systems(PostUpdate, camera_parity.before(bevy::camera::visibility::VisibilitySystems::CalculateBounds).before(bevy::transform::TransformSystems::Propagate))
        .add_systems(Update, (autoplay, bot::bot, tas::tas_play).before(read_controls)).add_systems(Update, minimize_for_tests).add_systems(Update, bot::record.after(read_controls))
        .add_systems(Update, read_controls.before(walk).before(interact))
        .add_systems(Update, animate_tiles)
        .add_systems(Update, floor_effects.after(walk))
        .add_systems(Update, update_swaps.after(script_tick))
        .add_systems(Update, (audio::play_sfx, audio::play_music, drain_script_sfx.before(audio::play_sfx)))
        .add_systems(EguiPrimaryContextPass, playtest::panel.run_if(|c: Res<Config>| c.playtest_panel))
        .add_systems(EguiPrimaryContextPass, (game::message_overlay, game::hud_overlay, menu::menu_overlay, shops::dialogue_overlay, cutscene::viewport_mask, cutscene::fx_overlay, cutscene::caption_overlay, cutscene::scene_box_overlay, cutscene::credits_overlay, title::title_overlay))
        .run();
}

#[derive(Resource)]
struct StartPos(Vec2);

/// Walking into the area of a door or a staircase goes to the room behind it. Nothing happens again until Nigel has left the
/// areas of the room he arrives in (he may arrive on one).
fn warps(mut sfx: ResMut<SfxQueue>, mut raft: ResMut<RaftArrival>, mut was_in: Local<bool>, room: Res<RoomRes>, index: Res<RoomIndex>, player: Query<&Player>, mut request: ResMut<RoomRequest>, mut arrival: ResMut<Arrival>, mut state: Local<(i32, bool)>) {
    let Ok(p) = player.single() else { return };
    let (cx, cy) = (p.x.floor() as i32, p.y.floor() as i32);
    let found = room.room.file.warps.iter().find(|w| cx >= w.area[0] && cx < w.area[0] + w.area[2] && cy >= w.area[1] && cy < w.area[1] + w.area[3]);
    // (the room is stored as id + 1: a new Local is (0, false), which would be room 0 itself)
    if state.0 != room.room.file.id + 1 {
        *state = (room.room.file.id + 1, true);
    }
    // the door under the raft he arrived on: ignored while he is carried over it
    if let Some(cell) = raft.0 {
        let area = room.room.file.warps.iter().find(|w| cell.0 >= w.area[0] && cell.0 < w.area[0] + w.area[2] && cell.1 >= w.area[1] && cell.1 < w.area[1] + w.area[3]).map(|w| w.area);
        if area.is_some() && found.map(|w| w.area) == area {
            *was_in = true;
            state.1 = true;
            return;
        } else if *was_in {
            *was_in = false;
            raft.0 = None;
        }
    }
    match found {
        None => state.1 = false,
        Some(w) if !state.1 && request.0.is_none() => {
            if index.contains(w.target_room) {
                println!("warp {:?} -> room {} cell {:?}", w.kind, w.target_room, w.target_cell);
                sfx.play(0x55); // SND_Door
                request.0 = Some(w.target_room);
                arrival.0 = Some(Vec2::new(w.target_cell[0] as f32 + 0.5, w.target_cell[1] as f32 + 0.5));
            } else {
                println!("warp to room {} which is not in the data", w.target_room);
                state.1 = true;
            }
        }
        _ => {}
    }
}

/// O / P: previous / next room of the index.
fn room_keys(ctl: Res<Controls>, room: Res<RoomRes>, index: Res<RoomIndex>, mut request: ResMut<RoomRequest>) {
    let dir = if ctl.previous_room.just {
        -1
    } else if ctl.next_room.just {
        1
    } else {
        return;
    };
    if let Some(id) = index.neighbour(room.room.file.id, dir) {
        request.0 = Some(id);
        versions::ask_directly();
    }
}

/// Test aid: LS_AUTOPLAY="1.0:+ArrowUp;1.5:-ArrowUp;2:+ShiftLeft" presses (+) and releases (-) keys at those times (seconds).
/// Test aid: LS_MINIMIZED=1 (set by tools/run_game.py) minimizes the window as soon as it exists, so a test run never gets in the way (with LS_SHOT the
/// window is off the screen instead: a minimized window draws nothing to take a picture of).
fn minimize_for_tests(mut windows: Query<&mut Window>, mut done: Local<bool>) {
    if *done || std::env::var("LS_MINIMIZED").is_err() {
        return;
    }
    *done = true;
    if std::env::var("LS_SHOT").is_err() {
        for mut w in &mut windows {
            w.set_minimized(true);
        }
    }
}

fn autoplay(time: Res<Time>, mut keys: ResMut<ButtonInput<KeyCode>>, mut done: Local<usize>) {
    let Ok(spec) = std::env::var("LS_AUTOPLAY") else { return };
    let steps: Vec<(f32, bool, KeyCode)> = spec
        .split(';')
        .filter_map(|s| {
            let (t, k) = s.split_once(':')?;
            let press = k.starts_with('+');
            let key = match &k[1..] {
                "ArrowUp" => KeyCode::ArrowUp,
                "ArrowDown" => KeyCode::ArrowDown,
                "ArrowLeft" => KeyCode::ArrowLeft,
                "ArrowRight" => KeyCode::ArrowRight,
                "ShiftLeft" => KeyCode::ShiftLeft,
                "Space" => KeyCode::Space,
                "KeyO" => KeyCode::KeyO,
                "KeyP" => KeyCode::KeyP,
                "KeyX" => KeyCode::KeyX,
                "Tab" => KeyCode::Tab,
                "Escape" => KeyCode::Escape,
                "F5" => KeyCode::F5,
                "F9" => KeyCode::F9,
                "KeyE" => KeyCode::KeyE,
                other => crate::config::parse_key(other)?,
            };
            Some((t.parse().ok()?, press, key))
        })
        .collect();
    while let Some(&(t, press, key)) = steps.get(*done) {
        if time.elapsed_secs() < t {
            break;
        }
        if press {
            keys.press(key);
        } else {
            keys.release(key);
        }
        *done += 1;
    }
}

/// The view is centred on the middle of the window: in a window of odd width (or height) that is half a pixel, and the
/// pictures, drawn on whole pixels, would be drawn between two pixels (lines between the blocks). The camera moves by half
/// a pixel in that case.
fn camera_parity(windows: Query<&Window>, mut cam: Query<&mut Transform, With<Camera2d>>) {
    let (Ok(w), Ok(mut c)) = (windows.single(), cam.single_mut()) else { return };
    let (ox, oy) = (if w.resolution.physical_width() % 2 == 1 { 0.5 } else { 0.0 }, if w.resolution.physical_height() % 2 == 1 { 0.5 } else { 0.0 });
    let fixed = (((c.translation.x - ox).round() + ox), (((c.translation.y - oy).round()) + oy));
    if c.translation.x != fixed.0 || c.translation.y != fixed.1 {
        c.translation.x = fixed.0;
        c.translation.y = fixed.1;
    }
}

/// Debug: LS_SHOT=file.png saves the window after a second and quits.
/// Tab is a key of the game (the item menu), and egui would take it to move the focus to the next widget of the debug panel (which then also takes the other keys):
/// egui does not get to hear it.
fn tab_is_for_the_game(mut inputs: Query<&mut EguiInput>) {
    for mut input in &mut inputs {
        input.events.retain(|e| !matches!(e, egui::Event::Key { key: egui::Key::Tab, .. }));
    }
}

fn debug_shot(mut commands: Commands, time: Res<Time>, mut done: Local<bool>, mut exit: MessageWriter<AppExit>) {
    let Ok(path) = std::env::var("LS_SHOT") else { return };
    if !*done && time.elapsed_secs() > std::env::var("LS_SHOT_AT").ok().and_then(|v| v.parse::<f32>().ok()).unwrap_or(3.0) {
        *done = true;
        commands.spawn(bevy::render::view::screenshot::Screenshot::primary_window()).observe(bevy::render::view::screenshot::save_to_disk(PathBuf::from(path)));
    } else if *done && time.elapsed_secs() > std::env::var("LS_SHOT_AT").ok().and_then(|v| v.parse::<f32>().ok()).unwrap_or(3.0) + 2.0 {
        exit.write(AppExit::Success);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    room: Res<RoomRes>,
    mut sprites: ResMut<Sprites>,
    start: Res<StartPos>,
    mut images: ResMut<Assets<Image>>,
    mut script: NonSendMut<Scripting>,
    mut sound: ResMut<Sound>,
    files: Res<DataFiles>,
) {
    commands.spawn(Camera2d);
    set_room_song(&room.room, &mut sound);
    // Nigel's picture is rebuilt every time it changes (see animate_player)
    sprites.player_texture = images.add(Image::new(Extent3d { width: 1, height: 1, depth_or_array_layers: 1 }, TextureDimension::D2, vec![0, 0, 0, 0], TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::default()));
    spawn_room(&mut commands, &asset_server, &mut layouts, &mut images, &files, &room, &mut sprites, start.0, None);
    if let Err(e) = script.load_room(&room.room) {
        println!("scripts: {:#}", e);
    }
}

/// Goes to the room asked for in the panel: the old one is removed, the new one drawn, Nigel put on its first door.
fn apply_room_request(
    mut commands: Commands,
    mut request: ResMut<RoomRequest>,
    data: Res<DataDir>,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut images: ResMut<Assets<Image>>,
    mut sprites: ResMut<Sprites>,
    roots: Query<Entity, With<WorldRoot>>,
    mut room_res_mut: ResMut<RoomRes>,
    mut solids: ResMut<Solids>,
    mut windows: Query<&mut Window>,
    (mut arrival, mut mode): (ResMut<Arrival>, ResMut<ArrivalMode>),
    mut script: NonSendMut<Scripting>,
    mut thrown: ResMut<Thrown>,
    mut sound: ResMut<Sound>,
    (files, versions): (Res<DataFiles>, Res<versions::RoomVersions>),
) {
    let Some(requested) = request.0.take() else { return };
    // a door leads to the first version of a room: the story flags choose the one that opens (a room asked for by hand is shown as it is)
    let id = if versions::take_direct() { requested } else { versions.redirect(requested, |f| script.flag(f)) };
    if id != requested {
        println!("[room] a door to {} opens {} (story flags)", requested, id);
    }
    let t0 = std::time::Instant::now();
    let log_time = std::env::var("LS_LOG_TIMING").is_ok();
    thrown.0.clear();
    let room = match Room::load(&room_dir(&data.0, id)) {
        Ok(mut r) => {
            // a replacing version has no doors of its own: it has those of the room it replaces
            let first = versions.original(id);
            if r.file.warps.is_empty() && first != id {
                r.file.warps = original_warps(&data.0, first);
            }
            r
        }
        Err(e) => {
            println!("room {}: {:#}", id, e);
            return;
        }
    };
    let t_load = t0.elapsed();
    for e in &roots {
        commands.entity(e).despawn();
    }
    solids.0.clear();
    let arrival_mode = std::mem::take(&mut mode.0);
    let start = arrival.0.take().map(|a| if arrival_mode == 0 { outside_warps(&room, a) } else { arrival_cell(&room, a, arrival_mode) }).unwrap_or_else(|| start_of(&room));
    set_room_song(&room, &mut sound);
    *room_res_mut = room_res(room);
    if let Err(e) = script.load_room(&room_res_mut.room) {
        println!("scripts: {:#}", e);
    }
    // he arrives on top of a raft that carries him (rooms 163 to 169: behaviours 35 to 38): on it, not inside it or on the bank. The other rafts (the one that falls
    // down the waterfall, rooms 498 and 426) are not his: he arrives where the door or the fall puts him
    let carrying: Vec<usize> = room_res_mut.room.file.entities.iter().enumerate().filter(|(_, e)| e.kind == 53 && (35..=38).contains(&e.behaviour)).map(|(i, _)| i).collect();
    let raft = script.entities().iter().find(|e| carrying.contains(&e.index) && (e.x - start.x).abs() <= 5.0 && (e.y - start.y).abs() <= 5.0).map(|e| (Vec2::new(e.x, e.y), e.z + e.hheight, e.hbase));
    let (start, on_raft) = match raft {
        // on the raft, in its middle (the door he came in by may be under it: see RaftArrival)
        Some((at, top, _)) => {
            commands.insert_resource(RaftArrival(Some((start.x.floor() as i32, start.y.floor() as i32))));
            (at, Some(top))
        }
        None => {
            commands.insert_resource(RaftArrival(None));
            (start, None)
        }
    };
    spawn_room(&mut commands, &asset_server, &mut layouts, &mut images, &files, &room_res_mut, &mut sprites, start, on_raft);
    let t_spawn = t0.elapsed();
    dump_cells(&room_res_mut.room);
    if let Ok(mut w) = windows.single_mut() {
        w.title = format!("Landstalker next - room {}", id);
    }
    if log_time {
        println!("[timing] room {}: load {:?}, + spawn {:?}, + scripts {:?}", id, t_load, t_spawn - t_load, t0.elapsed() - t_spawn);
    }
}

fn spawn_room(commands: &mut Commands, asset_server: &AssetServer, layouts: &mut Assets<TextureAtlasLayout>, images: &mut Assets<Image>, files: &DataFiles, room: &RoomRes, sprites: &mut Sprites, start: Vec2, start_top: Option<f32>) {

    let id = room.room.file.id;
    let dir = format!("rooms/room_{:03}", id);
    let rows = (room.room.file.map.blocks.tile_count / 32).max(1) as u32;
    let layout = layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(16), 32, rows, None, None));
    let blocks: Handle<Image> = asset_server.load(format!("{}/blocks.png", dir));
    let bsize = room.room.file.map.blocks.size;

    // the player's frames, as pixels
    let root_dir = std::env::var("BEVY_ASSET_ROOT").unwrap_or_default();
    if sprites.player_cpu.is_empty() {
        let pictures: Vec<Pic> = sprites.player.poses.values().flat_map(|v| v.pictures()).cloned().collect();
        for pic in pictures {
            if let Some(img) = files.image(&format!("sprites/sprite_000/{}", pic.file)) {
                sprites.player_cpu.insert(pic.file, img);
            }
        }
    }
    let t_cover = std::time::Instant::now();
    let cover = build_cover(room, &format!("{}/data/{}", root_dir, dir));
    if std::env::var("LS_LOG_TIMING").is_ok() {
        println!("[timing] cover map {:?}", t_cover.elapsed());
    }
    if let Ok(path) = std::env::var("LS_COVER_DUMP") {
        // debug: the map as a picture, red = priority pixel, blue = closed cell, (x0, y0) in the file name
        let mut img = image::RgbaImage::new(cover.w as u32, cover.h as u32);
        for y in 0..cover.h {
            for x in 0..cover.w {
                let i = (y * cover.w + x) as usize;
                if cover.cells[i] != 0 {
                    img.put_pixel(x as u32, y as u32, image::Rgba([255, 0, 0, 255]));
                } else if cover.sil[i] != 0 {
                    img.put_pixel(x as u32, y as u32, image::Rgba([0, 80, 255, 255]));
                }
            }
        }
        let _ = img.save(format!("{}_{}_{}.png", path, cover.x0, cover.y0));
    }
    commands.insert_resource(cover.clone());

    // z ranges: blocks < sprites (0..~3) < priority blocks
    let root = commands.spawn((Transform::from_scale(Vec3::new(SCALE, SCALE, 1.0)), Visibility::default(), WorldRoot)).id();
    let mut children: Vec<Entity> = Vec::new();
    let (swap_list, swap_flag_list) = swaps::parse(&room.room);
    let swap_cells = swaps::affected(&room.room, &swap_list, room.hm_left, room.hm_top);
    for (layer_index, (layer, shift, base_z, image)) in [("background", 16.0, -50.0, blocks.clone()), ("foreground", 0.0, -40.0, blocks.clone())].into_iter().enumerate() {
        for by in 0..bsize[1] {
            for bx in 0..bsize[0] {
                let index = room.room.block(layer, bx, by).unwrap_or(0);
                let can_change = swap_cells.contains(&(layer_index, bx, by));
                if index == 0 && !can_change {
                    continue;
                }
                let px = ((bx - by + room.block_h - 1) * 2 + room.hm_left) as f32 * 8.0 + shift;
                let py = (bx + by + room.hm_top) as f32 * 8.0;
                let z = base_z + (by * bsize[0] + bx) as f32 * 0.001;
                let sprite = Sprite::from_atlas_image(image.clone(), TextureAtlas { layout: layout.clone(), index: index as usize });
                let vis = if index == 0 { Visibility::Hidden } else { Visibility::Inherited };
                children.push(commands.spawn((sprite, vis, BlockSprite { layer: layer_index, bx, by }, Transform::from_xyz(px + 8.0, -(py + 8.0), z))).id());
            }
        }
    }
    commands.insert_resource(SwapState { base: room.room.tmx.layers.clone(), swaps: swap_list, flags: swap_flag_list });

    // the entities of the room: one sprite each, filled in every frame by `sync_entities` from the scripts' state
    for (index, e) in room.room.file.entities.iter().enumerate() {
        let texture = images.add(Image::new(Extent3d { width: 1, height: 1, depth_or_array_layers: 1 }, TextureDimension::D2, vec![0, 0, 0, 0], TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::default()));
        children.push(commands.spawn((RoomEnt { index, key: None, last: (e.position[0] as f32, e.position[1] as f32), since_moved: 1.0, walk_time: 0.0 }, Sprite::from_image(texture), Visibility::Hidden, Transform::default())).id());
    }
    // the spare rows of the projectiles of the bosses
    for k in 0..scripting::PROJECTILE_SLOTS {
        let texture = images.add(Image::new(Extent3d { width: 1, height: 1, depth_or_array_layers: 1 }, TextureDimension::D2, vec![0, 0, 0, 0], TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::default()));
        children.push(commands.spawn((RoomEnt { index: room.room.file.entities.len() + k, key: None, last: (0.0, 0.0), since_moved: 1.0, walk_time: 0.0 }, Sprite::from_image(texture), Visibility::Hidden, Transform::default())).id());
    }
    commands.insert_resource(RoomEnts(room.room.file.entities.clone()));
    // the animated blocks
    let mut groups = Vec::new();
    for a in &room.room.file.animations {
        let (Some(file), Some(frames), Some(delay)) = (a.get("file").and_then(|v| v.as_str()), a.get("frames").and_then(|v| v.as_u64()), a.get("delay").and_then(|v| v.as_u64())) else { continue };
        let list: Vec<usize> = a.get("blocks").and_then(|v| v.as_sequence()).map(|l| l.iter().filter_map(|b| b.as_u64().map(|b| b as usize)).collect()).unwrap_or_default();
        if let Ok(img) = image::open(format!("{}/data/{}/{}", root_dir, dir, file)) {
            if frames >= 2 && delay > 0 {
                groups.push(AnimGroup { atlas: img.to_rgba8(), blocks: list, frames: frames as u32, delay: delay as f32, shown: -1 });
            }
        }
    }
    commands.insert_resource(TileAnims { blocks: blocks.clone(), groups, clock: 0.0 });

    // Nigel
    let start_z = std::env::var("LS_START_Z").ok().and_then(|v| v.parse().ok()).unwrap_or_else(|| start_top.unwrap_or_else(|| room.floor_height(start.x.floor() as i32, start.y.floor() as i32))); // (test aid)
    let player = Player { x: start.x, y: start.y, z: start_z, vz: 0.0, grounded: true, air_peak: f32::MIN, facing: 3, anim_time: 0.0, moving: false, last_key: None, carrying: None, carried_solid: true, lift_phase: 0, lift_t: 0.0, drop: Vec3::ZERO, climbing: false };
    children.push(commands.spawn((player, PlayerSprite, Sprite::from_image(sprites.player_texture.clone()), Transform::default())).id());
    commands.entity(root).add_children(&children);
}

/// The cell a block shows at each height, as in RoomView::BuildPriorityBlocks: the frontmost one decides its depth.
/// Returns (closed cell shown, depth, z, wall).
fn block_cells(room: &RoomRes, bx: i32, by: i32, tiles: i32) -> (bool, f32, f32, bool) {
    let t = tiles as f32;
    let hm = &room.room.file.map;
    let off = hm.cell_offset;
    let mut best: Option<(f32, i32, bool)> = None;
    // the frontmost closed cell (a wall, a pillar, a statue) the block shows: when there is one, it decides
    let mut closed: Option<(f32, i32, bool)> = None;
    for z in 0..16 {
        let fx = bx as f32 + t / 4.0 + z as f32 - 0.5 + room.hm_left as f32;
        let fy = by as f32 - t / 4.0 + z as f32 + 0.5 + room.hm_top as f32;
        let (cx, cy) = (fx.floor() as i32, fy.floor() as i32);
        let (ix, iy) = (cx - off, cy - off);
        if ix < hm.heightmap.origin[0] || iy < hm.heightmap.origin[1] || ix >= hm.heightmap.origin[0] + hm.heightmap.size[0] || iy >= hm.heightmap.origin[1] + hm.heightmap.size[1] {
            continue;
        }
        let Some(cell) = room.room.cell(ix, iy) else { continue };
        let h = cell.height as i32;
        if h < z {
            continue;
        }
        let d = (cx + cy) as f32;
        if best.map_or(true, |b| d >= b.0) {
            best = Some((d, z, h > z));
        }
        if cell.blocked() && closed.map_or(true, |b| d >= b.0) {
            closed = Some((d, z, h > z));
        }
    }
    let (depth, z, wall) = if rule() == Rule::Mine { closed.or(best) } else { best }.unwrap_or((-1.0, 0, false));
    (closed.is_some(), depth, z as f32, wall)
}

/// Convex hull (counter-clockwise) of a few points.
fn convex_hull(mut pts: Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    pts.sort_by(|a, b| a.partial_cmp(b).unwrap());
    pts.dedup();
    let cross = |o: (f32, f32), a: (f32, f32), b: (f32, f32)| (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0);
    let mut lower: Vec<(f32, f32)> = Vec::new();
    for &p in &pts {
        while lower.len() >= 2 && cross(lower[lower.len() - 2], lower[lower.len() - 1], p) <= 0.0 {
            lower.pop();
        }
        lower.push(p);
    }
    let mut upper: Vec<(f32, f32)> = Vec::new();
    for &p in pts.iter().rev() {
        while upper.len() >= 2 && cross(upper[upper.len() - 2], upper[upper.len() - 1], p) <= 0.0 {
            upper.pop();
        }
        upper.push(p);
    }
    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}

fn inside_hull(hull: &[(f32, f32)], x: f32, y: f32) -> bool {
    let n = hull.len();
    (0..n).all(|i| {
        let (a, b) = (hull[i], hull[(i + 1) % n]);
        (b.0 - a.0) * (y - a.1) - (b.1 - a.1) * (x - a.0) >= 0.0
    })
}

/// Builds the map of the pixels that may hide a sprite: the top opaque pixel of each room pixel is looked at, in the drawing
/// order of the blocks (background, then foreground); it hides when it is a priority pixel or belongs to a block that shows
/// a closed cell (a wall, a pillar, a statue).
fn build_cover(room: &RoomRes, dir: &str) -> CoverMap {
    let bsize = room.room.file.map.blocks.size;
    let blocks = image::open(format!("{}/blocks.png", dir)).map(|i| i.to_rgba8()).ok();
    let priority = image::open(format!("{}/blocks_priority.png", dir)).map(|i| i.to_rgba8()).ok();
    let origin = |bx: i32, by: i32, tiles: i32| (((bx - by + room.block_h - 1) * 2 + room.hm_left + tiles) * 8, (bx + by + room.hm_top) * 8);
    let (mut x0, mut y0, mut x1, mut y1) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
    for by in 0..bsize[1] {
        for bx in 0..bsize[0] {
            let (px, py) = origin(bx, by, 2);
            x0 = x0.min(px - 16);
            y0 = y0.min(py);
            x1 = x1.max(px + 32);
            y1 = y1.max(py + 16);
        }
    }
    let (w, h) = ((x1 - x0).max(1), (y1 - y0).max(1));
    let mut map = CoverMap { x0, y0, w, h, cells: vec![0; (w * h) as usize], sil: vec![0; (w * h) as usize], pri: vec![false; (w * h) as usize], attrs: vec![HideAttr { depth: 0.0, z: 0.0, wall: false, priority: false }] };
    let Some(blocks) = blocks else { return map };
    let mut opaque = vec![false; (w * h) as usize];
    let mut under = vec![[0u8; 3]; (w * h) as usize];
    // the top opaque pixel comes from the foreground layer (decor: statues, braziers, trees...), not from the walls and floors
    let mut fgtop = vec![false; (w * h) as usize];
    let block_has_priority = |layer: &str, bx: i32, by: i32| -> bool {
        let Some(index) = room.room.block(layer, bx, by) else { return false };
        let Some(p) = priority.as_ref() else { return false };
        let (sx, sy) = ((index % 32 * 16) as u32, (index / 32 * 16) as u32);
        index != 0 && (0..16).any(|v| (0..16).any(|u| sx + u < p.width() && sy + v < p.height() && p.get_pixel(sx + u, sy + v)[3] > 0))
    };
    for (layer, tiles) in [("background", 2), ("foreground", 0)] {
        for by in 0..bsize[1] {
            for bx in 0..bsize[0] {
                let Some(index) = room.room.block(layer, bx, by) else { continue };
                if index == 0 {
                    continue;
                }
                let (closed, depth, z, wall) = block_cells(room, bx, by, tiles);
                if let Ok(spec) = std::env::var("LS_COVER_BLOCK") {
                    let v: Vec<i32> = spec.split(',').filter_map(|t| t.parse().ok()).collect();
                    let (px, py) = origin(bx, by, tiles);
                    if v.len() == 2 && v[0] >= px && v[0] < px + 16 && v[1] >= py && v[1] < py + 16 {
                        println!("{} block ({}, {}) index {} closed {} depth {} z {} wall {}", layer, bx, by, index, closed, depth, z, wall);
                    }
                }
                let (sx, sy) = ((index % 32 * 16) as u32, (index / 32 * 16) as u32);
                let pri = priority.as_ref();
                let pri_at = |u: u32, v: u32| pri.map_or(false, |p| sx + u < p.width() && sy + v < p.height() && p.get_pixel(sx + u, sy + v)[3] > 0);
                let has_priority = (0..16).any(|v| (0..16).any(|u| pri_at(u, v)));
                let _ = closed;
                let mut ids = [0u16; 2];
                // a priority object (a tree, a pillar) stands where its lowest block does: the blocks above it, which show
                // cells further back, take the depth of the bottom of the stack
                let (depth, z, wall) = if has_priority && rule() == Rule::Mine {
                    let mut k = 0;
                    while block_has_priority(layer, bx + k + 1, by + k + 1) {
                        k += 1;
                    }
                    let (_, d, zz, ww) = block_cells(room, bx + k, by + k, tiles);
                    (d, zz, ww)
                } else {
                    (depth, z, wall)
                };
                if has_priority {
                    map.attrs.push(HideAttr { depth, z, wall, priority: true });
                    ids[1] = (map.attrs.len() - 1) as u16;
                }
                let (px, py) = origin(bx, by, tiles);
                for v in 0..16u32 {
                    for u in 0..16u32 {
                        if sx + u >= blocks.width() || sy + v >= blocks.height() || blocks.get_pixel(sx + u, sy + v)[3] == 0 {
                            continue;
                        }
                        // an opaque pixel covers what was under it; it hides sprites only when it is a priority pixel or shows a closed cell
                        let id = if pri_at(u, v) && ids[1] != 0 { ids[1] } else { 0 };
                        let (cx, cy) = (px + u as i32 - x0, py + v as i32 - y0);
                        if cx >= 0 && cy >= 0 && cx < w && cy < h {
                            let i = (cy * w + cx) as usize;
                            let c = blocks.get_pixel(sx + u, sy + v).0;
                            if pri_at(u, v) {
                                map.pri[i] = true;
                            }
                            under[i] = [c[0], c[1], c[2]];
                            map.cells[i] = id;
                            opaque[i] = true;
                            fgtop[i] = tiles == 0;
                        }
                    }
                }
            }
        }
    }
    // silhouettes of the closed cells: the hexagon of the column from the floor around it up to its height, on opaque pixels
    let hm = &room.room.file.map;
    let off = hm.cell_offset;
    for iy in hm.heightmap.origin[1]..hm.heightmap.origin[1] + hm.heightmap.size[1] {
        for ix in hm.heightmap.origin[0]..hm.heightmap.origin[0] + hm.heightmap.size[0] {
            let Some(cell) = room.room.cell(ix, iy) else { continue };
            if matches!(rule(), Rule::Cpp | Rule::Original) || !cell.blocked() || cell.height == 0 {
                continue;
            }
            let (cx, cy) = (ix + off, iy + off);
            let top = cell.height as f32;
            let around = [(-1, 0), (1, 0), (0, -1), (0, 1)].iter().map(|(dx, dy)| room.room.cell(ix + dx, iy + dy).map_or(0.0, |c| c.height as f32)).fold(f32::MAX, f32::min);
            let bottom = around.min(top - 1.0).max(0.0);
            let mut pts: Vec<(f32, f32)> = Vec::new();
            for (dx, dy) in [(0, 0), (1, 0), (1, 1), (0, 1)] {
                for z in [top, bottom] {
                    let v = room.to_screen((cx + dx) as f32, (cy + dy) as f32, z);
                    pts.push((v.x, v.y));
                }
            }
            let hull = convex_hull(pts);
            let (minx, maxx) = hull.iter().fold((f32::MAX, f32::MIN), |a, p| (a.0.min(p.0), a.1.max(p.0)));
            let (miny, maxy) = hull.iter().fold((f32::MAX, f32::MIN), |a, p| (a.0.min(p.1), a.1.max(p.1)));
            // an object whose pixels are priority pixels (a tree, a stone) is cut by those alone: its hexagon also holds ground
            let mut priority_pixels = 0;
            for y in miny.floor() as i32..=maxy.ceil() as i32 {
                for x in minx.floor() as i32..=maxx.ceil() as i32 {
                    let (gx, gy) = (x - x0, y - y0);
                    if gx >= 0 && gy >= 0 && gx < w && gy < h && map.cells[(gy * w + gx) as usize] != 0 && inside_hull(&hull, x as f32 + 0.5, y as f32 + 0.5) {
                        priority_pixels += 1;
                    }
                }
            }
            if std::env::var("LS_SIL_DEBUG").is_ok() {
                let mut total = 0;
                for y in miny.floor() as i32..=maxy.ceil() as i32 {
                    for x in minx.floor() as i32..=maxx.ceil() as i32 {
                        let (gx, gy) = (x - x0, y - y0);
                        if gx >= 0 && gy >= 0 && gx < w && gy < h && opaque[(gy * w + gx) as usize] && inside_hull(&hull, x as f32 + 0.5, y as f32 + 0.5) {
                            total += 1;
                        }
                    }
                }
                println!("closed cell ({}, {}) h {} priority pixels {} / {} opaque in the hexagon", cx, cy, cell.height, priority_pixels, total);
            }
            if priority_pixels >= 24 {
                continue;
            }
            map.attrs.push(HideAttr { depth: (cx + cy) as f32, z: top, wall: false, priority: false });
            let id = (map.attrs.len() - 1) as u16;
            for y in miny.floor() as i32..=maxy.ceil() as i32 {
                for x in minx.floor() as i32..=maxx.ceil() as i32 {
                    let (gx, gy) = (x - x0, y - y0);
                    if gx < 0 || gy < 0 || gx >= w || gy >= h || !opaque[(gy * w + gx) as usize] || (rule() == Rule::Objects && !fgtop[(gy * w + gx) as usize]) {
                        continue;
                    }
                    if !inside_hull(&hull, x as f32 + 0.5, y as f32 + 0.5) {
                        continue;
                    }
                    let slot = &mut map.sil[(gy * w + gx) as usize];
                    // the frontmost column wins
                    if *slot == 0 || map.attrs[*slot as usize].depth < (cx + cy) as f32 {
                        *slot = id;
                    }
                }
            }
        }
    }
    map
}

/// The original game's test (`sub_44C4`): is the sprite hidden by the priority tiles? The heights of four cells in front of it
/// (the cells at +x, +y and +x+y of its own, and one more that depends on where it stands in its cell) are compared with
/// its height, then the same four cells one step further along the diagonal with the height one unit higher, 12 times:
/// as soon as one is higher, the sprite has low priority (the priority tiles cover it). With none, it is drawn on top.
fn low_priority(room: &RoomRes, x: f32, y: f32, z: f32) -> bool {
    let (cx, cy) = (x.floor() as i32, y.floor() as i32);
    let sub = |v: f32| ((v - v.floor()) * 16.0) as i32;
    let (sx, sy) = (sub(x), sub(y));
    // the four cells as offsets from the sprite's cell: a3 (+1, 0), a4 (+1, +1), a5 (0, +1), a6
    let mut a3 = (1, 0);
    let mut a4 = (1, 1);
    let mut a5 = (0, 1);
    let mut a6 = if sx == sy {
        a4
    } else if sx > sy {
        if sx == 8 && sy == 7 { a3 } else { (a3.0, a3.1 - 1) }
    } else if sy == 8 && sx == 7 {
        a5
    } else {
        (a5.0 - 1, a5.1)
    };
    let mut d4 = false;
    if sx > 8 {
        d4 = true;
        a3 = (a3.0 + 1, a3.1 + 1);
        a6 = (a6.0 + 1, a6.1 + 1);
    }
    if sy > 8 {
        a5 = (a5.0 + 1, a5.1 + 1);
        a6 = (a6.0 + 1, a6.1 + 1);
        if d4 {
            a4 = (a4.0 + 1, a4.1 + 1);
            a6 = (a6.0 - 1, a6.1 - 1);
        }
    }
    let mut line = z.floor() as i32;
    for _ in 0..12 {
        for (dx, dy) in [a3, a4, a5, a6] {
            let h = room.room.cell_at_world(cx + dx, cy + dy).map_or(0, |c| c.height as i32);
            if h > line {
                return true;
            }
        }
        for a in [&mut a3, &mut a4, &mut a5, &mut a6] {
            *a = (a.0 + 1, a.1 + 1);
        }
        line += 1;
    }
    false
}

/// The action key (Left Shift): lift the crate (or vase...) in front of Nigel, put down or throw what he holds.
fn interact(ctl: Res<Controls>, mut game: ResMut<GameState>, mut msg: ResMut<Message>, mut chest: ResMut<ChestOpen>, mut sfx: ResMut<SfxQueue>, script: NonSend<Scripting>, room: Res<RoomRes>, tuning: Res<TuningRes>, frozen: Res<GameFrozen>, cut: Res<cutscene::Cutscene>, mut thrown: ResMut<Thrown>, mut q: Query<&mut Player>) {
    if frozen.0 || !ctl.action.just || chest.0.is_some() || cut.active() {
        return;
    }
    let Ok(mut p) = q.single_mut() else { return };
    let _ = (&room, &tuning);
    const DIR_X: [f32; 4] = [-1.0, 0.0, 1.0, 0.0]; // facing: NW NE SE SW
    const DIR_Y: [f32; 4] = [0.0, -1.0, 0.0, 1.0];
    let f = (p.facing & 3) as usize;
    let snaps = script.entities();
    if let Some(i) = p.carrying {
        if p.lift_phase != 0 {
            return;
        }
        let Some(e) = snaps.get(i) else { return };
        let direction_held = ctl.dir.iter().any(|b| b.pressed);
        if p.grounded && !direction_held {
            // put it down: on the cell in front of him (exactly one cell on, like the original), from over his head, and it falls there
            let (mut nx, mut ny) = (p.x + DIR_X[f], p.y + DIR_Y[f]);
            // on a counter or a lectern (floor type 0x0E) what he puts down lies in the middle of the cell
            if room.room.cell_at_world(nx.floor() as i32, ny.floor() as i32).is_some_and(|c| c.floor_type == 0x0E) {
                (nx, ny) = (nx.floor() + 0.5, ny.floor() + 0.5);
            }
            let mut z = script.cell_floor(nx, ny, e.hbase);
            if let Some(top) = platform_top(&snaps, nx, ny, p.z + CARRY_HEIGHT, i, e.hbase) {
                z = z.max(top);
            }
            if std::env::var("LS_LOG_DROP").is_ok() {
                println!("[drop] facing {} at ({:.2},{:.2}) z {:.2} -> ({:.2},{:.2}) floor {:.2}: step ok {} can_place {}", p.facing, p.x, p.y, p.z, nx, ny, z, z <= p.z + 1.0 + 1e-4, script.can_place(i, nx, ny, z));
            }
            if z <= p.z + 1.0 + 1e-4 && script.can_place(i, nx, ny, z) {
                script.set_f(i, "x", nx);
                script.set_f(i, "y", ny);
                script.set_f(i, "z", p.z + CARRY_HEIGHT);
                script.set_b(i, "carried", false);
                script.set_b(i, "solid", p.carried_solid);
                thrown.0.push(ThrownItem { index: i, vx: 0.0, vy: 0.0, vz: 0.0, t: 0.0 });
                p.carrying = None;
                p.lift_phase = 2;
                p.lift_t = 0.0;
                sfx.play(0x63); // NigelDropObj1
            }
        } else {
            // throw it (a direction held, or in the air)
            script.set_b(i, "carried", false);
            script.set_b(i, "solid", p.carried_solid);
            thrown.0.push(ThrownItem { index: i, vx: DIR_X[f] * 7.0, vy: DIR_Y[f] * 7.0, vz: 3.0, t: 0.0 });
            p.carrying = None;
            sfx.play(0x56); // Throw
        }
        return;
    }
    if !p.grounded {
        return;
    }
    // lift: something pickable in front of him
    let reach = physics::PLAYER_RADIUS + 0.4;
    let (qx, qy) = (p.x + DIR_X[f] * reach, p.y + DIR_Y[f] * reach);
    let mut best: Option<(usize, f32)> = None;
    for e in &snaps {
        if !e.pickable || !LIFTABLE.contains(&e.kind) || !e.visible || e.hostile || e.talks || e.dying || e.carried || thrown.0.iter().any(|t| t.index == e.index) {
            continue;
        }
        let half = e.hbase + 0.35;
        if (qx - e.x).abs() > half || (qy - e.y).abs() > half {
            continue;
        }
        let top = e.z + e.hheight;
        // (a book on its shelf is at the height of Nigel's waist: it can be taken from the floor)
        let reach = if e.kind >= 200 { 1.2 } else { 0.8 };
        if p.z >= top || p.z + physics::PLAYER_HEIGHT <= e.z || (p.z - e.z).abs() > reach {
            continue;
        }
        let d = ((qx - e.x).powi(2) + (qy - e.y).powi(2)).sqrt();
        if best.map_or(true, |b| d < b.1) {
            best = Some((e.index, d));
        }
    }
    if let Some((i, _)) = best {
        p.carried_solid = snaps[i].solid;
        script.set_b(i, "carried", true);
        script.set_b(i, "solid", false);
        script.set_b(i, "falls", false);
        p.carrying = Some(i);
        p.drop = Vec3::new(snaps[i].x, snaps[i].y, snaps[i].z);
        p.lift_phase = 1;
        p.lift_t = 0.0;
        sfx.play(0x43); // CursorSelect
        return;
    }
    // a chest in front of him (entity 18; the contents come from the room's list, in the order the chests appear)
    let mut ordinal = 0usize;
    let file = &room.room.file;
    for e in snaps.iter().filter(|e| e.kind == 18 && e.visible) {
        let this = ordinal;
        ordinal += 1;
        let half = e.hbase + 0.2;
        if (qx - e.x).abs() > half || (qy - e.y).abs() > half {
            continue;
        }
        if p.z >= e.z + e.hheight || p.z + physics::PLAYER_HEIGHT <= e.z {
            continue;
        }
        if game.opened.contains(&(file.id, e.index)) {
            msg.show(crate::i18n::t("messages.pickup.chest_empty"));
            return;
        }
        game.opened.insert((file.id, e.index));
        let content = if file.chests.enabled { file.chests.contents.get(this).copied().unwrap_or(-1) } else { -1 };
        chest.0 = Some(ChestSeq { entity: e.index, content, t: 0.0, given: false, item_sprite: None });
        return;
    }
}

/// The chest opens, its content is given (an item with a fanfare, or gold) and held up.
fn update_chest(
    mut commands: Commands,
    time: Res<Time>,
    script: NonSend<Scripting>,
    room: Res<RoomRes>,
    sprites: Res<Sprites>,
    files: Res<DataFiles>,
    mut images: ResMut<Assets<Image>>,
    mut chest: ResMut<ChestOpen>,
    mut game: ResMut<GameState>,
    mut msg: ResMut<Message>,
    mut sfx: ResMut<SfxQueue>,
    mut sound: ResMut<Sound>,
    settings: Res<audio::AudioSettings>,
    roots: Query<Entity, With<WorldRoot>>,
    mut item: Query<&mut Transform, With<ChestItem>>,
) {
    let Some(seq) = chest.0.as_mut() else { return };
    seq.t += time.delta_secs().min(0.05);
    let snaps = script.entities();
    let Some(c) = snaps.get(seq.entity) else {
        chest.0 = None;
        return;
    };
    if !seq.given && seq.t >= CHEST_OPEN_TIME {
        seq.given = true;
        match seq.content {
            v if (58..=61).contains(&v) => {
                let gold = [5, 20, 50, 200][(v - 58) as usize];
                game.gold += gold;
                sfx.play(0x6A);
                msg.show(i18n::tf("messages.pickup.found_gold", &[("n", gold.to_string())]));
            }
            63 | -1 => msg.show(crate::i18n::t("messages.pickup.chest_empty")),
            v if v >= 0x40 => {
                game.gold += v - 0x40;
                sfx.play(0x6A);
                msg.show(i18n::tf("messages.pickup.found_gold", &[("n", (v - 0x40).to_string())]));
            }
            v => {
                // the fanfare of the chest (MusicChestOpen): over the music of the room, or in its place like the original
                if settings.chest_keeps_music {
                    sound.request_over(0x05);
                } else {
                    sound.request_jingle(0x05, 2.6);
                }
                game.add_item(v);
                msg.show(i18n::tf("messages.pickup.chest_opened", &[("item", game.item_name(v))]));
                // the item rises above the chest
                if let Some(pic) = entity_picture(0xC0 + v, 0, None, &sprites.entities, "", None, None) {
                    if let (Some(img), Ok(root)) = (files.image(&pic.path), roots.single()) {
                        let e = commands
                            .spawn((ChestItem, Sprite::from_image(images.add(Image::new(Extent3d { width: img.width(), height: img.height(), depth_or_array_layers: 1 }, TextureDimension::D2, img.into_raw(), TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::default()))), Transform::default()))
                            .id();
                        commands.entity(root).add_child(e);
                        seq.item_sprite = Some(e);
                    }
                }
            }
        }
    }
    // the item: rising 1 unit over 0.4 s, 1.2 units above the chest
    if let (Some(_), Ok(mut tf)) = (seq.item_sprite, item.single_mut()) {
        let rise = ((seq.t - CHEST_OPEN_TIME) / 0.4).clamp(0.0, 1.0);
        let content = seq.content;
        if let Some(pic) = entity_picture(0xC0 + content, 0, None, &sprites.entities, "", None, None) {
            let p = snap(room.to_screen(c.x, c.y, c.z + 1.2 + rise));
            let (left, top) = (p.x - pic.ax as f32, p.y - pic.ay as f32);
            tf.translation = Vec3::new(left + pic.w as f32 / 2.0, -(top + pic.h as f32 / 2.0), depth(c.x, c.y, c.z) + 0.5);
        }
    }
    if seq.t >= CHEST_OPEN_TIME + CHEST_REVEAL_TIME {
        if let Some(e) = seq.item_sprite {
            commands.entity(e).despawn();
        }
        chest.0 = None;
    }
}

/// The top of the highest object (not an enemy) under (x, y) that is not higher than `z` (+ a little).
fn platform_top(snaps: &[scripting::EntSnapshot], x: f32, y: f32, z: f32, except: usize, radius: f32) -> Option<f32> {
    let mut best: Option<f32> = None;
    for e in snaps {
        if e.index == except || !e.solid || !e.visible || e.hostile || e.talks || e.dying || e.killable || e.carried || e.kind >= 0xC0 {
            continue;
        }
        let top = e.z + e.hheight;
        if top > z + 0.05 || best.map_or(false, |b| top <= b) {
            continue;
        }
        if (x - e.x).abs() < e.hbase + radius && (y - e.y).abs() < e.hbase + radius {
            best = Some(top);
        }
    }
    best
}

/// What Nigel holds follows him; what he threw flies, lands, and vases break.
fn update_carry(time: Res<Time>, script: NonSend<Scripting>, tuning: Res<TuningRes>, frozen: Res<GameFrozen>, mut thrown: ResMut<Thrown>, mut q: Query<&mut Player>) {
    if frozen.0 {
        return;
    }
    let dt = time.delta_secs().min(0.05);
    let Ok(mut p) = q.single_mut() else { return };
    if p.carrying.is_none() && p.lift_phase == 2 {
        p.lift_t += dt;
        if p.lift_t >= LIFT_TIME {
            p.lift_phase = 0;
            p.lift_t = 0.0;
        }
    }
    if let Some(i) = p.carrying {
        match p.lift_phase {
            1 => {
                p.lift_t += dt;
                let k = (p.lift_t / (LIFT_TIME * 0.7)).min(1.0);
                script.set_f(i, "x", p.drop.x + (p.x - p.drop.x) * k);
                script.set_f(i, "y", p.drop.y + (p.y - p.drop.y) * k);
                script.set_f(i, "z", p.drop.z + (p.z + CARRY_HEIGHT - p.drop.z) * k);
                if p.lift_t >= LIFT_TIME {
                    p.lift_phase = 0;
                    p.lift_t = 0.0;
                }
            }
            2 => {
                p.lift_t += dt;
                let k = (p.lift_t / LIFT_TIME).min(1.0);
                script.set_f(i, "x", p.x + (p.drop.x - p.x) * k);
                script.set_f(i, "y", p.y + (p.drop.y - p.y) * k);
                script.set_f(i, "z", (p.z + CARRY_HEIGHT) + (p.drop.z - (p.z + CARRY_HEIGHT)) * k);
                if p.lift_t >= LIFT_TIME {
                    script.set_f(i, "x", p.drop.x);
                    script.set_f(i, "y", p.drop.y);
                    script.set_f(i, "z", p.drop.z);
                    script.set_b(i, "carried", false);
                    script.set_b(i, "solid", p.carried_solid);
                    p.carrying = None;
                    p.lift_phase = 0;
                }
            }
            _ => {
                script.set_f(i, "x", p.x);
                script.set_f(i, "y", p.y);
                script.set_f(i, "z", p.z + CARRY_HEIGHT);
            }
        }
    }
    if thrown.0.is_empty() {
        return;
    }
    let snaps = script.entities();
    let mut landed = Vec::new();
    for (n, t) in thrown.0.iter_mut().enumerate() {
        let Some(e) = snaps.get(t.index) else { continue };
        t.t += dt;
        let (mut x, mut y, mut z) = (e.x, e.y, e.z);
        if t.vx != 0.0 || t.vy != 0.0 {
            let (nx, ny) = (x + t.vx * dt, y + t.vy * dt);
            let walkable = script.cell_floor(nx, ny, 0.0) >= 0.0 && script.can_place(t.index, nx, ny, z);
            if t.t < 0.15 || walkable {
                x = nx;
                y = ny;
            } else {
                t.vx = 0.0;
                t.vy = 0.0;
            }
            t.vx *= 1.0 - 0.8 * dt;
            t.vy *= 1.0 - 0.8 * dt;
        }
        t.vz -= tuning.0.gravity * dt;
        z += t.vz * dt;
        let mut floor = script.cell_floor(x, y, e.hbase);
        if let Some(top) = platform_top(&snaps, x, y, e.z + 0.05, t.index, e.hbase * 0.6) {
            floor = floor.max(top);
        }
        script.set_f(t.index, "x", x);
        script.set_f(t.index, "y", y);
        if t.t > 0.08 && z <= floor + 1e-4 && t.vz <= 0.0 {
            script.set_f(t.index, "z", floor);
            if e.kind == 17 {
                script.set_b(t.index, "visible", false); // a vase shatters
            }
            landed.push(n);
        } else {
            script.set_f(t.index, "z", z);
        }
    }
    for n in landed.into_iter().rev() {
        thrown.0.remove(n);
    }
}

/// Changes the pictures of the animated blocks (writes them into the blocks atlas).
fn animate_tiles(time: Res<Time>, anims: Option<ResMut<TileAnims>>, mut images: ResMut<Assets<Image>>, frozen: Res<GameFrozen>) {
    let Some(mut a) = anims else { return };
    if a.groups.is_empty() {
        return;
    }
    if !frozen.0 {
        a.clock += time.delta_secs();
    }
    let a = &mut *a;
    let Some(mut atlas) = images.get_mut(&a.blocks) else { return };
    let (width, data) = (atlas.width() as usize, atlas.data.as_mut());
    let Some(data) = data else { return };
    for g in &mut a.groups {
        let frame = ((a.clock * 60.0 / g.delay) as u32 % g.frames) as i32;
        if frame == g.shown {
            continue;
        }
        g.shown = frame;
        for (u, &b) in g.blocks.iter().enumerate() {
            let (bx, by) = ((b % 32) * 16, (b / 32) * 16);
            for row in 0..16usize {
                for col in 0..16usize {
                    let sx = (u * 16 + col) as u32;
                    let sy = frame as u32 * 16 + row as u32;
                    if sx >= g.atlas.width() || sy >= g.atlas.height() {
                        continue;
                    }
                    let px = g.atlas.get_pixel(sx, sy).0;
                    let at = ((by + row) * width + bx + col) * 4;
                    if at + 4 <= data.len() {
                        data[at..at + 4].copy_from_slice(&px);
                    }
                }
            }
        }
    }
}

/// A flag changed: the map is rebuilt from the swaps that hold now, and the blocks drawn follow.
fn update_swaps(
    mut commands: Commands,
    script: NonSend<Scripting>,
    mut room: ResMut<RoomRes>,
    state: Option<Res<SwapState>>,
    mut blocks: Query<(&BlockSprite, &mut Sprite, &mut Visibility)>,
) {
    if !script.take_swaps_dirty() {
        return;
    }
    let Some(state) = state else { return };
    if state.swaps.is_empty() || state.flags.is_empty() {
        return;
    }
    let (hm_left, hm_top) = (room.hm_left, room.hm_top);
    swaps::apply(&mut room.room, &state.base, &state.swaps, &state.flags, &|f| script.flag(f), hm_left, hm_top);
    const LAYERS: [&str; 2] = ["background", "foreground"];
    for (b, mut sprite, mut vis) in &mut blocks {
        let index = room.room.block(LAYERS[b.layer], b.bx, b.by).unwrap_or(0);
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            atlas.index = index as usize;
        }
        let want = if index == 0 { Visibility::Hidden } else { Visibility::Inherited };
        if *vis != want {
            *vis = want;
        }
    }
    let root_dir = std::env::var("BEVY_ASSET_ROOT").unwrap_or_default();
    commands.insert_resource(build_cover(&room, &format!("{}/data/rooms/room_{:03}", root_dir, room.room.file.id)));
}

/// The sounds the scripts asked for.
fn drain_script_sfx(script: NonSend<Scripting>, mut sfx: ResMut<SfxQueue>) {
    for id in script.take_sfx() {
        sfx.play(id);
    }
}

/// Tells the sound system which song the room wants (when a room is loaded).
fn set_room_song(room: &Room, sound: &mut Sound) {
    sound.room_song = audio::room_song(room.file.music, false);
}

/// Runs the scripts: 60 ticks a second, one per frame of the console (measured on the ROM: an entity of speed 2 moves 2/16 of a cell every frame),
/// after telling them where the player is.
fn script_tick(
    time: Res<Time>,
    mut acc: Local<f32>,
    mut stuck: Local<i32>,
    script: NonSend<Scripting>,
    room: Res<RoomRes>,
    solids: Res<Solids>,
    tuning: Res<TuningRes>,
    frozen: Res<GameFrozen>,
    combat: Res<Combat>,
    cut: Res<cutscene::Cutscene>,
    mut players: Query<&mut Player>,
) {
    if frozen.0 || combat.dead || cut.busy > 0.0 || cut.tracking() {
        return;
    }
    let Ok(mut p) = players.single_mut() else { return };
    *acc += time.delta_secs().min(0.1);
    const TICK: f32 = 1.0 / 60.0;
    while *acc >= TICK {
        *acc -= TICK;
        let world = World { terrain: &room.room, solids: &solids.0, tuning: tuning.0 };
        let mut support = if p.grounded { world.support_of(p.x, p.y, p.z).map_or(-1, |i| i as i32) } else { -1 };
        // a platform that moves several times a frame (the raft): the solids are those of the start of the frame, so he is kept on the one he stood on while his feet
        // are still over its top at its place now
        if support < 0 && p.grounded && *stuck > 0 {
            if let Some(e) = script.entities().into_iter().find(|e| e.index as i32 + 1 == *stuck) {
                if (p.x - e.x).abs() <= e.hbase + 0.3 && (p.y - e.y).abs() <= e.hbase + 0.3 && (p.z - (e.z + e.hheight)).abs() < 0.2 {
                    support = *stuck - 1;
                }
            }
        }
        *stuck = support + 1;
        if let Some(i) = std::env::var("LS_ON").ok().and_then(|v| v.parse().ok()) {
            support = i; // test aid: pretend the player stands on this entity
        }
        script.set_player(p.x, p.y, p.z, p.facing as i32, p.grounded, support);
        let before = if support >= 0 { script.entity_pos(support) } else { None };
        script.tick();
        // the player standing on a platform that moves goes with it
        if let (Some((bx, by, bz)), Some((ax, ay, az))) = (before, if support >= 0 { script.entity_pos(support) } else { None }) {
            p.x += ax - bx;
            p.y += ay - by;
            p.z += az - bz;
        }
        if let Some(dir) = script.take_player_turn() {
            // entity direction (0 NE, 1 SE, 2 SW, 3 NW) -> the player's facing (0 NW, 1 NE, 2 SE, 3 SW)
            p.facing = [1u8, 2, 3, 0][(dir & 3) as usize];
        }
    }
}

/// Puts the sprites and the solids of the room's entities where the scripts have them. A sprite hidden by the priority tiles
/// (see `low_priority`) has its picture cut at their pixels, like Nigel.
fn sync_entities(
    players: Query<&Player>,
    script: NonSend<Scripting>,
    room: Res<RoomRes>,
    data: Res<RoomEnts>,
    sprites: Res<Sprites>,
    cover: Option<Res<CoverMap>>,
    files: Res<DataFiles>,
    time: Res<Time>,
    game: Res<GameState>,
    chest: Res<ChestOpen>,
    mut cpu: ResMut<CpuImages>,
    mut images: ResMut<Assets<Image>>,
    mut solids: ResMut<Solids>,
    mut q: Query<(&mut RoomEnt, &mut Transform, &mut Visibility, &mut Sprite)>,
) {
    let snaps = script.entities();
    solids.0 = snaps
        .iter()
        // (the items lying about are not solid, the books of the churches are: Nigel does not walk through them)
        .filter(|e| e.solid && e.visible && (e.kind < 0xC0 || shops::CHURCH_BOOKS.contains(&e.kind)) && e.hbase > 0.0)
        .map(|e| Solid { x: e.x, y: e.y, z: e.z, half: e.hbase, height: e.hheight, entity: e.index })
        .collect();
    let dir = format!("rooms/room_{:03}", room.room.file.id);
    for (mut re, mut tf, mut vis, mut sprite) in &mut q {
        let Some(s) = snaps.get(re.index) else { continue };
        // a dying enemy flickers
        let want = if s.visible && !(s.dying && (s.die_t * 20.0) as i32 % 2 == 1) { Visibility::Inherited } else { Visibility::Hidden };
        if *vis != want {
            *vis = want;
        }
        if !s.visible {
            continue;
        }
        // walking: the picture changes while the entity moves
        let moved = (s.x - re.last.0).abs() + (s.y - re.last.1).abs() > 1e-4;
        re.last = (s.x, s.y);
        if moved {
            re.since_moved = 0.0;
            re.walk_time += time.delta_secs();
        } else {
            re.since_moved += time.delta_secs();
            if re.since_moved > 0.12 {
                re.walk_time = 0.0;
            }
        }
        let walking = re.since_moved <= 0.12 && re.walk_time > 0.0;
        let pic = if s.kind == 18 {
            // a chest: closed, opening (4 pictures over the lid time), open
            let opening = chest.0.as_ref().filter(|c| c.entity == s.index);
            let frame = if let Some(c) = opening {
                1 + ((c.t / (CHEST_OPEN_TIME / 4.0)) as usize).min(3)
            } else if game.opened.contains(&(room.room.file.id, s.index)) {
                usize::MAX
            } else {
                0
            };
            chest_picture(&sprites, frame, s.orientation)
        } else {
            // what an enemy left behind (a bag of gold) and the projectiles (no entity of the room file) have no room pictures
            let room_poses = if s.pickup_gold > 0 || s.pickup_item >= 0 { None } else { data.0.get(re.index).and_then(|d| d.poses.as_ref()) };
            entity_picture(s.kind, s.orientation, room_poses, &sprites.entities, &dir, if walking { Some(re.walk_time) } else { None }, if s.pose > 0 { Some((s.pose, s.pose_p)) } else { None })
        };
        let Some(pic) = pic else { continue };
        let flip = pic.mirror;
        let p = snap(room.to_screen(s.x, s.y, s.z));
        let ax = if flip { pic.w - pic.ax } else { pic.ax } as f32;
        let (left, top) = (p.x - ax, p.y - pic.ay as f32);
        tf.translation = Vec3::new(left + pic.w as f32 / 2.0, -(top + pic.h as f32 / 2.0), entity_depth(s, &snaps, players.single().is_ok_and(|p| p.carrying == Some(s.index) && p.lift_phase == 1)));
        if std::env::var("LS_DBG_DEPTH").is_ok() && (s.index == 0 || s.index == 2) {
            println!("ent {} kind {} pos {:.3} {:.3} {:.3} h {:.2} base {:.2} depth {:.4}", s.index, s.kind, s.x, s.y, s.z, s.hheight, s.hbase, tf.translation.z);
        } // what Nigel holds is over his head, whatever the order of the frame
        let low = rule() == Rule::Original && low_priority(&room, s.x, s.y, s.z);
        let key = (pic.path.clone(), flip, (left * 3.0).round() as i32, (top * 3.0).round() as i32, low);
        if re.key.as_ref() == Some(&key) {
            continue;
        }
        re.key = Some(key);
        let src = cpu.0.entry(pic.path.clone()).or_insert_with(|| files.image(&pic.path));
        let Some(src) = src.as_ref() else { continue };
        let (w, h) = src.dimensions();
        let mut buf = Vec::with_capacity((w * h * 4) as usize);
        for j in 0..h {
            for i in 0..w {
                let sx = if flip { w - 1 - i } else { i };
                let mut px = src.get_pixel(sx, j).0;
                if low && px[3] != 0 && cover.as_ref().map_or(false, |c| c.priority_at((left + i as f32 + 0.5).floor() as i32, (top + j as f32 + 0.5).floor() as i32)) {
                    px[3] = 0;
                }
                buf.extend_from_slice(&px);
            }
        }
        // a new image each time (changing the size of an image in place made a sprite that was hidden when it was
        // created stay invisible)
        sprite.image = images.add(Image::new(Extent3d { width: w, height: h, depth_or_array_layers: 1 }, TextureDimension::D2, buf, TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::default()));
    }
}

/// Draw order of a room entity: by its place, but in front of whatever it stands on (a crate on a switch), and a held object
/// over Nigel's head.
fn entity_depth(e: &scripting::EntSnapshot, all: &[scripting::EntSnapshot], lifting: bool) -> f32 {
    let mut d = depth(e.x, e.y, e.z);
    // being lifted it is still where it was on its way up: in front of Nigel or behind him, like the cell it comes from
    if e.carried && !lifting {
        return d + 0.03;
    }
    for o in all {
        if o.index == e.index || !o.visible || !o.solid || o.carried || o.kind >= 0xC0 {
            continue;
        }
        let top = o.z + o.hheight;
        if (e.z - top).abs() < 0.1 && e.z > o.z && (e.x - o.x).abs() < o.hbase + e.hbase && (e.y - o.y).abs() < o.hbase + e.hbase {
            d = d.max(entity_depth_base(o) + 0.002);
        }
    }
    d
}

fn entity_depth_base(o: &scripting::EntSnapshot) -> f32 {
    depth(o.x, o.y, o.z)
}

/// Positions in whole screen pixels (SCALE screen pixels per pixel of the original picture).
fn snap(v: Vec2) -> Vec2 {
    Vec2::new((v.x * SCALE).round() / SCALE, (v.y * SCALE).round() / SCALE)
}

/// Distance between two cell depths in z: sprites are ordered by the x + y of their cell first (like the blocks that may hide
/// them), then by the position inside the cell and by height.
const CELL_Z: f32 = 0.1;

/// Draw order of sprites: the further down-right (x + y) and the higher, the later.
fn depth(x: f32, y: f32, z: f32) -> f32 {
    let (fx, fy) = (x.floor(), y.floor());
    (fx + fy) * CELL_Z + ((x - fx) + (y - fy)) * 0.01 + z * 0.0005
}

/// Depth of the player: by position like everything else, but in front of every object he stands on or jumps over
/// (the centres alone would draw him behind a button he is standing on when he is a little to its back).
fn player_depth(p: &Player, solids: &[Solid]) -> f32 {
    let mut d = depth(p.x, p.y, p.z) + 0.0005;
    for s in solids {
        let overlaps = (p.x - s.x).abs() < s.half + physics::PLAYER_RADIUS && (p.y - s.y).abs() < s.half + physics::PLAYER_RADIUS;
        if overlaps && p.z >= s.top() - 0.06 {
            d = d.max(depth(s.x, s.y, s.z) + 0.002);
        }
    }
    d
}

/// Which picture an entity shows: the room's own rendering of its `idle` pose when it has one, else the shared poses of its type.
/// The view and the mirroring come from its direction (the sprite's `directions`: back view for NE / NW, front view for SE / SW,
/// mirrored for NW and SE, unless it says otherwise).
/// `pose`: hurt (1), attacking (2) or spinning (3) enemies show their own pictures, and how far through the pose they are (0..1).
fn entity_picture(kind: i32, orientation: i32, room_poses: Option<&Poses>, defs: &HashMap<u32, EntityDef>, room_dir: &str, walk_time: Option<f32>, pose: Option<(i32, f32)>) -> Option<Picture> {
    static CONSOLE: std::sync::LazyLock<Directions> = std::sync::LazyLock::new(Directions::default);
    let dir = Dir::from_orientation(orientation);
    let def = defs.get(&(kind as u32));
    let dirs = def.map_or(&*CONSOLE, |d| &d.directions);
    let base = format!("sprites/entities/entity_{:03}", kind);
    if let (Some((pose, progress)), Some(d)) = (pose, def) {
        if let Some((list, mirror)) = d.poses.get(if pose == 1 { "hurt" } else { "attack" }).and_then(|v| v.pick(dir, dirs)) {
            let n = list.len();
            let k = if pose == 3 { (progress * n as f32) as usize % n } else { ((progress.max(0.0) * n as f32) as usize).min(n - 1) };
            return Some(picture_of(&base, &list[k], mirror));
        }
    }
    // walking: the pictures of the walk, one every ENEMY_WALK_FRAME_TIME (0.18 s), when the entity has some for this side (a room that only
    // gives a still picture of a character does not stop it from walking when it moves: the old man of the intro)
    if let (Some(t), Some(d)) = (walk_time, def) {
        if room_poses.map_or(true, |p| p.get("walk").is_none()) {
            if let Some((list, mirror)) = d.poses.get("walk").and_then(|v| v.pick(dir, dirs)) {
                return Some(picture_of(&base, &list[(t / 0.18) as usize % list.len()], mirror));
            }
        }
    }
    if let Some((list, mirror)) = room_poses.and_then(|p| p.get("idle")).and_then(|v| v.pick(dir, dirs)) {
        return Some(picture_of(room_dir, &list[0], mirror));
    }
    let d = def?;
    if let (Some(t), true) = (walk_time, false) {
        if let Some((list, mirror)) = d.poses.get("walk").and_then(|v| v.pick(dir, dirs)) {
            return Some(picture_of(&base, &list[(t / 0.18) as usize % list.len()], mirror));
        }
    }
    let (list, mirror) = d.poses.get("idle").and_then(|v| v.pick(dir, dirs))?;
    Some(picture_of(&base, &list[0], mirror))
}

// ---------------------------------------------------------------------------------------------------------------
// the player
// ---------------------------------------------------------------------------------------------------------------

fn walk(mut ladder_t: Local<f32>, config: Res<config::Config>, time: Res<Time>, cut: Res<cutscene::Cutscene>, combat: Res<Combat>, game: Res<GameState>, chest: Res<ChestOpen>, ctl: Res<Controls>, mut sfx: ResMut<SfxQueue>, room: Res<RoomRes>, solids: Res<Solids>, mut tuning: ResMut<TuningRes>, frozen: Res<GameFrozen>, mut q: Query<&mut Player>) {
    if frozen.0 || cut.tracking() {
        return;
    }
    if ctl.reload_physics.just {
        tuning.0 = load_tuning();
        println!("physics: {:?} (jump speed {:.1})", tuning.0, tuning.0.jump_speed());
    }
    let Ok(mut p) = q.single_mut() else { return };
    let dt = time.delta_secs().min(0.05);
    let confused = game.has_status(game::STATUS_CONFUSION);
    let (mut dx, mut dy);
    if config.original_directions {
        // the keys as the original reads them: two together are a diagonal of the screen, one alone goes along the axis he faces (see `physics::input_direction`)
        (dx, dy) = physics::input_direction([ctl.dir[0].pressed, ctl.dir[1].pressed, ctl.dir[2].pressed, ctl.dir[3].pressed], p.facing, confused);
    } else {
        // one direction at a time, the most recently pressed key wins: up north-west, right north-east, down south-east, left south-west
        const DIRS: [(f32, f32); 4] = [(-1.0, 0.0), (1.0, 0.0), (0.0, 1.0), (0.0, -1.0)];
        for i in 0..4 {
            if ctl.dir[i].just {
                p.last_key = Some(i);
            }
        }
        if let Some(i) = p.last_key {
            if !ctl.dir[i].pressed {
                p.last_key = None;
            }
        }
        if p.last_key.is_none() {
            p.last_key = (0..4).find(|&i| ctl.dir[i].pressed);
        }
        (dx, dy) = (0.0, 0.0);
        if let Some(i) = p.last_key {
            // confused: the controls are reversed
            let sign = if confused { -1.0 } else { 1.0 };
            dx = DIRS[i].0 * sign;
            dy = DIRS[i].1 * sign;
        }
    }
    let keyed = dx != 0.0 || dy != 0.0;
    // ice (floor 0x2A NE, 0x2B SE, 0x2C SW, 0x2D NW) pushes Nigel along unless he wears the Snow Spikes
    let locked = combat.locked();
    if !locked && p.grounded && game.boots != game::BOOTS_SNOW {
        if let Some(c) = room.room.cell_at_world(p.x.floor() as i32, p.y.floor() as i32) {
            if std::env::var("LS_DBG_FLOOR").is_ok() && c.floor_type != 0 {
                println!("floor type {:#x} at {:.2} {:.2}", c.floor_type, p.x, p.y);
            }
            match c.floor_type {
                0x2A => (dx, dy) = (0.0, -1.0),
                0x2B => (dx, dy) = (1.0, 0.0),
                0x2C => (dx, dy) = (0.0, 1.0),
                0x2D => (dx, dy) = (-1.0, 0.0),
                _ => {}
            }
        }
    }
    // swinging the sword, thrown back, dead: no walking; a scene of the story: only the recorded input moves him
    let locked = locked || cut.blocks_player();
    if locked {
        dx = 0.0;
        dy = 0.0;
    }
    if cut.input_locked && !locked {
        (dx, dy) = cut.playback_direction().unwrap_or((0.0, 0.0));
    }
    if dx != 0.0 || dy != 0.0 {
        let candidates: Vec<u8> = [(dx < 0.0, 0u8), (dy < 0.0, 1), (dx > 0.0, 2), (dy > 0.0, 3)].iter().filter(|c| c.0).map(|c| c.1).collect();
        if !candidates.contains(&p.facing) {
            p.facing = candidates[0];
        }
    }
    let jump = ctl.jump.just && p.lift_phase == 0 && chest.0.is_none() && !locked && !cut.input_locked;
    if p.lift_phase != 0 || chest.0.is_some() {
        dx = 0.0;
        dy = 0.0;
    }
    let world = World { terrain: &room.room, solids: &solids.0, tuning: tuning.0 };
    // ladders (floor 0x0B: against the NW wall, 0x0C: against the NE wall): pressing towards the wall climbs while the way
    // ahead is too high to step onto; Nigel hangs there when no key is pressed
    p.climbing = false;
    if let Some(cell) = room.room.cell_at_world(p.x.floor() as i32, p.y.floor() as i32) {
        if (cell.floor_type == 0x0B || cell.floor_type == 0x0C) && p.vz <= 0.0 && p.lift_phase == 0 && p.carrying.is_none() {
            const LADDER_SPEED: f32 = 3.0;
            let (ldx, ldy) = if cell.floor_type == 0x0B { (-1.0, 0.0) } else { (0.0, -1.0) };
            let floor_here = cell.height as f32;
            let hanging = p.z > floor_here + 0.05;
            let mut handled = false;
            if dx == ldx && dy == ldy && !world.can_stand(p.x + ldx * 0.25, p.y + ldy * 0.25, p.z) {
                p.z += LADDER_SPEED * dt;
                p.facing = if cell.floor_type == 0x0B { 0 } else { 1 };
                p.climbing = true;
                handled = true;
            } else if hanging && dx == -ldx && dy == -ldy {
                p.z = (p.z - LADDER_SPEED * dt).max(floor_here);
                p.climbing = p.z > floor_here + 0.001;
                handled = p.climbing || hanging;
            } else if hanging && dx == 0.0 && dy == 0.0 {
                p.climbing = true;
                handled = true;
            }
            if handled {
                p.vz = 0.0;
                p.grounded = true;
                p.moving = p.climbing && (dx != 0.0 || dy != 0.0);
                return;
            }
        }
    }
    let mut b = Body { x: p.x, y: p.y, z: p.z, vz: p.vz, grounded: p.grounded, air_peak: p.air_peak };
    let before = (b.x, b.y);
    // thrown back by a blow: one axis at a time so walls stop it without sticking
    let (mx, my) = if combat.knock.2 > 0.0 { (combat.knock.0 * dt, combat.knock.1 * dt) } else { (dx * tuning.0.walk_speed * dt, dy * tuning.0.walk_speed * dt) };
    let ev = world.step(&mut b, mx, my, jump, dt);
    if ev.jumped {
        sfx.play(0x53);
    }
    if ev.landed {
        sfx.play(0x5D);
    }
    p.moving = (dx != 0.0 || dy != 0.0) && ((b.x - before.0).abs() > 1e-5 || (b.y - before.1).abs() > 1e-5);
    // climbing a ladder or a rope: a sound at each rung (SND_LadderClimb)
    *ladder_t = if p.climbing && p.moving { *ladder_t - dt } else { 0.0 };
    if p.climbing && p.moving && *ladder_t <= 0.0 {
        sfx.play(0x5E);
        *ladder_t = 0.28;
    }
    // a wall in the way at one corner of the box only: the original pushes him towards the free corner and turns him that way
    if keyed && !p.moving && b.grounded && b.vz <= 0.0 && combat.knock.2 <= 0.0 && !locked && !cut.input_locked && (dx != 0.0) != (dy != 0.0) {
        if std::env::var("LS_LOG_NUDGE").is_ok() {
            println!("[nudge] blocked at ({:.2},{:.2}) z {:.2} going ({},{}): {:?}", b.x, b.y, b.z, dx, dy, physics::nudge_direction(&room.room, &tuning.0, b.x, b.y, b.z, dx, dy));
        }
        // blocked at both corners (the original does nothing): the help for an exit, which goes further than the original's: a door within two cells to the side
        // of what he walks into draws him along the wall to it (landstalker.ini, [game] assist_exits)
        let towards_exit = || -> Option<(f32, f32)> {
            if !config.assist_exits {
                return None;
            }
            let mut best: Option<(f32, (f32, f32))> = None;
            for w in &room.room.file.warps {
                let [wx, wy, ww, wh] = w.area;
                let (cx, cy) = (wx as f32 + ww as f32 / 2.0, wy as f32 + wh as f32 / 2.0);
                // on the side he is heading to, within 5 cells, and to one side of him by at most two cells (more is another way)
                let (along, side) = if dx != 0.0 { ((cx - b.x) * dx.signum(), cy - b.y) } else { ((cy - b.y) * dy.signum(), cx - b.x) };
                if !(0.0..=5.0).contains(&along) || side.abs() > 2.0 || side.abs() < 0.08 {
                    continue;
                }
                if best.map_or(true, |(d, _)| side.abs() < d) {
                    best = Some((side.abs(), if dx != 0.0 { (0.0, side.signum()) } else { (side.signum(), 0.0) }));
                }
            }
            best.map(|(_, d)| d)
        };
        if let Some((nx, ny)) = physics::nudge_direction(&room.room, &tuning.0, b.x, b.y, b.z, dx, dy).or_else(towards_exit) {
            let step = 60.0 / 16.0 * dt; // 1/16 of a cell per frame of the original
            if world.can_stand(b.x + nx * step, b.y + ny * step, b.z) {
                b.x += nx * step;
                b.y += ny * step;
                p.facing = if nx < 0.0 { 0 } else if ny < 0.0 { 1 } else if nx > 0.0 { 2 } else { 3 };
                p.moving = true;
            }
        }
    }
    p.x = b.x;
    p.y = b.y;
    p.z = b.z;
    p.vz = b.vz;
    p.grounded = b.grounded;
    p.air_peak = b.air_peak;
}

/// The chest's picture number `frame` (the last one when it is out of range: open).
fn chest_picture(sprites: &Sprites, frame: usize, orientation: i32) -> Option<Picture> {
    let (list, mirror) = sprites.chest.poses.get("open")?.pick(Dir::from_orientation(orientation), &sprites.chest.directions)?;
    Some(picture_of("sprites/sprite_036", list.get(frame.min(list.len().saturating_sub(1)))?, mirror))
}

/// Nigel's sprite: his picture, with the pixels hidden by a block in front of him cut out (see `CoverMap`).
fn animate_player(
    time: Res<Time>,
    room: Res<RoomRes>,
    sprites: Res<Sprites>,
    solids: Res<Solids>,
    _chest: Res<ChestOpen>,
    combat: Res<Combat>,
    cut: Res<cutscene::Cutscene>,
    game: Res<GameState>,
    cover: Option<Res<CoverMap>>,
    mut images: ResMut<Assets<Image>>,
    mut cache: Local<Option<(String, bool, i32, i32, i32, i32, i32)>>,
    mut q: Query<(&mut Player, &mut Sprite, &mut Transform, &mut Visibility), With<PlayerSprite>>,
) {
    let Ok((mut p, mut sprite, mut tf, mut vis)) = q.single_mut() else { return };
    // protected after a blow: Nigel blinks
    let blink = combat.invulnerable > 0.0 && !combat.dead && (combat.invulnerable * 10.0) as i32 % 2 == 0;
    let want = if blink || cut.hidden { Visibility::Hidden } else { Visibility::Inherited };
    if *vis != want {
        *vis = want;
    }
    p.anim_time = if p.moving { p.anim_time + time.delta_secs() } else { 0.0 };
    // the ailments tint the sprite: blue paralysis, green poison, violet curse, yellow confusion
    sprite.color = if game.has_status(game::STATUS_PARALYSIS) {
        Color::srgb_u8(150, 160, 255)
    } else if game.has_status(game::STATUS_POISON) {
        Color::srgb_u8(150, 255, 150)
    } else if game.has_status(game::STATUS_CURSE) {
        Color::srgb_u8(210, 140, 255)
    } else if game.has_status(game::STATUS_CONFUSION) {
        Color::srgb_u8(255, 255, 130)
    } else {
        Color::WHITE
    };
    let carrying = p.carrying.is_some();
    // on a ladder or a rope: the pictures of the climb (a cycle over the height he gains)
    let climbing_now = p.climbing || (cut.tracking() && cut.track_action & 0x1000 != 0);
    let tracked_swing = cut.tracking() && cut.track_action & 0x0F00 != 0;
    let tracked_hurt = cut.tracking() && cut.track_action & 0x2000 != 0;
    let pose = if combat.dead || cut.faint.is_some() { "death" } else if tracked_swing { "sword" } else if tracked_hurt { "hurt" } else if climbing_now { "climb" } else if p.lift_phase != 0 { "lift" } else if carrying && !p.grounded { "carry_jump" } else if carrying { "carry_walk" } else if combat.swing.is_some() { "sword" } else if combat.hurt > 0.0 { "hurt" } else if !p.grounded { "jump" } else if p.moving { "walk" } else { "idle" };
    // holding up what the chest held (the same picture whatever the direction)
    // (while a chest opens Nigel stays as he is, facing it: the pictures of the "hold_item" pose are sword pictures, which must not show)
    let holding: Option<&ChestSeq> = None;
    let pose = pose;
    let Some((frames, flip)) = sprites.player.poses.get(pose).and_then(|v| v.pick(Dir::from_facing(p.facing), &sprites.player.directions)) else { return };
    let frame = Some(frames).and_then(|f| {
        if f.is_empty() {
            None
        } else if let Some(c) = holding {
            Some(&f[(((c.t - CHEST_OPEN_TIME) / (CHEST_REVEAL_TIME / f.len() as f32)) as usize).min(f.len() - 1)])
        } else if pose == "death" {
            // dead: he falls and stays down
            let t = if combat.dead { combat.death_t } else { cut.faint.unwrap_or(0.0) };
            Some(&f[((t / (combat::DEATH_ANIM_TIME / f.len() as f32)) as usize).min(f.len() - 1)])
        } else if pose == "sword" {
            // the sword: four pictures over the swing
            let swing_t = if tracked_swing { cut.track_action_t } else { combat.swing.unwrap_or(0.0) };
            Some(&f[((swing_t / (combat::SWING_DURATION / f.len() as f32)) as usize).min(f.len() - 1)])
        } else if climbing_now {
            // up a ladder or a rope: the three pictures of the climb, one per quarter unit gained
            Some(&f[((p.z * 4.0).max(0.0) as usize) % f.len()])
        } else if pose == "hurt" {
            Some(&f[if combat.hurt > combat::HURT_POSE_TIME / 2.0 { 0 } else { 1.min(f.len() - 1) }])
        } else if p.lift_phase != 0 {
            // lifting plays the animation backwards, setting down forwards
            let step = ((p.lift_t / (LIFT_TIME / f.len() as f32)) as usize).min(f.len() - 1);
            Some(&f[if p.lift_phase == 1 { f.len() - 1 - step } else { step }])
        } else if !p.grounded {
            Some(&f[if p.vz > 0.0 { 0 } else { f.len().min(2) - 1 }])
        } else if p.moving {
            let cycle = f.len().min(4);
            Some(&f[((p.anim_time / 0.12) as usize) % cycle])
        } else {
            Some(&f[0])
        }
    });
    let Some(frame) = frame else { return };
    let Some(src) = sprites.player_cpu.get(&frame.file) else { return };
    let pos = snap(room.to_screen(p.x, p.y, p.z));
    let ax = if flip { frame.w - frame.ax } else { frame.ax } as f32;
    let (left, top) = (pos.x - ax, pos.y - frame.ay as f32);
    tf.translation = Vec3::new(left + frame.w as f32 / 2.0, -(top + frame.h as f32 / 2.0), player_depth(&p, &solids.0));
    sprite.flip_x = false;

    let cell_depth = p.x.floor() + p.y.floor();
    let low = rule() == Rule::Original && low_priority(&room, p.x, p.y, p.z);
    // the swoosh of the sword does not go through rock: when what lies in front of him rises above the sword, only the body is drawn
    let in_rock = pose == "sword" && {
        const AHEAD: [(f32, f32); 4] = [(-1.0, 0.0), (0.0, -1.0), (1.0, 0.0), (0.0, 1.0)]; // facing: NW NE SE SW
        let (dx, dy) = AHEAD[(p.facing & 3) as usize];
        [1.0f32, 1.6].iter().any(|d| room.room.cell_at_world((p.x + dx * d).floor() as i32, (p.y + dy * d).floor() as i32).is_some_and(|c| c.blocked() || c.height as f32 > p.z + 1.0))
    };
    let key = (frame.file.clone(), flip, (left * 3.0).round() as i32, (top * 3.0).round() as i32, (p.z * 1000.0).round() as i32, cell_depth as i32 * 4 + low as i32 + in_rock as i32 * 2, room.room.file.id);
    if cache.as_ref() == Some(&key) {
        return;
    }
    *cache = Some(key);
    let (w, h) = src.dimensions();
    let no_cover = std::env::var("LS_NO_OCC").is_ok();
    let mut data = Vec::with_capacity((w * h * 4) as usize);
    for j in 0..h {
        for i in 0..w {
            let sx = if flip { w - 1 - i } else { i };
            let mut px = src.get_pixel(sx, j).0;
            // (his body: 14 pixels each side of his feet, 46 up; what is outside is the swoosh)
            if in_rock && !((i as f32 - ax).abs() <= 14.0 && (j as f32) >= frame.ay as f32 - 46.0 && (j as f32) <= frame.ay as f32 + 3.0) {
                px[3] = 0;
            }
            if px[3] != 0 && !no_cover {
                let (rx, ry) = ((left + i as f32 + 0.5).floor() as i32, (top + j as f32 + 0.5).floor() as i32);
                let hidden = match rule() {
                    Rule::Original => low && cover.as_ref().map_or(false, |c| c.priority_at(rx, ry)),
                    _ => cover.as_ref().map_or(false, |c| c.hides(rx, ry, cell_depth, p.z)),
                };
                if hidden {
                    px[3] = 0;
                }
            }
            data.extend_from_slice(&px);
        }
    }
    let image = Image::new(Extent3d { width: w, height: h, depth_or_array_layers: 1 }, TextureDimension::D2, data, TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::default());
    // a new image each time: a picture of another size written over the old one left the GPU texture at the old size
    sprite.image = images.add(image);
}

/// Where the camera is looking, in the room's coordinates, while it glides between Nigel and a character of a scene.
#[derive(Default)]
struct CameraState {
    room: i32,
    pos: Option<Vec3>,
    gliding: bool,
}

/// C toggles between a camera centred on Nigel and a camera that stays where it is (to see what moves).
#[derive(Resource)]
struct CameraFollow(bool);

/// F10 freezes the whole game (the clock stops: nothing moves, nothing animates) and F10 again goes on: to take a picture of a moment.
fn pause_key(keys: Res<ButtonInput<KeyCode>>, mut clock: ResMut<Time<Virtual>>) {
    if keys.just_pressed(KeyCode::F10) {
        if clock.is_paused() {
            clock.unpause();
        } else {
            clock.pause();
        }
    }
}

fn toggle_camera(ctl: Res<Controls>, mut follow: ResMut<CameraFollow>, mut windows: Query<&mut Window>) {
    if ctl.camera.just {
        follow.0 = !follow.0;
        println!("camera {}", if follow.0 { "follows Nigel" } else { "locked" });
        if let Ok(mut w) = windows.single_mut() {
            w.title = format!("Landstalker next - camera {}", if follow.0 { "follows Nigel (C)" } else { "locked (C)" });
        }
    }
}

/// Test aid: LS_LOG_ENTS=n prints, every n frames (at 60 Hz) of the game's time, where Nigel and each entity of the room are, in the
/// units of the original's RAM (tile + sixteenths) so that the log can be set against tools/emuhawk's.
fn log_entities(time: Res<Time>, room: Res<RoomRes>, script: NonSend<Scripting>, cut: Res<cutscene::Cutscene>, player: Query<&Player>, mut last: Local<i64>) {
    let Some(every) = std::env::var("LS_LOG_ENTS").ok().and_then(|v| v.parse::<i64>().ok()) else { return };
    let frame = (time.elapsed_secs() * 60.0).round() as i64;
    if frame / every == *last / every && *last != 0 {
        return;
    }
    *last = frame.max(1);
    let Ok(p) = player.single() else { return };
    let fmt = |v: f32| format!("{}.{:02x}", v.floor() as i32, ((v - v.floor()) * 16.0) as i32 * 1);
    let mut line = format!("[ents] f={} room={} player={},{} z={:.2} faint={} hidden={} caption={:?} fx={:.2}", frame, room.room.file.id, fmt(p.x), fmt(p.y), p.z, cut.faint.is_some(), cut.hidden, cut.caption.map(|c| c.0), cut.fx_level);
    for e in script.entities().iter().filter(|e| e.kind != 0) {
        line.push_str(&format!(" | e{} t{} {},{} z{:.2}{}", e.index, e.kind, fmt(e.x), fmt(e.y), e.z, if e.visible { "" } else { " hidden" }));
    }
    println!("{}", line);
}

fn follow_camera(follow: Res<CameraFollow>, time: Res<Time>, combat: Res<Combat>, room: Res<RoomRes>, cut: Res<cutscene::Cutscene>, script: NonSend<Scripting>, mut state: Local<CameraState>, windows: Query<&Window>, player: Query<&Player>, mut cam: Query<&mut Transform, (With<Camera2d>, Without<Player>)>) {
    if !follow.0 {
        return;
    }
    let (Ok(p), Ok(mut c)) = (player.single(), cam.single_mut()) else { return };
    // what the camera looks at: Nigel, or the character a scene (a script, a boss waking up) points it at; it glides from one to the other
    // and sticks to Nigel again when it has caught up
    // test aid: LS_CAMERA_FOCUS=n looks at entity n from the start
    if state.pos.is_none() {
        if let Some(n) = std::env::var("LS_CAMERA_FOCUS").ok().and_then(|v| v.parse::<usize>().ok()) {
            script.set_camera_focus(Some(n));
        }
    }
    let target = script.camera_focus().or(cut.focus).and_then(|i| script.entity_pos(i as i32)).map(|(x, y, z)| Vec3::new(x, y, z + 1.0)).unwrap_or(Vec3::new(p.x, p.y, p.z + 1.0));
    let focused = script.camera_focus().or(cut.focus).is_some();
    if state.room != room.room.file.id || state.pos.is_none() {
        state.room = room.room.file.id;
        state.pos = Some(target);
        state.gliding = false;
    }
    let mut pos = state.pos.unwrap_or(target);
    if focused || state.gliding {
        state.gliding = true;
        pos += (target - pos) * (1.0 - (-8.0 * time.delta_secs().min(0.1)).exp());
        if !focused && pos.distance(target) < 0.02 {
            pos = target;
            state.gliding = false;
        }
    } else {
        pos = target;
    }
    state.pos = Some(pos);
    let s = room.to_screen(pos.x, pos.y, pos.z);
    // whole SCREEN pixels (a third of a pixel of the original picture): smooth scrolling, no sprite between two screen pixels
    // the panel covers the right of the window: Nigel is centred in what is left
    // a boss dying shakes the screen
    let (left, total, amp) = combat.shake;
    let (sx, sy) = if left > 0.0 && total > 0.0 {
        let t = time.elapsed_secs() * 60.0;
        let k = amp * SCALE * left / total;
        ((t * 12.9898).sin() * k, (t * 78.233).sin() * k)
    } else {
        (0.0, 0.0)
    };
    c.translation.x = (s.x * SCALE + playtest::PANEL_W / 2.0 + sx).round();
    // like the original, whose screen keeps Nigel 85 pixels from the top (not in the middle): more of the room ahead of him shows
    let drop = windows.single().map_or(0.0, |w| w.height() / 2.0 - NIGEL_SCREEN_Y * SCALE);
    c.translation.y = -(s.y * SCALE + sy).round() - drop.round();
}
