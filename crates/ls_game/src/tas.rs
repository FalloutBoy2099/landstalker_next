//! The TAS: the route of a room played by the program. A room is recorded once (what is pressed, when, from where Nigel starts, with which flags, items and
//! health, and which room he leaves for) and then played again at will: the default route of the room.
//!
//! * `data/tas/room_NNN.yaml`: one room: `start` (where Nigel is, and the state of the adventure: flags, items, health...), `inputs` (the keys held, from each
//!   moment on: `[seconds, "down+right"]`), `end` (when it ended, the room he goes to, the flags then).
//! * `data/tas/route.yaml`: the logical sequence of rooms of a recording of a whole route (`rooms: [148, 149, ...]`), which plays room after room.
//! * Recording: a route, from room to room (the `start_tas` and `stop_tas` keys of landstalker.ini, or the `TAS` window of the debug panel). It starts at the first key
//!   pressed. A room that is reloaded (the load, the panel...) starts its take again: what was tried before in it is forgotten.
//! * Playing: the panel, or landstalker.ini `[debug] tas = route` (plays the route by itself from the first room), `tas_start_room = N` (the game starts in room N with the
//!   flags, items and health the recording of that room started with: to begin at a level of the adventure).
//!
//! The keys are pressed like a person would (those of landstalker.ini): what the game hears is the same.

use crate::bot::{RecMode, Recorder};
use crate::config::Config;
use crate::game::GameState;
use crate::scripting::Scripting;
use crate::{Player, RoomRes};
use bevy::input::keyboard::Key;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Where Nigel is at the start of a room, and the state of the adventure.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TasStart {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub facing: i32,
    pub health: i32,
    pub max_health: i32,
    pub gold: i32,
    /// (item, how many)
    pub items: Vec<(i32, i32)>,
    pub sword: i32,
    pub armour: i32,
    pub boots: i32,
    pub ring: i32,
    pub status: u8,
    pub progress: i32,
    /// the story flags that are set
    pub flags: Vec<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TasEnd {
    /// how long it lasted
    pub time: f32,
    /// the room he went to (None: the recording was stopped in the room)
    pub next_room: Option<i32>,
    /// the story flags that are set then
    pub flags: Vec<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TasRoom {
    pub room: i32,
    pub start: TasStart,
    /// `[seconds, keys]`: the keys held (`up`, `down+right`, `action`...; empty: none) from that moment on
    pub inputs: Vec<(f32, String)>,
    pub end: TasEnd,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TasRoute {
    /// the rooms, in the order they are played
    pub rooms: Vec<i32>,
}

pub fn dir_of(root: &Path) -> PathBuf {
    root.join("data").join("tas")
}

fn room_path(dir: &Path, room: i32) -> PathBuf {
    dir.join(format!("room_{:03}.yaml", room))
}

impl TasRoom {
    pub fn load(dir: &Path, room: i32) -> Option<TasRoom> {
        serde_yaml::from_str(&std::fs::read_to_string(room_path(dir, room)).ok()?).ok()
    }

    pub fn save(&self, dir: &Path) -> std::io::Result<PathBuf> {
        std::fs::create_dir_all(dir)?;
        let path = room_path(dir, self.room);
        let text = format!("# The TAS of room {}: see src/tas.rs. Recorded in the game (debug panel), playable by `[debug] tas` of landstalker.ini.\n{}", self.room, serde_yaml::to_string(self).unwrap_or_default());
        std::fs::write(&path, text)?;
        Ok(path)
    }

    /// The keys held at a moment (seconds since the start).
    pub fn keys_at(&self, t: f32) -> &str {
        self.inputs.iter().rev().find(|(at, _)| *at <= t).map_or("", |(_, k)| k.as_str())
    }

    /// The moment the last of the inputs begins, and how long the recording lasted.
    pub fn length(&self) -> f32 {
        self.end.time.max(self.inputs.last().map_or(0.0, |(t, _)| *t))
    }
}

impl TasRoute {
    pub fn load(dir: &Path) -> Option<TasRoute> {
        serde_yaml::from_str(&std::fs::read_to_string(dir.join("route.yaml")).ok()?).ok()
    }

    pub fn save(&self, dir: &Path) -> std::io::Result<()> {
        std::fs::create_dir_all(dir)?;
        let text = format!("# The logical sequence of the rooms of a recorded route: each one is played from its TAS (room_NNN.yaml).\n{}", serde_yaml::to_string(self).unwrap_or_default());
        std::fs::write(dir.join("route.yaml"), text)
    }
}

/// The state of the adventure and of Nigel, as a start of a TAS.
pub fn snapshot(game: &GameState, script: &Scripting, p: &Player) -> TasStart {
    let mut items: Vec<(i32, i32)> = game.items.iter().filter(|(_, n)| **n > 0).map(|(i, n)| (*i, *n)).collect();
    items.sort();
    let mut flags = script.flags();
    flags.sort();
    TasStart { x: p.x, y: p.y, z: p.z, facing: p.facing as i32, health: game.health, max_health: game.max_health, gold: game.gold, items, sword: game.sword, armour: game.armour, boots: game.boots, ring: game.ring, status: game.status, progress: game.progress, flags }
}

/// Gives the game the state a TAS started with.
pub fn apply_snapshot(s: &TasStart, game: &mut GameState, script: &Scripting) {
    game.health = s.health;
    game.max_health = s.max_health;
    game.gold = s.gold;
    game.items = s.items.iter().copied().collect();
    game.sword = s.sword;
    game.armour = s.armour;
    game.boots = s.boots;
    game.ring = s.ring;
    game.status = s.status;
    game.progress = s.progress;
    script.replace_flags(&s.flags);
}

/// The TAS the game starts from (landstalker.ini `tas_start_room`, or LS_TAS_START): where it begins and with what.
#[derive(Resource, Default)]
pub struct TasStartFrom(pub Option<TasRoom>);

pub fn start_file(root: &Path, config: &Config) -> Option<TasRoom> {
    let room = std::env::var("LS_TAS_START").ok().and_then(|v| v.parse().ok()).unwrap_or(config.tas_start_room);
    if room <= 0 {
        return None;
    }
    let t = TasRoom::load(&dir_of(root), room);
    if t.is_none() {
        println!("[tas] no TAS for room {} in data/tas: the game starts as usual", room);
    }
    t
}

/// Startup: the state of the TAS is given to the game (and Nigel is where it began).
pub fn apply_start(from: Res<TasStartFrom>, mut game: ResMut<GameState>, script: NonSend<Scripting>, mut players: Query<&mut Player>) {
    let Some(t) = &from.0 else { return };
    apply_snapshot(&t.start, &mut game, &script);
    if let Ok(mut p) = players.single_mut() {
        p.z = t.start.z;
        p.facing = t.start.facing as u8;
    }
    println!("[tas] starting in room {} with {} flags, {} items", t.room, t.start.flags.len(), t.start.items.len());
}

// ---------------------------------------------------------------------------------------------------------------
// recording
// ---------------------------------------------------------------------------------------------------------------

/// What is being recorded now.
#[derive(Default)]
pub struct Taking {
    room: i32,
    mode: RecMode,
    /// None until the first key
    started: Option<f32>,
    clock: f32,
    keys: String,
    take: Option<TasRoom>,
    route: Vec<i32>,
    /// seconds after a room change during which nothing starts (Nigel is still being put where he arrives)
    grace: f32,
}

/// The names of the keys held, as the game hears them (the same names as the Recorder's).
fn held(ctl: &crate::config::Controls) -> String {
    let mut names: Vec<&str> = Vec::new();
    for (b, n) in [(&ctl.dir[0], "up"), (&ctl.dir[1], "down"), (&ctl.dir[2], "left"), (&ctl.dir[3], "right"), (&ctl.jump, "jump"), (&ctl.attack, "attack"), (&ctl.action, "action"), (&ctl.menu, "menu"), (&ctl.quick_previous, "quick_previous"), (&ctl.quick_use, "quick_use"), (&ctl.quick_next, "quick_next"), (&ctl.use_key, "use_key"), (&ctl.equipment, "equipment"), (&ctl.ekeeke, "ekeeke")] {
        if b.pressed {
            names.push(n);
        }
    }
    names.join("+")
}

#[allow(clippy::too_many_arguments)]
pub fn tas_record(time: Res<Time>, ctl: Res<crate::config::Controls>, mut rec: ResMut<Recorder>, mut msg: ResMut<crate::game::Message>, play: Res<TasPlayer>, game: Res<GameState>, room: Res<RoomRes>, script: NonSend<Scripting>, players: Query<&Player>, mut taking: Local<Taking>, mut last_at: Local<Option<(i32, f32, f32)>>) {
    let dir = dir_of(Path::new(&std::env::var("BEVY_ASSET_ROOT").unwrap_or_default()));
    let id = room.room.file.id;
    let Ok(p) = players.single() else { return };
    // the keys of landstalker.ini: `start_tas` records a whole route from here, `stop_tas` ends it (the route is written)
    if ctl.start_tas.just && !rec.active() && play.current.is_none() {
        rec.start(RecMode::Continuous, id);
        msg.show(crate::i18n::t("messages.tas.started"));
    }
    if ctl.stop_tas.just && rec.active() {
        rec.stop();
        msg.show(crate::i18n::t("messages.tas.stopped"));
    }
    // a recording starts when the Recorder starts one (the Rec buttons, LS_RECORD, the TAS window), ends when it stops
    if rec.mode == RecMode::Continuous && taking.mode == RecMode::Off && play.current.is_none() {
        *taking = Taking { room: id, mode: rec.mode, ..Default::default() };
    }
    if rec.mode == RecMode::Off && taking.mode != RecMode::Off {
        // stopped (or a room recording that went out of the room: the Recorder stops by itself)
        let next = (id != taking.room).then_some(id);
        finish(&mut taking, &dir, &script, next, true);
        return;
    }
    if taking.mode == RecMode::Off {
        return;
    }
    if id != taking.room {
        // the room changed: the route goes on in the new one
        finish(&mut taking, &dir, &script, Some(id), false);
        taking.room = id;
        taking.grace = 0.3;
        *last_at = None;
    }
    if taking.grace > 0.0 {
        taking.grace -= time.delta_secs();
        *last_at = Some((id, p.x, p.y));
        return;
    }
    // the room is loaded again (the load, the panel, the death): what was recorded in it so far is forgotten and the take starts again at the next key
    // (seen as a jump of Nigel in the same room: more than three cells between two frames is not walking)
    let reloaded = taking.take.is_some() && last_at.is_some_and(|(r, x, y)| r == id && (p.x - x).hypot(p.y - y) > 3.0);
    *last_at = Some((id, p.x, p.y));
    if reloaded {
        println!("[tas] room {} reloaded: the take starts again", id);
        taking.take = None;
        taking.started = None;
        taking.keys.clear();
        msg.show(crate::i18n::t("messages.tas.restarted"));
    }
    let keys = held(&ctl);
    if taking.started.is_none() {
        if keys.is_empty() {
            return;
        }
        taking.started = Some(0.0);
        taking.clock = 0.0;
        taking.keys = "?".into();
        taking.take = Some(TasRoom { room: id, start: snapshot(&game, &script, p), ..Default::default() });
        println!("[tas] recording room {} from ({:.2}, {:.2})", id, p.x, p.y);
    } else {
        taking.clock += time.delta_secs();
    }
    if keys != taking.keys {
        let t = (taking.clock * 100.0).round() / 100.0;
        if let Some(take) = taking.take.as_mut() {
            take.inputs.push((t, keys.clone()));
        }
        taking.keys = keys;
    }
}

/// Closes the room being recorded (and writes it); in a route the room goes into the sequence.
fn finish(taking: &mut Taking, dir: &Path, script: &Scripting, next: Option<i32>, last: bool) {
    if let Some(mut take) = taking.take.take() {
        take.end = TasEnd { time: (taking.clock * 100.0).round() / 100.0, next_room: next, flags: { let mut f = script.flags(); f.sort(); f } };
        match take.save(dir) {
            Ok(path) => println!("[tas] written: {}", path.display()),
            Err(e) => println!("[tas] cannot write: {}", e),
        }
        taking.route.push(take.room);
    }
    taking.started = None;
    taking.keys.clear();
    if last {
        if taking.mode == RecMode::Continuous && taking.route.len() > 1 {
            let route = TasRoute { rooms: std::mem::take(&mut taking.route) };
            match route.save(dir) {
                Ok(()) => println!("[tas] route of {} rooms written: data/tas/route.yaml", route.rooms.len()),
                Err(e) => println!("[tas] cannot write the route: {}", e),
            }
        }
        *taking = Taking::default();
    }
}

// ---------------------------------------------------------------------------------------------------------------
// playing
// ---------------------------------------------------------------------------------------------------------------

/// What is played now.
#[derive(Resource, Default)]
pub struct TasPlayer {
    /// the TAS of the room being played
    pub current: Option<TasRoom>,
    /// seconds since it began
    t: f32,
    /// follow the route from room to room
    pub follow: bool,
    /// the rooms of the route still to play
    queue: Vec<i32>,
    /// the keys held by the player now (to release what is not held any more)
    pressed: Vec<KeyCode>,
    pressed_logical: Vec<Key>,
    /// asked for by the panel / the ini: (room, follow the route)
    pub request: Option<(i32, bool)>,
    started_env: bool,
    pub status: String,
}

impl TasPlayer {
    pub fn playing(&self) -> bool {
        self.current.is_some()
    }

    pub fn stop(&mut self) {
        self.current = None;
        self.follow = false;
        self.queue.clear();
        self.status = "stopped".into();
    }
}

fn key_of(config: &Config, name: &str) -> Option<KeyCode> {
    config.keys.get(name).and_then(|v| v.first()).copied()
}

/// Plays the TAS: the keys are held at the moments they were; before the controls are read.
#[allow(clippy::too_many_arguments)]
pub fn tas_play(
    time: Res<Time>,
    config: Res<Config>,
    room: Res<RoomRes>,
    mut play: ResMut<TasPlayer>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut logical: ResMut<ButtonInput<Key>>,
    mut game: ResMut<GameState>,
    script: NonSend<Scripting>,
    mut players: Query<&mut Player>,
    mut last_room: Local<i32>,
) {
    let root = std::env::var("BEVY_ASSET_ROOT").unwrap_or_default();
    let dir = dir_of(Path::new(&root));
    let id = room.room.file.id;
    // asked for by the ini: `tas = room` / `route` plays from the start of the game
    if !play.started_env {
        play.started_env = true;
        if config.tas_mode == "route" {
            play.request = Some((id, true));
        }
    }
    if let Some((r, follow)) = play.request.take() {
        let route = if follow { TasRoute::load(&dir) } else { None };
        // the route: from the room asked for to its end
        let queue: Vec<i32> = match &route {
            Some(rt) => rt.rooms.iter().copied().skip_while(|x| *x != r).skip(1).collect(),
            None => Vec::new(),
        };
        match TasRoom::load(&dir, r) {
            Some(t) => {
                if let Ok(mut p) = players.single_mut() {
                    // the first room is played from where it was recorded
                    if r == id {
                        p.x = t.start.x;
                        p.y = t.start.y;
                        p.z = t.start.z;
                        p.facing = t.start.facing as u8;
                    }
                }
                if r == id {
                    apply_snapshot(&t.start, &mut game, &script);
                }
                play.status = format!("playing room {} ({} inputs{})", r, t.inputs.len(), if follow { ", then the route" } else { "" });
                println!("[tas] {}", play.status);
                play.current = Some(t);
                play.t = 0.0;
                play.follow = follow;
                play.queue = queue;
            }
            None => {
                play.status = format!("no TAS for room {}", r);
                println!("[tas] {}", play.status);
            }
        }
        *last_room = id;
    }
    // what is held is released first (what the player himself presses is not touched)
    let held_before: Vec<KeyCode> = std::mem::take(&mut play.pressed);
    let logical_before: Vec<Key> = std::mem::take(&mut play.pressed_logical);
    for k in held_before {
        keys.release(k);
    }
    for k in logical_before {
        logical.release(k);
    }
    let Some(tas) = play.current.clone() else {
        *last_room = id;
        return;
    };
    // a room of the route is left: the next one is played (if it has a TAS), otherwise it is over
    if id != *last_room {
        *last_room = id;
        if play.follow {
            let next = play.queue.iter().position(|r| *r == id).map(|i| play.queue[i]).or(play.queue.first().copied().filter(|r| *r == id));
            if let (Some(n), Some(t)) = (next, TasRoom::load(&dir, id)) {
                if let Some(i) = play.queue.iter().position(|r| *r == n) {
                    play.queue.drain(..=i);
                }
                play.status = format!("playing room {}", id);
                println!("[tas] {}", play.status);
                play.current = Some(t);
                play.t = 0.0;
                return;
            }
        }
        play.stop();
        play.status = format!("over: room {} left", tas.room);
        println!("[tas] {}", play.status);
        return;
    }
    // (time goes on in the dialogues and the scenes too: they are in the recording)
    play.t += time.delta_secs();
    if play.t > tas.length() + 0.5 {
        if play.follow && !play.queue.is_empty() {
            // the end of the room's inputs without leaving it: wait for the next room a moment
            if play.t > tas.length() + 5.0 {
                play.stop();
                play.status = "over: the room was not left".into();
            }
        } else {
            play.stop();
            play.status = "over".into();
            println!("[tas] over");
        }
        return;
    }
    let wanted = tas.keys_at(play.t).to_string();
    for name in wanted.split('+').filter(|n| !n.is_empty()) {
        if let Some(k) = key_of(&config, name) {
            keys.press(k);
            play.pressed.push(k);
            if let Some(c) = crate::config::letter_of(k).filter(|_| config.logical_letters) {
                let key = Key::Character(c.to_string().into());
                logical.press(key.clone());
                play.pressed_logical.push(key);
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------
// the debug panel
// ---------------------------------------------------------------------------------------------------------------

/// The TAS in the debug panel: record a route, stop, play the route from this room.
pub fn tas_ui(ui: &mut bevy_egui::egui::Ui, room_id: i32, rec: &mut Recorder, play: &mut TasPlayer) {
    use bevy_egui::egui;
    let id = room_id;
    let dir = dir_of(Path::new(&std::env::var("BEVY_ASSET_ROOT").unwrap_or_default()));
    ui.label(egui::RichText::new("TAS").strong());
    let has = TasRoom::load(&dir, id);
    ui.label(match &has {
        Some(t) => format!("room {}: a TAS of {:.1} s, {} inputs, to room {}", id, t.length(), t.inputs.len(), t.end.next_room.map_or("-".to_string(), |r| r.to_string())),
        None => format!("room {}: no TAS", id),
    });
    if let Some(route) = TasRoute::load(&dir) {
        ui.label(format!("route: {} rooms ({} .. {})", route.rooms.len(), route.rooms.first().copied().unwrap_or(0), route.rooms.last().copied().unwrap_or(0)));
    }
    ui.separator();
    ui.horizontal_wrapped(|ui| {
        if ui.add_enabled(!play.playing(), egui::Button::new("Record route")).clicked() {
            rec.start(RecMode::Continuous, id);
        }
        if ui.add_enabled(rec.active(), egui::Button::new("Stop recording")).clicked() {
            rec.stop();
        }
    });
    if rec.active() {
        ui.label(if rec.writing() { "recording..." } else { "waiting for the first key" });
    }
    ui.horizontal_wrapped(|ui| {
        if ui.add_enabled(has.is_some() && !rec.active() && !play.playing(), egui::Button::new("Play route from here")).clicked() {
            play.request = Some((id, true));
        }
        if ui.add_enabled(play.playing(), egui::Button::new("Stop playing")).clicked() {
            play.stop();
        }
    });
    if !play.status.is_empty() {
        ui.label(play.status.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_room_goes_to_a_file_and_back() {
        let dir = std::env::temp_dir().join(format!("ls_tas_test_{}", std::process::id()));
        let t = TasRoom {
            room: 28,
            start: TasStart { x: 18.5, y: 20.5, z: 2.0, facing: 3, health: 1024, max_health: 1024, gold: 30, items: vec![(0, 3), (50, 1)], sword: 1, armour: -1, boots: -1, ring: -1, status: 0, progress: 2, flags: vec![0, 18] },
            inputs: vec![(0.0, "down".into()), (0.5, "down+right".into()), (1.25, "".into()), (2.0, "action".into())],
            end: TasEnd { time: 2.4, next_room: Some(30), flags: vec![0, 18, 21] },
        };
        let path = t.save(&dir).unwrap();
        assert!(path.ends_with("room_028.yaml"));
        assert_eq!(TasRoom::load(&dir, 28), Some(t.clone()));
        assert_eq!(t.keys_at(0.2), "down");
        assert_eq!(t.keys_at(0.5), "down+right");
        assert_eq!(t.keys_at(1.5), "");
        assert_eq!(t.keys_at(9.0), "action");
        let route = TasRoute { rooms: vec![28, 30, 31] };
        route.save(&dir).unwrap();
        assert_eq!(TasRoute::load(&dir), Some(route));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
