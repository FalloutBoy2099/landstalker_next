//! Lua scripting: the entities of a room are Lua tables (`ents`), their behaviours coroutines of data/scripts/global.lua
//! (the original's bytecode translated by tools/behaviours_to_lua.py, or hand-made scripts). This module is the bridge:
//! it fills `ents` when a room is loaded, gives the scripts the functions they call (collisions, flags, the player...)
//! and reads the entities back for the renderer and the physics.

use anyhow::{anyhow, Result};
use ls_data::Room;
use mlua::{Function, Lua, Table, Value};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;

/// What the scripts need to know about each kind of entity.
#[derive(Clone, Copy, Default)]
pub struct KindInfo {
    /// half side of the hitbox and its height, in cells / height units
    pub hbase: f32,
    pub hheight: f32,
    pub known: bool,
    /// an enemy that can be killed (health != 255)
    pub killable: bool,
    /// has the stats of a fighter (enemies, but also the spiked balls and boulders)
    pub enemy: bool,
    /// the attack value of its stats
    pub attack: i32,
}

/// One entity as the renderer and the physics see it.
#[derive(Clone, Debug)]
pub struct EntSnapshot {
    pub index: usize,
    pub kind: i32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub orientation: i32,
    pub visible: bool,
    pub solid: bool,
    pub hbase: f32,
    pub hheight: f32,
    pub hostile: bool,
    pub talks: bool,
    pub dying: bool,
    pub pickable: bool,
    pub carried: bool,
    pub killable: bool,
    pub enemy: bool,
    pub dormant: bool,
    /// seconds left of the blink of an enemy that was hit; 0 when it is not
    pub hit_flash: f32,
    /// what the enemy shows: 0 its walk, 1 hurt, 2 attacking (and how far through it is)
    pub pose: i32,
    pub pose_p: f32,
    /// a drop lying there: gold, or an item (-1 none)
    pub pickup_gold: i32,
    pub pickup_item: i32,
    /// moved during the last script tick
    pub moved: bool,
    /// seconds since it started dying
    pub die_t: f32,
    /// a projectile in flight (a boss's fireball, axe...): its attack (0 when the row is not one) and velocity
    pub proj_attack: i32,
    pub proj_vx: f32,
    pub proj_vy: f32,
}

/// Spare rows at the end of a room's entities for the projectiles of the bosses.
pub const PROJECTILE_SLOTS: usize = 16;

struct Shared {
    room: Option<Room>,
    flags: HashSet<i32>,
    kinds: HashMap<i32, KindInfo>,
    triggers: Triggers,
    log: bool,
    sfx: Vec<u8>,
    /// a flag changed: the swaps (Rust) and the rules (Lua) must be applied again
    swaps_dirty: bool,
    rules_dirty: bool,
    /// cutscene actions the scripts asked for: (entity, action), and recorded inputs to replay
    cutscenes: Vec<(i32, i32)>,
    playbacks: Vec<i32>,
    /// the intro of a new game: the scenes' characters walk through anything
    intro: bool,
    /// a script's camera: the entity the screen is centred on instead of Nigel (a scene looking at a character)
    camera: Option<usize>,
    /// a script held the player for four seconds (the original's Pause4s: the raft that carries him)
    pause: bool,
}

pub struct Scripting {
    pub lua: Lua,
    shared: Rc<RefCell<Shared>>,
}

fn get_f(t: &Table, k: &str) -> f32 {
    t.get::<f64>(k).map(|v| v as f32).unwrap_or(0.0)
}

fn get_i(t: &Table, k: &str) -> i32 {
    t.get::<i32>(k).unwrap_or(0)
}

fn get_b(t: &Table, k: &str) -> bool {
    t.get::<bool>(k).unwrap_or(false)
}

impl Scripting {
    pub fn new(data_dir: &Path) -> Result<Scripting> {
        Self::new_inner(data_dir).map_err(|e| anyhow!("{}", e))
    }

    fn new_inner(data_dir: &Path) -> mlua::Result<Scripting> {
        let lua = Lua::new();
        let triggers = Triggers::load(&data_dir.join("text/triggers.txt"));
        let preset: HashSet<i32> = std::env::var("LS_FLAGS").unwrap_or_default().split(',').filter_map(|t| t.trim().parse().ok()).collect(); // test aid
        let shared = Rc::new(RefCell::new(Shared { room: None, flags: preset, kinds: HashMap::new(), triggers, log: std::env::var("LS_LOG_SCRIPT").is_ok(), sfx: Vec::new(), swaps_dirty: false, rules_dirty: false, cutscenes: Vec::new(), playbacks: Vec::new(), intro: false, camera: None, pause: false }));
        let g = lua.globals();
        let scripts_dir: PathBuf = data_dir.join("scripts");

        {
            let dir = scripts_dir.clone();
            g.set(
                "dofile_relative",
                lua.create_function(move |lua, rel: String| {
                    let text = std::fs::read_to_string(dir.join(&rel)).map_err(mlua::Error::external)?;
                    lua.load(text).set_name(rel).exec()
                })?,
            )?;
        }
        g.set("log", lua.create_function(|_, s: String| { println!("[lua] {}", s); Ok(()) })?)?;
        {
            let sh = shared.clone();
            g.set("sfx", lua.create_function(move |_, n: i32| {
                let mut s = sh.borrow_mut();
                if s.log {
                    println!("[lua] sfx {:#x}", n);
                }
                if (0x41..=0x7A).contains(&n) {
                    s.sfx.push(n as u8);
                }
                Ok(())
            })?)?;
        }
        {
            let sh = shared.clone();
            g.set("floor_height", lua.create_function(move |_, (cx, cy): (i32, i32)| Ok(floor_height(&sh.borrow(), cx, cy)))?)?;
        }
        {
            let sh = shared.clone();
            g.set("flag", lua.create_function(move |_, n: i32| Ok(sh.borrow().flags.contains(&n)))?)?;
        }
        {
            let sh = shared.clone();
            g.set("set_flag", lua.create_function(move |_, n: i32| {
                let mut s = sh.borrow_mut();
                if s.log { println!("[lua] flag {} set", n); }
                s.flags.insert(n);
                s.swaps_dirty = true;
                s.rules_dirty = true;
                Ok(())
            })?)?;
        }
        {
            let sh = shared.clone();
            g.set("clear_flag", lua.create_function(move |_, n: i32| {
                let mut s = sh.borrow_mut();
                if s.log { println!("[lua] flag {} cleared", n); }
                s.flags.remove(&n);
                s.swaps_dirty = true;
                s.rules_dirty = true;
                Ok(())
            })?)?;
        }
        {
            let sh = shared.clone();
            g.set("take_rules_dirty", lua.create_function(move |_, ()| Ok(std::mem::take(&mut sh.borrow_mut().rules_dirty)))?)?;
        }
        {
            let sh = shared.clone();
            g.set("overlaps_player", lua.create_function(move |lua, e: Table| {
                let p: Table = lua.globals().get("P")?;
                let (half, top) = (get_f(&e, "hbase"), get_f(&e, "z") + get_f(&e, "hheight"));
                let _ = &sh;
                Ok((get_f(&p, "x") - get_f(&e, "x")).abs() < half + 0.4 && (get_f(&p, "y") - get_f(&e, "y")).abs() < half + 0.4 && get_f(&p, "z") < top && get_f(&p, "z") + 2.0 > get_f(&e, "z"))
            })?)?;
        }
        {
            let sh = shared.clone();
            g.set("trigger_program", lua.create_function(move |lua, n: i32| {
                let p: Table = lua.globals().get("P")?;
                let s = sh.borrow();
                if !s.triggers.has(n) {
                    return Ok(true);
                }
                let room = s.room.as_ref();
                let (px, py, pz) = (get_f(&p, "x"), get_f(&p, "y"), get_f(&p, "z"));
                let (cx, cy) = (px.floor() as i32, py.floor() as i32);
                let (subx, suby) = ((((px - cx as f32) * 16.0) as i32).clamp(0, 15), (((py - cy as f32) * 16.0) as i32).clamp(0, 15));
                let facing = p.get::<i32>("facing").unwrap_or(0);
                let grounded = get_b(&p, "grounded");
                let flags = &s.flags;
                let fh = room.and_then(|r| r.cell_at_world(cx, cy)).map_or(0, |c| c.height as i32);
                Ok(s.triggers.evaluate(n, &|var, size| match var {
                    Var::Px => if size == 'w' { ((cx & 0xFF) << 8) | (cy & 0xFF) } else { cx },
                    Var::Py => cy,
                    Var::Pz => { let z = (pz * 16.0).round() as i32; if size == 'w' { z } else { z & 0xFF } }
                    Var::Sx => if size == 'w' { (subx << 8) | suby } else { subx },
                    Var::Sy => suby,
                    Var::Fh => (fh * 16) & 0xFF,
                    Var::Rot => { let to_rot = [3, 0, 1, 2]; to_rot[(facing & 3) as usize] << 6 }
                    Var::Fall => if grounded { 0 } else { 1 },
                    Var::Playback => get_i(&p, "playback"),
                    Var::Flag(byte) => (0..8).fold(0, |acc, bit| acc | if flags.contains(&(byte * 8 + bit)) { 1 << bit } else { 0 }),
                }))
            })?)?;
        }
        {
            let sh = shared.clone();
            g.set("request_cutscene", lua.create_function(move |_, (source, id): (i32, i32)| {
                sh.borrow_mut().cutscenes.push((source, id));
                Ok(())
            })?)?;
            let sh = shared.clone();
            g.set("request_pause", lua.create_function(move |_, ()| {
                sh.borrow_mut().pause = true;
                Ok(())
            })?)?;
            let sh = shared.clone();
            // camera_focus(i): the camera looks at entity i (0-based); camera_focus(-1) gives it back to Nigel
            g.set("camera_focus", lua.create_function(move |_, i: i32| {
                sh.borrow_mut().camera = if i >= 0 { Some(i as usize) } else { None };
                Ok(())
            })?)?;
            let sh = shared.clone();
            g.set("request_playback", lua.create_function(move |_, n: i32| {
                sh.borrow_mut().playbacks.push(n);
                Ok(())
            })?)?;
        }
        {
            let sh = shared.clone();
            g.set("step", lua.create_function(move |lua, (e, noclip): (Table, Option<bool>)| Ok({ let s = sh.borrow(); step_entity(lua, &s, &e, noclip.unwrap_or(false) || s.intro) }))?)?;
        }
        {
            let sh = shared.clone();
            g.set("hits", lua.create_function(move |lua, e: Table| {
                let (x, y, z) = (get_f(&e, "x"), get_f(&e, "y"), get_f(&e, "z"));
                Ok(hits_others(lua, &sh.borrow(), &e, x, y, z, false))
            })?)?;
        }
        {
            // RoomView::EnemyMove: a walking enemy goes by (dx, dy) if its box fits there (a floating one only needs the map)
            let sh = shared.clone();
            g.set("enemy_move", lua.create_function(move |lua, (e, dx, dy, ghost): (Table, f32, f32, bool)| {
                let s = sh.borrow();
                let (x, y, z) = (get_f(&e, "x"), get_f(&e, "y"), get_f(&e, "z"));
                let (nx, ny) = (x + dx, y + dy);
                let Some(room) = s.room.as_ref() else { return Ok(false) };
                let ok = if ghost { room.cell_at_world(nx.floor() as i32, ny.floor() as i32).is_some() } else { can_stand(lua, &s, &e, nx, ny, z, false, true, None, 1.0) };
                if ok {
                    e.set("x", nx)?;
                    e.set("y", ny)?;
                }
                Ok(ok)
            })?)?;
        }
        {
            // RoomView::EnemyFloor: the floor under an enemy, or the top of a low object (a pressure plate) it can step onto
            let sh = shared.clone();
            g.set("enemy_floor", lua.create_function(move |lua, e: Table| Ok(enemy_floor(lua, &sh.borrow(), &e)))?)?;
        }
        {
            // the box of a kind of entity: half side, height
            let sh = shared.clone();
            g.set("kind_box", lua.create_function(move |_, kind: i32| {
                let s = sh.borrow();
                let k = s.kinds.get(&kind).copied().unwrap_or_default();
                Ok((k.hbase, k.hheight))
            })?)?;
        }
        {
            // what the enemies leave behind: DROPS[kind] = { item, one_in }
            let drops = lua.create_table()?;
            for line in std::fs::read_to_string(data_dir.join("text/enemy_drops.txt")).unwrap_or_default().lines() {
                let n: Vec<i32> = line.split_whitespace().filter_map(|t| t.parse().ok()).collect();
                if !line.starts_with('#') && n.len() == 3 {
                    let row = lua.create_table()?;
                    row.set("item", n[1])?;
                    row.set("one_in", n[2])?;
                    drops.set(n[0], row)?;
                }
            }
            g.set("DROPS", drops)?;
        }
        {
            // is there a wall (or nothing) at this point of the room? (what stops a projectile)
            let sh = shared.clone();
            g.set("wall_at", lua.create_function(move |_, (x, y): (f32, f32)| {
                let s = sh.borrow();
                Ok(s.room.as_ref().and_then(|r| r.cell_at_world(x.floor() as i32, y.floor() as i32)).map_or(true, |c| c.blocked() || c.closed_to_npcs()))
            })?)?;
        }
        let player = lua.create_table()?;
        g.set("P", player)?;
        g.set("ents", lua.create_table()?)?;

        let text = std::fs::read_to_string(scripts_dir.join("global.lua")).map_err(mlua::Error::external)?;
        lua.load(text).set_name("global.lua").exec()?;
        drop(g);
        Ok(Scripting { lua, shared })
    }

    /// The sound effects the scripts asked for since the last call.
    pub fn take_sfx(&self) -> Vec<u8> {
        std::mem::take(&mut self.shared.borrow_mut().sfx)
    }

    pub fn set_kinds(&mut self, kinds: HashMap<i32, KindInfo>) {
        self.shared.borrow_mut().kinds = kinds;
    }

    /// New room: `ents` is rebuilt from the room file and every script restarts.
    pub fn load_room(&mut self, room: &Room) -> Result<()> {
        self.load_room_inner(room).map_err(|e| anyhow!("{}", e))
    }

    fn load_room_inner(&mut self, room: &Room) -> mlua::Result<()> {
        let kinds = self.shared.borrow().kinds.clone();
        self.shared.borrow_mut().camera = None;
        let t = self.lua.create_table()?;
        for (i, e) in room.file.entities.iter().enumerate() {
            let k = kinds.get(&e.kind).copied().unwrap_or_default();
            let row = self.lua.create_table()?;
            row.set("i", i as i32)?;
            row.set("kind", e.kind)?;
            row.set("x", e.position[0])?;
            row.set("y", e.position[1])?;
            row.set("z", e.position[2])?;
            row.set("orientation", match e.orientation.as_str() { "NE" => 0, "SE" => 1, "SW" => 2, _ => 3 })?;
            // the room file gives an index into the original's SpeedLUT (0, 1, 2, 4, 8, 0 sixteenths of a cell per tick)
            row.set("speed", [0, 1, 2, 4, 8, 0][(e.speed.clamp(0, 5)) as usize])?;
            row.set("visible", e.flags.visible)?;
            row.set("base_visible", e.flags.visible)?;
            row.set("hostile", e.flags.hostile)?;
            row.set("gravity", e.flags.gravity)?;
            row.set("solid", e.flags.solid)?;
            row.set("talks", e.flags.talks)?;
            row.set("falls", false)?;
            row.set("dying", false)?;
            row.set("behaviour", e.behaviour)?;
            row.set("hbase", k.hbase)?;
            row.set("hheight", k.hheight)?;
            row.set("killable", k.killable)?;
            row.set("enemy", k.enemy)?;
            row.set("attack", k.attack)?;
            row.set("proj_attack", 0)?;
            row.set("speed_index", e.speed)?;
            row.set("dormant", false)?;
            row.set("dead", false)?;
            row.set("hit_flash", 0.0)?;
            row.set("pose", 0)?;
            row.set("pickup_gold", 0)?;
            row.set("pickup_item", -1)?;
            row.set("moved", false)?;
            row.set("pickable", e.flags.pickable)?;
            row.set("carried", false)?;
            t.push(row)?;
        }
        // spare rows for the projectiles of the bosses: hidden until one is thrown
        for k in 0..PROJECTILE_SLOTS {
            let row = self.lua.create_table()?;
            let i = room.file.entities.len() + k;
            row.set("i", i as i32)?;
            row.set("slot", true)?;
            row.set("kind", 0)?;
            row.set("x", 0.0)?;
            row.set("y", 0.0)?;
            row.set("z", 0.0)?;
            row.set("orientation", 0)?;
            row.set("speed", 0)?;
            row.set("visible", false)?;
            row.set("base_visible", false)?;
            for key in ["hostile", "gravity", "solid", "talks", "falls", "dying", "pickable", "carried", "killable", "enemy", "dormant", "dead", "moved"] {
                row.set(key, false)?;
            }
            row.set("behaviour", 0)?;
            row.set("hbase", 0.0)?;
            row.set("hheight", 0.0)?;
            row.set("speed_index", 0)?;
            row.set("hit_flash", 0.0)?;
            row.set("pose", 0)?;
            row.set("pickup_gold", 0)?;
            row.set("pickup_item", -1)?;
            row.set("proj_attack", 0)?;
            t.push(row)?;
        }
        self.lua.globals().set("ents", t)?;
        // the room's flag rules (who is hidden by which story flag) and its tile swap triggers
        let rules = self.lua.create_table()?;
        for r in &room.file.entity_flags {
            let row = self.lua.create_table()?;
            let get_i = |k: &str| r.get(k).and_then(|v| v.as_i64()).unwrap_or(0);
            let get_b = |k: &str| r.get(k).and_then(|v| v.as_bool()).unwrap_or(false);
            row.set("kind", r.get("kind").and_then(|v| v.as_str()).unwrap_or(""))?;
            row.set("entity", get_i("entity"))?;
            row.set("flag", get_i("flag"))?;
            row.set("when_set", get_b("when_set"))?;
            row.set("show_flag", get_i("show_flag"))?;
            row.set("show_set", get_b("show_set"))?;
            row.set("hide_flag", get_i("hide_flag"))?;
            row.set("hide_set", get_b("hide_set"))?;
            rules.push(row)?;
        }
        self.lua.globals().set("flag_rules", rules)?;
        let sflags = self.lua.create_table()?;
        for r in &room.file.swap_flags {
            let row = self.lua.create_table()?;
            row.set("trigger", r.get("trigger").and_then(|v| v.as_i64()).unwrap_or(0))?;
            row.set("flag", r.get("flag").and_then(|v| v.as_i64()).unwrap_or(0))?;
            row.set("locked", r.get("locked").and_then(|v| v.as_bool()).unwrap_or(false))?;
            sflags.push(row)?;
        }
        self.lua.globals().set("swap_flags", sflags)?;
        {
            let mut s = self.shared.borrow_mut();
            s.swaps_dirty = true;
            s.rules_dirty = true;
        }
        self.shared.borrow_mut().room = Some(room.clone());
        let start: Function = self.lua.globals().get("start_room")?;
        start.call::<()>(())?;
        Ok(())
    }

    pub fn set_player(&self, x: f32, y: f32, z: f32, facing: i32, grounded: bool, support: i32) {
        if let Ok(p) = self.lua.globals().get::<Table>("P") {
            let _ = p.set("x", x);
            let _ = p.set("y", y);
            let _ = p.set("z", z);
            let _ = p.set("facing", facing);
            let _ = p.set("grounded", grounded);
            let _ = p.set("support", support);
        }
    }

    /// The direction a script turned the player to (RotatePlayer), once.
    pub fn take_player_turn(&self) -> Option<i32> {
        let p: Table = self.lua.globals().get("P").ok()?;
        let v: Value = p.get("turn_to").ok()?;
        if let Value::Integer(i) = v {
            let _ = p.set("turn_to", Value::Nil);
            return Some(i as i32);
        }
        None
    }

    /// One script tick.
    pub fn tick(&self) {
        if let Ok(f) = self.lua.globals().get::<Function>("tick") {
            if let Err(e) = f.call::<()>(()) {
                println!("[lua] tick: {}", e);
            }
        }
    }

    /// Did a flag change since the last call? (the tile swaps must be applied again)
    pub fn take_swaps_dirty(&self) -> bool {
        std::mem::take(&mut self.shared.borrow_mut().swaps_dirty)
    }

    /// The cutscene actions the scripts asked for since the last call: (entity, action).
    pub fn take_cutscenes(&self) -> Vec<(i32, i32)> {
        std::mem::take(&mut self.shared.borrow_mut().cutscenes)
    }

    /// The entity a script wants the camera on (None: Nigel).
    pub fn camera_focus(&self) -> Option<usize> {
        self.shared.borrow().camera
    }

    pub fn set_camera_focus(&self, index: Option<usize>) {
        self.shared.borrow_mut().camera = index;
    }

    /// A script asked to hold the player for four seconds (254 frames), once.
    pub fn take_pause(&self) -> bool {
        std::mem::take(&mut self.shared.borrow_mut().pause)
    }

    pub fn take_playbacks(&self) -> Vec<i32> {
        std::mem::take(&mut self.shared.borrow_mut().playbacks)
    }

    /// The intro scenes: the characters walk through anything.
    pub fn set_intro(&self, on: bool) {
        self.shared.borrow_mut().intro = on;
    }

    /// What the scripts' PLAYBACK condition sees (see Cutscene::playback_state).
    pub fn set_playback_state(&self, v: i32) {
        if let Ok(p) = self.lua.globals().get::<Table>("P") {
            let _ = p.set("playback", v);
        }
    }

    pub fn clear_flag(&self, n: i32) {
        let mut s = self.shared.borrow_mut();
        s.flags.remove(&n);
        s.swaps_dirty = true;
        s.rules_dirty = true;
    }

    fn entity_table(&self, index: usize) -> Option<Table> {
        self.lua.globals().get::<Table>("ents").ok()?.get::<Table>(index as i32 + 1).ok()
    }

    /// An entity turns hostile (and wakes up) or friendly.
    pub fn set_hostile(&self, index: usize, on: bool) {
        if let Some(e) = self.entity_table(index) {
            let _ = e.set("hostile", on);
            if on {
                let _ = e.set("dormant", false);
            }
        }
    }

    /// An entity appears or disappears for good; returns whether it was visible.
    pub fn set_visible(&self, index: usize, on: bool) -> bool {
        let Some(e) = self.entity_table(index) else { return false };
        let was = get_b(&e, "visible");
        let _ = e.set("visible", on);
        let _ = e.set("base_visible", on);
        was
    }

    /// A recorded cutscene puts an entity where the original had it (see track.rs); the scripts stand still meanwhile.
    pub fn place_entity(&self, index: usize, x: f32, y: f32, z: f32, orientation: i32, visible: bool, moved: bool) {
        let Some(e) = self.entity_table(index) else { return };
        let _ = e.set("x", x);
        let _ = e.set("y", y);
        let _ = e.set("z", z);
        let _ = e.set("orientation", orientation);
        let _ = e.set("visible", visible);
        let _ = e.set("moved", moved);
    }

    pub fn hide_entity(&self, index: usize) {
        self.set_visible(index, false);
    }

    /// Sets a story flag (a door unlocked with a key): the tile swaps and the entities' rules follow.
    pub fn set_flag(&self, n: i32) {
        let mut s = self.shared.borrow_mut();
        s.flags.insert(n);
        s.swaps_dirty = true;
        s.rules_dirty = true;
    }

    /// The story flags that are set.
    pub fn flags(&self) -> Vec<i32> {
        self.shared.borrow().flags.iter().copied().collect()
    }

    /// Replaces the story flags (a loaded game).
    pub fn replace_flags(&self, flags: &[i32]) {
        let mut s = self.shared.borrow_mut();
        s.flags = flags.iter().copied().collect();
        s.swaps_dirty = true;
        s.rules_dirty = true;
    }

    pub fn flag(&self, n: i32) -> bool {
        self.shared.borrow().flags.contains(&n)
    }

    /// Where an entity is now (to carry the player standing on it).
    pub fn entity_pos(&self, index: i32) -> Option<(f32, f32, f32)> {
        let ents = self.lua.globals().get::<Table>("ents").ok()?;
        let e: Table = ents.get(index + 1).ok()?;
        Some((get_f(&e, "x"), get_f(&e, "y"), get_f(&e, "z")))
    }

    pub fn set_f(&self, index: usize, key: &str, v: f32) {
        if let Ok(ents) = self.lua.globals().get::<Table>("ents") {
            if let Ok(e) = ents.get::<Table>(index as i32 + 1) {
                let _ = e.set(key, v);
            }
        }
    }

    pub fn set_b(&self, index: usize, key: &str, v: bool) {
        if let Ok(ents) = self.lua.globals().get::<Table>("ents") {
            if let Ok(e) = ents.get::<Table>(index as i32 + 1) {
                let _ = e.set(key, v);
            }
        }
    }

    /// Could entity `index` stand at (x, y, z)? Cells under its box and no solid object in the way (nothing is stepped onto).
    pub fn can_place(&self, index: usize, x: f32, y: f32, z: f32) -> bool {
        let Ok(ents) = self.lua.globals().get::<Table>("ents") else { return false };
        let Ok(e) = ents.get::<Table>(index as i32 + 1) else { return false };
        can_stand(&self.lua, &self.shared.borrow(), &e, x, y, z, false, false, None, 0.0)
    }

    /// Height of the floor (cells only) under a box of half side `half` at (x, y).
    pub fn cell_floor(&self, x: f32, y: f32, half: f32) -> f32 {
        let s = self.shared.borrow();
        let Some(room) = s.room.as_ref() else { return 0.0 };
        let mut best = 0.0f32;
        for cx in (x - half + 0.01).floor() as i32..=(x + half - 0.01).floor() as i32 {
            for cy in (y - half + 0.01).floor() as i32..=(y + half - 0.01).floor() as i32 {
                if let Some(c) = room.cell_at_world(cx, cy) {
                    if !c.blocked() {
                        best = best.max(c.height as f32);
                    }
                }
            }
        }
        best
    }

    /// The sword hit enemy `index`: it blinks, chases for a while, and is pushed (fx, fy) away while the way is free.
    pub fn hit_enemy(&self, index: usize, fx: f32, fy: f32, knock: f32) {
        let Ok(ents) = self.lua.globals().get::<Table>("ents") else { return };
        let Ok(e) = ents.get::<Table>(index as i32 + 1) else { return };
        let _ = e.set("hit_flash", 0.25);
        let _ = e.set("ai_alert", 6.0);
        let s = self.shared.borrow();
        let mut moved = 0.0;
        while moved < knock {
            let (nx, ny, z) = (get_f(&e, "x") + fx * 0.1, get_f(&e, "y") + fy * 0.1, get_f(&e, "z"));
            if !can_stand(&self.lua, &s, &e, nx, ny, z, false, true, None, 1.0) {
                break;
            }
            let _ = e.set("x", nx);
            let _ = e.set("y", ny);
            moved += 0.1;
        }
    }

    /// Enemy `index` is dead: it flickers for a moment (`seconds`), then leaves its drop.
    pub fn kill_enemy(&self, index: usize, seconds: f32, gold: i32) {
        let Ok(ents) = self.lua.globals().get::<Table>("ents") else { return };
        let Ok(e) = ents.get::<Table>(index as i32 + 1) else { return };
        let _ = e.set("dying", true);
        let _ = e.set("die_t", 0.0);
        let _ = e.set("die_dur", seconds);
        let _ = e.set("pending_gold", gold);
    }

    /// Enemies killed in an earlier visit: gone for good.
    pub fn remove_entities(&self, indices: &[usize]) {
        let Ok(ents) = self.lua.globals().get::<Table>("ents") else { return };
        for &i in indices {
            if let Ok(e) = ents.get::<Table>(i as i32 + 1) {
                let _ = e.set("visible", false);
                let _ = e.set("base_visible", false);
                let _ = e.set("hostile", false);
                let _ = e.set("dead", true);
                let _ = e.set("co", Value::Nil);
            }
        }
    }

    /// A projectile hit Nigel: it is gone.
    pub fn clear_projectile(&self, index: usize) {
        let Ok(ents) = self.lua.globals().get::<Table>("ents") else { return };
        if let Ok(e) = ents.get::<Table>(index as i32 + 1) {
            let _ = e.set("proj_attack", 0);
            let _ = e.set("visible", false);
        }
    }

    /// A drop was taken (or went away).
    pub fn clear_pickup(&self, index: usize) {
        let Ok(ents) = self.lua.globals().get::<Table>("ents") else { return };
        if let Ok(e) = ents.get::<Table>(index as i32 + 1) {
            let _ = e.set("visible", false);
            let _ = e.set("base_visible", false);
            let _ = e.set("pickup_gold", 0);
            let _ = e.set("pickup_item", -1);
        }
    }

    pub fn entities(&self) -> Vec<EntSnapshot> {
        let Ok(ents) = self.lua.globals().get::<Table>("ents") else { return Vec::new() };
        let mut out = Vec::new();
        for e in ents.sequence_values::<Table>().flatten() {
            out.push(EntSnapshot {
                index: e.get::<i32>("i").unwrap_or(0) as usize,
                kind: e.get::<i32>("kind").unwrap_or(0),
                x: get_f(&e, "x"),
                y: get_f(&e, "y"),
                z: get_f(&e, "z"),
                orientation: e.get::<i32>("orientation").unwrap_or(0) & 3,
                visible: get_b(&e, "visible"),
                solid: get_b(&e, "solid"),
                hbase: get_f(&e, "hbase"),
                hheight: get_f(&e, "hheight"),
                hostile: get_b(&e, "hostile"),
                talks: get_b(&e, "talks"),
                dying: get_b(&e, "dying"),
                pickable: get_b(&e, "pickable"),
                carried: get_b(&e, "carried"),
                killable: get_b(&e, "killable"),
                enemy: get_b(&e, "enemy"),
                dormant: get_b(&e, "dormant"),
                hit_flash: get_f(&e, "hit_flash"),
                pose: e.get::<i32>("pose").unwrap_or(0),
                pose_p: get_f(&e, "pose_p"),
                pickup_gold: e.get::<i32>("pickup_gold").unwrap_or(0),
                pickup_item: e.get::<i32>("pickup_item").unwrap_or(-1),
                moved: get_b(&e, "moved"),
                die_t: get_f(&e, "die_t"),
                proj_attack: e.get::<i32>("proj_attack").unwrap_or(0),
                proj_vx: get_f(&e, "vx"),
                proj_vy: get_f(&e, "vy"),
            });
        }
        out
    }
}

fn enemy_floor(lua: &Lua, s: &Shared, e: &Table) -> f32 {
    let (x, y, z) = (get_f(e, "x"), get_f(e, "y"), get_f(e, "z"));
    let mut f = floor_height(s, x.floor() as i32, y.floor() as i32);
    let me = e.get::<i32>("i").unwrap_or(-1);
    if let Ok(ents) = lua.globals().get::<Table>("ents") {
        for o in ents.sequence_values::<Table>().flatten() {
            let kind = o.get::<i32>("kind").unwrap_or(0);
            let k = s.kinds.get(&kind).copied().unwrap_or_default();
            let platform = !get_b(&o, "hostile") && !get_b(&o, "talks") && !get_b(&o, "dying") && !k.killable && k.known;
            if o.get::<i32>("i").unwrap_or(-2) == me || !get_b(&o, "solid") || !get_b(&o, "visible") || kind >= 0xC0 || !platform {
                continue;
            }
            let (oh, otop) = (get_f(&o, "hbase"), get_f(&o, "z") + get_f(&o, "hheight"));
            if (x - get_f(&o, "x")).abs() < oh && (y - get_f(&o, "y")).abs() < oh && otop > f && otop <= f.max(z) + 0.6 {
                f = otop;
            }
        }
    }
    f
}

fn floor_height(s: &Shared, cx: i32, cy: i32) -> f32 {
    s.room.as_ref().and_then(|r| r.cell_at_world(cx, cy)).map_or(0.0, |c| c.height as f32)
}

/// RoomView::BehaviourStep: one step in the entity's direction; false when something is in the way.
fn step_entity(lua: &Lua, s: &Shared, e: &Table, noclip: bool) -> bool {
    const DX: [f32; 4] = [0.0, 1.0, 0.0, -1.0];
    const DY: [f32; 4] = [-1.0, 0.0, 1.0, 0.0];
    let o = (e.get::<i32>("orientation").unwrap_or(0) & 3) as usize;
    let d = get_f(e, "speed") / 16.0;
    let (x, y, z) = (get_f(e, "x"), get_f(e, "y"), get_f(e, "z"));
    let (nx, ny) = (x + DX[o] * d, y + DY[o] * d);
    let Some(room) = s.room.as_ref() else { return false };
    if room.cell_at_world(nx.floor() as i32, ny.floor() as i32).is_none() {
        return false;
    }
    let kind = e.get::<i32>("kind").unwrap_or(0);
    let known = s.kinds.get(&kind).map_or(false, |k| k.known);
    // rafts (53) glide over water and a rolling boulder (20) follows its corridor: the heightmap allows neither (but
    // they are stopped by the other objects: the blocks that hold the boulder of room 18 back)
    if !noclip && known && !can_stand(lua, s, e, nx, ny, z, kind == 53 || kind == 20, true, Some((DX[o], DY[o])), 0.0) {
        return false;
    }
    let _ = e.set("x", nx);
    let _ = e.set("y", ny);
    if get_b(e, "gravity") {
        let _ = e.set("z", floor_height(s, nx.floor() as i32, ny.floor() as i32));
    }
    true
}

/// RoomView::CanEnemyStandAt: the cells under the entity's box allow it, and nothing solid is in the way.
/// `step`: how high a rise the entity walks onto (0: none, the rule of the original's scripts; 1 for the walking enemies).
fn can_stand(lua: &Lua, s: &Shared, e: &Table, x: f32, y: f32, z: f32, ignore_cells: bool, hop: bool, dir: Option<(f32, f32)>, step: f32) -> bool {
    let half = get_f(e, "hbase") - 0.02;
    if !ignore_cells {
        let Some(room) = s.room.as_ref() else { return false };
        // moving: only the cells along the edge in front of the entity count (sub_31A6 looks at the leading row or column), so
        // a platform that sticks out over a higher block can still move away from it; without a direction: the four corners
        let mut points: Vec<(f32, f32)> = Vec::new();
        match dir {
            Some((dx, dy)) if dx != 0.0 || dy != 0.0 => {
                let mut t = -half;
                while t < half + 0.01 {
                    let t_clamped = t.min(half);
                    points.push(if dx != 0.0 { (x + dx * half, y + t_clamped) } else { (x + t_clamped, y + dy * half) });
                    t += 0.5;
                }
                points.push(if dx != 0.0 { (x + dx * half, y + half) } else { (x + half, y + dy * half) });
            }
            _ => {
                for cx in [x - half, x + half] {
                    for cy in [y - half, y + half] {
                        points.push((cx, cy));
                    }
                }
            }
        }
        for (cx, cy) in points {
            {
                let (ix, iy) = (cx.floor() as i32, cy.floor() as i32);
                match room.cell_at_world(ix, iy) {
                    None => return false,
                    Some(c) => {
                        if c.blocked() || c.closed_to_npcs() {
                            return false;
                        }
                        // the original (sub_31A6): a cell higher than the entity's feet, in whole units, stops it: no step up
                        let too_high = if step > 0.0 { c.height as f32 > z + step + 1e-6 } else { c.height as i32 > (z + 1e-4).floor() as i32 };
                        if too_high {
                            return false;
                        }
                    }
                }
            }
        }
    }
    !hits_others(lua, s, e, x, y, z, hop)
}

/// Is the entity's box at (x, y, z) inside the player or another solid entity? `hop` lets low objects (a pressure
/// plate) be walked onto, as for the walking enemies; platforms that carry things use it without.
fn hits_others(lua: &Lua, s: &Shared, e: &Table, x: f32, y: f32, z: f32, hop: bool) -> bool {
    let half = get_f(e, "hbase") - 0.02;
    let top = z + get_f(e, "hheight");
    if let Ok(p) = lua.globals().get::<Table>("P") {
        let (px, py, pz) = (get_f(&p, "x"), get_f(&p, "y"), get_f(&p, "z"));
        if (x - px).abs() < half + 0.4 && (y - py).abs() < half + 0.4 && pz < top && pz + 2.0 > z {
            return true;
        }
    }
    let me = e.get::<i32>("i").unwrap_or(-1);
    if let Ok(ents) = lua.globals().get::<Table>("ents") {
        for o in ents.sequence_values::<Table>().flatten() {
            if o.get::<i32>("i").unwrap_or(-2) == me {
                continue;
            }
            let kind = o.get::<i32>("kind").unwrap_or(0);
            if !get_b(&o, "solid") || !get_b(&o, "visible") || kind >= 0xC0 || !s.kinds.get(&kind).map_or(false, |k| k.known) {
                continue;
            }
            let (oh, otop, oz) = (get_f(&o, "hbase"), get_f(&o, "z") + get_f(&o, "hheight"), get_f(&o, "z"));
            let platform = !get_b(&o, "hostile") && !get_b(&o, "talks") && !get_b(&o, "dying") && !s.kinds.get(&kind).map_or(false, |k| k.killable);
            if hop && platform && otop <= z + 0.6 {
                continue;
            }
            if (x - get_f(&o, "x")).abs() < half + oh && (y - get_f(&o, "y")).abs() < half + oh && z < otop && top > oz {
                return true;
            }
        }
    }
    false
}

// ---------------------------------------------------------------------------------------------------------------
// trigger programs (data/text/triggers.txt), the small routines of the original that WaitForCondition runs
// ---------------------------------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub enum Var {
    Px,
    Py,
    Pz,
    Sx,
    Sy,
    Fh,
    Rot,
    Fall,
    Playback,
    Flag(i32),
}

#[derive(Clone)]
enum Ins {
    Cmp(char, Var, i32),
    Tst(char, Var),
    Btst(i32, Var),
    Load(Var),
    AndD0(i32),
    CmpD0(i32),
    Branch(String, String),
    Label(String),
}

pub struct Triggers {
    programs: HashMap<i32, Vec<Ins>>,
}

fn parse_var(s: &str) -> Var {
    match s {
        "PX" => Var::Px,
        "PY" => Var::Py,
        "PZ" => Var::Pz,
        "SX" => Var::Sx,
        "SY" => Var::Sy,
        "FH" => Var::Fh,
        "ROT" => Var::Rot,
        "FALL" => Var::Fall,
        "PLAYBACK" => Var::Playback,
        _ => s.strip_prefix("FLAG").and_then(|n| n.parse().ok()).map(Var::Flag).unwrap_or(Var::Px),
    }
}

impl Triggers {
    pub fn load(path: &Path) -> Triggers {
        let mut programs: HashMap<i32, Vec<Ins>> = HashMap::new();
        let text = std::fs::read_to_string(path).unwrap_or_default();
        let mut current: Option<i32> = None;
        for line in text.lines() {
            let line = line.trim_end();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(n) = line.strip_prefix("TA ") {
                current = n.trim().parse().ok();
                if let Some(c) = current {
                    programs.entry(c).or_default();
                }
                continue;
            }
            let Some(c) = current else { continue };
            let mut it = line.split_whitespace();
            let Some(op) = it.next() else { continue };
            let args: Vec<&str> = it.collect();
            let num = |s: &str| s.parse::<i32>().unwrap_or(0);
            let ins = if let Some(label) = op.strip_suffix(':') {
                Ins::Label(label.to_string())
            } else {
                match op {
                    "cmp" if args.len() >= 3 => Ins::Cmp(args[0].chars().next().unwrap_or('b'), parse_var(args[1]), num(args[2])),
                    "tst" if args.len() >= 2 => Ins::Tst(args[0].chars().next().unwrap_or('b'), parse_var(args[1])),
                    "btst" if args.len() >= 2 => Ins::Btst(num(args[0]), parse_var(args[1])),
                    "load" if !args.is_empty() => Ins::Load(parse_var(args[0])),
                    "andd0" if !args.is_empty() => Ins::AndD0(num(args[0])),
                    "cmpd0" if !args.is_empty() => Ins::CmpD0(num(args[0])),
                    _ => Ins::Branch(op.to_string(), args.first().map(|s| s.to_string()).unwrap_or_default()),
                }
            };
            programs.entry(c).or_default().push(ins);
        }
        Triggers { programs }
    }

    pub fn has(&self, n: i32) -> bool {
        self.programs.contains_key(&n)
    }

    /// Does the condition hold? (A routine that falls off its end does not hold.)
    pub fn evaluate(&self, n: i32, get: &dyn Fn(Var, char) -> i32) -> bool {
        let Some(prog) = self.programs.get(&n) else { return true };
        let (mut nf, mut zf, mut cf) = (false, false, false);
        let mut d0 = 0i32;
        let mask = |s: char| if s == 'w' { 0xFFFF } else { 0xFF };
        let sign = |s: char| if s == 'w' { 0x8000 } else { 0x80 };
        let mut pc = 0usize;
        for _ in 0..500 {
            if pc >= prog.len() {
                break;
            }
            match &prog[pc] {
                Ins::Label(_) => {}
                Ins::Cmp(size, var, imm) => {
                    let m = mask(*size);
                    let v = get(*var, *size) & m;
                    let im = imm & m;
                    let res = (v - im) & m;
                    zf = res == 0;
                    cf = v < im;
                    nf = res & sign(*size) != 0;
                }
                Ins::Tst(size, var) => {
                    let v = get(*var, *size) & mask(*size);
                    zf = v == 0;
                    nf = v & sign(*size) != 0;
                    cf = false;
                }
                Ins::Btst(bit, var) => zf = (get(*var, 'b') >> bit) & 1 == 0,
                Ins::Load(var) => {
                    d0 = get(*var, 'b') & 0xFF;
                    zf = d0 == 0;
                    nf = d0 & 0x80 != 0;
                    cf = false;
                }
                Ins::AndD0(imm) => {
                    d0 &= imm & 0xFF;
                    zf = d0 == 0;
                    nf = d0 & 0x80 != 0;
                    cf = false;
                }
                Ins::CmpD0(imm) => {
                    let im = imm & 0xFF;
                    let res = (d0 - im) & 0xFF;
                    zf = res == 0;
                    cf = d0 < im;
                    nf = res & 0x80 != 0;
                }
                Ins::Branch(cond, target) => {
                    let take = match cond.as_str() {
                        "bra" => true,
                        "beq" => zf,
                        "bne" => !zf,
                        "bcc" => !cf,
                        "bcs" => cf,
                        "bhi" => !cf && !zf,
                        "bls" => cf || zf,
                        "bmi" => nf,
                        "bpl" => !nf,
                        _ => false,
                    };
                    if take {
                        if target == "done" {
                            return true;
                        }
                        if target == "notdone" {
                            return false;
                        }
                        match prog.iter().position(|i| matches!(i, Ins::Label(l) if l == target)) {
                            Some(k) => pc = k,
                            None => return false,
                        }
                    }
                }
            }
            pc += 1;
        }
        false
    }
}
